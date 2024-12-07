const INPUT: &str = include_str!("input.txt");

fn solve_part1(target: usize, head: usize, rest: &[usize]) -> bool {
    if rest.is_empty() {
        target == head
    } else {
        solve_part1(target, head + rest[0], &rest[1..])
            || solve_part1(target, head * rest[0], &rest[1..])
    }
}

fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let line: Vec<&str> = line.split(": ").collect();
    let target: usize = line[0].parse().unwrap();
    (
        target,
        line[1]
            .split_whitespace()
            .map(|d| d.parse().unwrap())
            .collect(),
    )
}

fn part1(input: &str) -> usize {
    // only two operators, and only left to right
    // solve recursively: take a list of values,
    // then for each of '*' and '+', apply to the head
    // then try with the rest
    input
        .lines()
        .map(|line| {
            let (target, values) = parse_line(line);
            if solve_part1(target, values[0], &values[1..]) {
                target
            } else {
                0
            }
        })
        .sum()
}

fn solve_part2(target: usize, head: usize, rest: &[usize]) -> bool {
    if rest.is_empty() {
        target == head
    } else {
        solve_part2(target, head + rest[0], &rest[1..])
            || solve_part2(target, head * rest[0], &rest[1..])
            || solve_part2(target, concat_numbers(head, rest[0]), &rest[1..])
    }
}

fn concat_numbers(head: usize, rest: usize) -> usize {
    head * 10usize.pow(rest.ilog10() + 1) + rest
}

fn part2(input: &str) -> usize {
    // only two operators, and only left to right
    // solve recursively: take a list of values,
    // then for each of '*' and '+', apply to the head
    // then try with the rest
    input
        .lines()
        .map(|line| {
            let (target, values) = parse_line(line);
            if solve_part2(target, values[0], &values[1..]) {
                target
            } else {
                0
            }
        })
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

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test_case(TEST_INPUT, 3749; "test input")]
    #[test_case(INPUT, 2299996598890; "input")]
    fn test_part1(input: &str, target: usize) {
        assert_eq!(target, part1(input));
    }

    #[test_case(TEST_INPUT, 11387; "test input")]
    #[test_case(INPUT, 362646859298554; "input")]
    fn test_part2(input: &str, target: usize) {
        assert_eq!(target, part2(input));
    }

    #[test_case(15, 6, 156)]
    #[test_case(8, 6, 86)]
    #[test_case(17, 8, 178)]
    #[test_case(10, 10, 1010)]
    fn test_concat_numbers(head: usize, rest: usize, target: usize) {
        assert_eq!(target, concat_numbers(head, rest));
    }
}
