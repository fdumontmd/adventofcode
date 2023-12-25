use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use aoc_utils::grid::{Grid, Taxicab};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tile {
    Start,
    Plot,
    Rock,
    Visited,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Tile::Start,
            b'.' => Tile::Plot,
            b'#' => Tile::Rock,
            _ => panic!("Unknown tile {}", value as char),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Start => 'S',
                Tile::Plot => '.',
                Tile::Rock => '#',
                Tile::Visited => 'O',
            }
        )
    }
}

#[tracing::instrument]
pub fn reachable_in(input: &str, max_steps: usize) -> usize {
    let mut grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    let pos = grid.idx_to_pos(grid.iter().position(|&t| t == Tile::Start).unwrap());
    grid[pos] = Tile::Plot;

    let mut visited: HashSet<(bool, (usize, usize))> = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((0, pos));

    while let Some((steps, pos)) = queue.pop_front() {
        if visited.contains(&(steps % 2 == 0, pos)) {
            continue;
        }

        visited.insert((steps % 2 == 0, pos));

        if steps >= max_steps {
            continue;
        }

        for n in grid.neighbours(pos) {
            if grid[n] == Tile::Plot {
                queue.push_back((steps + 1, n));
            }
        }
    }

    visited
        .into_iter()
        .filter(|(steps, _)| *steps == (max_steps % 2 == 0))
        .count()
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    Ok(format!("{}", reachable_in(input, 64)))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[rstest]
    #[case(INPUT, 6, 16)]
    fn test_reachable_in(#[case] input: &str, #[case] steps: usize, #[case] reachable: usize) {
        assert_eq!(reachable, reachable_in(input, steps));
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("3737", process(include_str!("../input.txt"))?);
        Ok(())
    }
}
