use std::collections::HashSet;
const INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> usize {
    let mut acc: Vec<u8> = Vec::new();
    let mut total = 0;

    for line in input.lines() {
        if line.is_empty() {
            acc = acc.iter().cloned().filter(|c| c.is_ascii_lowercase()).collect();
            acc.sort();
            let mut set = HashSet::new();
            acc.iter().for_each(|c| {set.insert(*c);});
            total += set.len();
            acc.clear();
        } else {
            acc.extend_from_slice(line.as_bytes());
        }
    }

    if !acc.is_empty() {
        acc = acc.iter().cloned().filter(|c| c.is_ascii_lowercase()).collect();
        acc.sort();
        let mut set = HashSet::new();
        acc.iter().for_each(|c| {set.insert(*c);});
        total += set.len();
        acc.clear();
    }

    total
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    let mut set: HashSet<u8> = HashSet::new();
    let mut newgroup = true;

    for line in input.lines() {
        if line.is_empty() {
            total += set.len();
            set = HashSet::new();
            newgroup = true;
        } else {
            let mut newset = HashSet::new();
            for c in line.as_bytes() {
                if c.is_ascii_lowercase() {
                    newset.insert(*c);
                }
            }
            if newgroup {
                set = newset;
            } else {
                set = set.intersection(&newset).cloned().collect();
            }
            newgroup = false;
        }
    }

    total += set.len();
    
    total
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#;
    #[test]
    fn check_part1() { assert_eq!(part1(TEST_DATA), 11);
    }

    #[test]
    fn check_part2() { assert_eq!(part2(TEST_DATA), 6);
    }
}

