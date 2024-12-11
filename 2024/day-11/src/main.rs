use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

// keep track of individual numbers and stone counts instead of
// processing each stone separately
fn count_stones_after(stones: Vec<usize>, steps: usize) -> usize {
    let mut counts = HashMap::new();

    for stone in stones {
        *counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_counts = HashMap::new();
        for (stone, count) in counts {
            if stone == 0 {
                *new_counts.entry(1).or_insert(0) += count;
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let shift = 10usize.pow(digits / 2);
                    *new_counts.entry(stone / shift).or_insert(0) += count;
                    *new_counts.entry(stone % shift).or_insert(0) += count;
                } else {
                    *new_counts.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }
        counts = new_counts;
    }

    counts.values().sum()
}

fn part1(input: &str) -> usize {
    count_stones_after(parse(input), 25)
}

fn part2(input: &str) -> usize {
    count_stones_after(parse(input), 75)
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT_1: &str = "0 1 10 99 999";

    const TEST_INPUT_2: &str = "125 17";

    #[test_case(TEST_INPUT_1, 1, 7; "test input 1, 1 blink")]
    #[test_case(TEST_INPUT_2, 1, 3; "test input 2, 1 blink")]
    #[test_case(TEST_INPUT_2, 2, 4; "test input 2, 2 blink")]
    #[test_case(TEST_INPUT_2, 3, 5; "test input 2, 3 blink")]
    #[test_case(TEST_INPUT_2, 4, 9; "test input 2, 4 blink")]
    #[test_case(TEST_INPUT_2, 5, 13; "test input 2, 5 blink")]
    #[test_case(TEST_INPUT_2, 6, 22; "test input 2, 6 blink")]
    #[test_case(INPUT, 25, 194782; "input")]
    fn test_part1(input: &str, steps: usize, count: usize) {
        assert_eq!(count, count_stones_after(parse(input), steps));
    }

    #[test_case(INPUT, 233007586663131; "input")]
    fn test_part2(input: &str, count: usize) {
        assert_eq!(count, part2(input));
    }
}
