use std::sync::LazyLock;

use regex::Regex;

const INPUT: &str = include_str!("input.txt");

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

fn part1(input: &str) -> usize {
    let mut sum = 0;

    for (_, [left, right]) in RE.captures_iter(input).map(|c| c.extract()) {
        sum += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
    }

    sum
}

fn part2(mut input: &str) -> usize {
    let mut sum = 0;
    let mut fragment;

    while !input.is_empty() {
        (fragment, input) = if let Some(pos) = input.find("don't()") {
            (&input[..pos], &input[pos..])
        } else {
            (input, "")
        };

        sum += part1(fragment);

        input = if let Some(pos) = input.find("do()") {
            &input[pos + 4..]
        } else {
            ""
        }
    }

    sum
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        161; "test"
    )]
    #[test_case(INPUT, 183669043; "input")]
    fn test_part_1(input: &str, value: usize) {
        assert_eq!(value, part1(input));
    }

    #[test_case(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        48; "test"
    )]
    #[test_case(INPUT, 59097164; "input")]
    fn test_part_2(input: &str, value: usize) {
        assert_eq!(value, part2(input));
    }
}
