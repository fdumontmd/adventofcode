use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for line in input.lines() {
        let nums: Vec<_> = line.split_whitespace().collect();
        left.push(nums[0].parse().unwrap());
        right.push(nums[1].parse().unwrap());
    }
    (left, right)
}

pub fn part1(input: &str) -> usize {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn elt_count(data: impl IntoIterator<Item = usize>) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();

    for i in data {
        *counts.entry(i).or_insert(0) += 1;
    }

    counts
}

pub fn part2(input: &str) -> usize {
    let (left, right) = parse(input);

    let count = elt_count(right);

    left.into_iter()
        .map(|l| l * count.get(&l).unwrap_or(&0))
        .sum()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test_case(TEST_INPUT, 11)]
    #[test_case(INPUT, 2904518)]
    pub fn test_part1(input: &str, val: usize) {
        assert_eq!(val, part1(input));
    }

    #[test_case(TEST_INPUT, 31)]
    #[test_case(INPUT, 18650129)]
    pub fn test_part2(input: &str, val: usize) {
        assert_eq!(val, part2(input));
    }
}
