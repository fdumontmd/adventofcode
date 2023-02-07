use regex::Regex;

const INPUT: &str = include_str!("input.txt");

struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

fn parse_input(input: &str) -> Vec<(Policy, String)> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

    input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
        let cap = re.captures(line).unwrap();
        let min: usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        let letter: char = cap.get(3).unwrap().as_str().chars().next().unwrap();
        let password: String = cap.get(4).unwrap().as_str().to_string();
        (Policy { min, max, letter }, password)
    })
    .collect()
}

fn policy_match_01(policy: &Policy, password: &str) -> bool {
    let count = password.chars().filter(|c| *c == policy.letter).count();
    count >= policy.min && count <= policy.max
}

fn part1(data: &Vec<(Policy, String)>) -> usize {
    data.iter().filter(|&(policy, password)| policy_match_01(&policy, &password)).count()
}

fn policy_match_02(policy: &Policy, password: &str) -> bool {
    let chars: Vec<_> = password.chars().collect();
    let mut matching = 0;
    if chars.len() >= policy.min  && chars[policy.min - 1] == policy.letter {
        matching += 1;

    }
    if chars.len() >= policy.max  && chars[policy.max - 1] == policy.letter {
        matching += 1;
    }

    matching == 1
}

fn part2(data: &Vec<(Policy, String)>) -> usize {
    data.iter().filter(|&(policy, password)| policy_match_02(&policy, &password)).count()
}

fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

    #[test]
    fn check_part1() {
        let input = parse_input(DATA);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn check_part2() {
        let input = parse_input(DATA);
        assert_eq!(part2(&input), 1);
    }
}
