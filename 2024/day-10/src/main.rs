use std::{collections::HashSet, fmt::Display};

use aoc_utils::grid::{Grid, Taxicab};

const INPUT: &[u8] = include_bytes!("input.txt");

struct Height(u8);

impl From<u8> for Height {
    fn from(value: u8) -> Self {
        if value.is_ascii_digit() {
            Height(value - b'0')
        } else {
            panic!("invalid value {}", value as char)
        }
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn part1(input: &[u8]) -> usize {
    let grid: Grid<Height, Taxicab> = Grid::try_from(input).unwrap();
    let starting_idx: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, h)| if h.0 == 0 { Some(idx) } else { None })
        .collect();

    starting_idx
        .into_iter()
        .map(|idx| {
            let mut summits = HashSet::new();

            let mut stack = vec![idx];

            while let Some(idx) = stack.pop() {
                let pos = grid.idx_to_pos(idx);
                let level = grid[pos].0;
                if level == 9 {
                    summits.insert(pos);
                } else {
                    for n in grid.neighbours(pos).filter(|n| grid[*n].0 == level + 1) {
                        stack.push(grid.pos_to_idx(n))
                    }
                }
            }

            summits.len()
        })
        .sum()
}

// surprisingly not slow
// maybe memoizing would help. maybe. but it's already fast
fn part2(input: &[u8]) -> usize {
    let grid: Grid<Height, Taxicab> = Grid::try_from(input).unwrap();
    let starting_idx: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(idx, h)| if h.0 == 0 { Some(idx) } else { None })
        .collect();

    starting_idx
        .into_iter()
        .map(|idx| {
            let mut trails = 0;

            let mut stack = vec![idx];

            while let Some(idx) = stack.pop() {
                let pos = grid.idx_to_pos(idx);
                let level = grid[pos].0;
                if level == 9 {
                    trails += 1;
                } else {
                    for n in grid.neighbours(pos).filter(|n| grid[*n].0 == level + 1) {
                        stack.push(grid.pos_to_idx(n))
                    }
                }
            }

            trails
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

    const TEST_INPUT: &[u8] = b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test_case(TEST_INPUT, 36; "test input")]
    #[test_case(INPUT, 733; "input")]
    fn test_part1(input: &[u8], score: usize) {
        assert_eq!(score, part1(input));
    }

    #[test_case(TEST_INPUT, 81; "test input")]
    #[test_case(INPUT, 1514; "input")]
    fn test_part2(input: &[u8], score: usize) {
        assert_eq!(score, part2(input));
    }
}
