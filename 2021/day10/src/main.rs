static INPUT: &str = include_str!("input.txt");

fn error_score(b: u8) -> u64 {
    match b {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!("unknown closer '{}'", b as char),
    }
}

fn matching(b: u8) -> u8 {
    match b {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => panic!("unknown opener '{}'", b as char),
    }
}

fn input_error_score(input: &str) -> u64 {
    let mut stack = Vec::new();

    for b in input.bytes() {
        match b {
            b'(' | b'[' | b'{' | b'<' => stack.push(b),
            b')' | b']' | b'}' | b'>' => {
                if let Some(o) = stack.pop() {
                    let o = matching(o);
                    if b != o {
                        // eprintln!("Expected {}, but found {} instead.", o as char, b as char);
                        return error_score(b);
                    }
                } else {
                    // eprintln!("Unexpected closing '{}'", b as char);
                    return error_score(b);
                }
            }
            _ => panic!("unexpected char {}", b as char),
        }
    }
    0
}

fn part_1(input: &str) -> u64 {
    input.lines().map(input_error_score).sum()
}

fn input_complete_score(input: &str) -> u64 {
    let mut stack = Vec::new();

    for b in input.bytes() {
        match b {
            b'(' | b'[' | b'{' | b'<' => stack.push(b),
            b')' | b']' | b'}' | b'>' => {
                if let Some(o) = stack.pop() {
                    let o = matching(o);
                    assert_eq!(b, o);
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    stack.reverse();
    stack
        .into_iter()
        .fold(0, |s, b| s * 5 + complete_score(matching(b)))
}

fn complete_score(b: u8) -> u64 {
    match b {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => unreachable!(),
    }
}

fn part_2(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .lines()
        .filter(|l| input_error_score(l) == 0)
        .map(input_complete_score)
        .collect();

    assert!(scores.len() % 2 == 1);
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    static TEST_INPUT: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() {
        assert_eq!(26397, part_1(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(288957, part_2(TEST_INPUT));
    }
}
