use std::collections::HashSet;

use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn follow_if_needed(h: (i32, i32), mut t: (i32, i32)) -> (i32, i32) {
    let delta_x = h.0 - t.0;
    let delta_y = h.1 - t.1;

    if delta_x.abs() > 1 || delta_y.abs() > 1 {
        t.0 += delta_x.clamp(-1, 1);
        t.1 += delta_y.clamp(-1, 1);
    }
    t
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn adjust_pos(&self, pos: (i32, i32)) -> (i32, i32) {
        let d = self.delta();
        (pos.0 + d.0, pos.1 + d.1)
    }
}

fn parse_command(line: &str) -> (Direction, usize) {
    if let Some((d, dist)) = line.split(' ').tuples().next() {
        let dir = match d {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Unknown direction in command {}", line),
        };
        let dist = dist.parse().expect("cannot parse distance in {}");

        return (dir, dist);
    }
    panic!("Could not parse command {}", line)
}

fn run_sim(input: &str, rope_len: usize) -> usize {
    let mut visited = HashSet::new();
    let mut knots = vec![(0, 0); rope_len];
    visited.insert(knots[rope_len - 1]);
    for (dir, dist) in input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_command)
    {
        for _ in 0..dist {
            knots[0] = dir.adjust_pos(knots[0]);
            let mut iter = knots.iter_mut().peekable();
            while let Some(h) = iter.next() {
                if let Some(t) = iter.peek_mut() {
                    **t = follow_if_needed(*h, **t);
                }
            }

            visited.insert(knots[rope_len - 1]);
        }
    }
    visited.len()
}

fn part_01(input: &str) -> usize {
    run_sim(input, 2)
}

fn part_02(input: &str) -> usize {
    run_sim(input, 10)
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::*;
    static TEST_INPUT: &str = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    static LARGER_TEST_INPUT: &str = r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test_case((2, 1), (1, 1), (1, 1))]
    #[test_case((1, 1), (2, 2), (2, 2))]
    #[test_case((1, 1), (1, 1), (1, 1))]
    #[test_case((3, 1), (1, 1), (2, 1))]
    #[test_case((1, 3), (1, 1), (1, 2))]
    #[test_case((2, 1), (1, 3), (2, 2))]
    #[test_case((3, 2), (1, 3), (2, 2))]
    fn test_follow(h: (i32, i32), t: (i32, i32), e: (i32, i32)) {
        assert_eq!(e, follow_if_needed(h, t));
    }

    #[test_case(13, TEST_INPUT)]
    #[test_case(5779, INPUT)]
    fn test_part_1(count: usize, input: &str) {
        assert_eq!(count, part_01(input));
    }

    #[test_case(1, TEST_INPUT)]
    #[test_case(36, LARGER_TEST_INPUT)]
    #[test_case(2331, INPUT)]
    fn test_part_2(count: usize, input: &str) {
        assert_eq!(count, part_02(input));
    }
}
