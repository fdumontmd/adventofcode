use std::{collections::HashSet, fmt::Display};

use aoc_utils::grid::{Grid, Taxicab};

use crate::custom_error::AocError;

pub type Position = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
    Visited,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Path,
            b'#' => Tile::Forest,
            b'^' => Tile::SlopeUp,
            b'v' => Tile::SlopeDown,
            b'<' => Tile::SlopeLeft,
            b'>' => Tile::SlopeRight,
            _ => panic!("unknown tile {}", value as char),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Path => '.',
                Tile::Forest => '#',
                Tile::SlopeUp => '^',
                Tile::SlopeDown => 'v',
                Tile::SlopeLeft => '<',
                Tile::SlopeRight => '>',
                Tile::Visited => 'O',
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub pos: Position,
    pub path: HashSet<Position>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            pos: (1, 0),
            path: HashSet::new(),
        }
    }
}

fn can_move_into(grid: &Grid<Tile, Taxicab>, pos: Position, next: Position) -> bool {
    match grid[next] {
        Tile::Path => true,
        Tile::Forest => false,
        Tile::SlopeUp => pos == (next.0, next.1 + 1),
        Tile::SlopeDown => pos == (next.0, next.1 - 1),
        Tile::SlopeLeft => pos == (next.0 + 1, next.1),
        Tile::SlopeRight => pos == (next.0 - 1, next.1),
        Tile::Visited => unreachable!(),
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    // dfs (with a LIFO queue, i.e. Vec), and clone the used paths
    // on branching. Minimize cloning by only enqueuing alternatives,
    // i.e. if only one selectable neighbour, keep following
    let mut max_path: Option<State> = None;

    // when generating neighbours, filter out y == 0 (starting point)
    // dest is y == grid.height() - 1

    let mut queue = vec![State::default()];

    while let Some(mut s) = queue.pop() {
        loop {
            if s.pos.1 == grid.height() - 1 {
                let cur = max_path.as_ref().map(|s| s.path.len()).unwrap_or(0);
                if s.path.len() > cur {
                    max_path = Some(s);
                }
                break;
            } else {
                let neighbours: Vec<_> = grid
                    .neighbours(s.pos)
                    .filter(|n| n.1 > 0 && can_move_into(&grid, s.pos, *n) && !s.path.contains(n))
                    .collect();
                if neighbours.len() == 1 {
                    s.pos = neighbours[0];
                    s.path.insert(s.pos);
                } else {
                    for n in neighbours {
                        let mut ns = s.clone();
                        ns.pos = n;
                        ns.path.insert(n);
                        queue.push(ns);
                    }
                    break;
                }
            }
        }
    }

    Ok(format!("{}", max_path.unwrap().path.len()))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[rstest]
    #[case(INPUT, "94")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
