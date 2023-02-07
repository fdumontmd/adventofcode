use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Rule {
    container: String,
    contained: Vec<(usize, String)>,
}

fn parse(input: &str) -> Vec<Rule> {
    let re = Regex::new("(\\d+) (\\w+ \\w+) bags?.").unwrap();
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let pos = l.find(" bags contain").unwrap();
            let container = l[0..pos].to_string();
            let mut contained = Vec::new();
            for cap in re.captures_iter(&l[pos+14..]) {
                let count: usize = cap[1].parse().unwrap();
                let bag = cap[2].to_string();
                contained.push((count, bag));
            }
            Rule {
                container,
                contained,
            }
        })
        .collect()
}

fn part1(rule: &Vec<Rule>) -> usize {
    // build a map of contained bag to container
    // lookup shiny gold
    // check all container to shiny gold to a set
    // diff new set with current one
    // for each diff, loop
    // if empty diff, count set

    let mut contained = HashMap::new();
    for rule in rule {
        for (_, bag) in &rule.contained {
            contained.entry(bag.to_owned()).or_insert(Vec::new())
                .push(&rule.container)
        }
    }

    let mut containers = HashSet::new();
    
    let mut new_containers = HashSet::new();

    for container in contained.get("shiny gold").unwrap() {
        new_containers.insert(container);
    }

    while !new_containers.is_empty() {
        containers = containers.union(&new_containers).cloned().collect();

        let mut tmp = HashSet::new();

        new_containers.iter().for_each(|&bag| {
            if let Some(v) = contained.get(*bag) {
                for container in v {
                    tmp.insert(container);
                }
            }
        });

        new_containers = tmp.difference(&containers).cloned().collect();
    }

    containers.len()
}

fn part2(rules: &Vec<Rule>) -> usize {
    let mut containers = HashMap::new();

    for rule in rules {
        containers.insert(&*rule.container, &rule.contained);
    }

    // memoize
    let mut bag_count = HashMap::new();

    fn rec<'a>(target: &'a str, containers: &HashMap<&'a str, &'a Vec<(usize, String)>>, bag_count: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(u) = bag_count.get(target) {
            *u
        } else {
            let count = if let Some(v) = containers.get(target) {
                v.iter().map(|(count, ref target)| {
                    count * rec(target, containers, bag_count)
                }).sum::<usize>() + 1
            } else {
                1
            };
            bag_count.insert(target, count);
            count
        }
    }

    // outer shiny bag does not count
    rec("shiny gold", &containers, &mut bag_count) - 1
}

fn main() {
    println!("part 1: {:?}", part1(&parse(INPUT)));
    println!("part 2: {:?}", part2(&parse(INPUT)));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

    #[test]
    fn check_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 4);
    }

    const TEST_INPUT_2: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#;

    #[test]
    fn check_part2_1() {
        assert_eq!(part2(&parse(TEST_INPUT)), 32);
    }
    #[test]
    fn check_part2_2() {
        assert_eq!(part2(&parse(TEST_INPUT_2)), 126);
    }
}
