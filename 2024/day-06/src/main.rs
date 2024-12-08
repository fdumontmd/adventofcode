use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use aoc_utils::grid::Grid;

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Block,
    StartUp,
    StartDown,
    StartLeft,
    StartRight,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Block => '#',
            Tile::Empty => '.',
            Tile::StartUp => '^',
            Tile::StartDown => 'v',
            Tile::StartLeft => '<',
            Tile::StartRight => '>',
        };
        write!(f, "{c}")
    }
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Tile::Block,
            b'.' => Tile::Empty,
            b'^' => Tile::StartUp,
            b'v' => Tile::StartDown,
            b'<' => Tile::StartLeft,
            b'>' => Tile::StartRight,
            _ => panic!("unknow tile {}", value as char),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Tile> for Direction {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Block => panic!("should not start from block"),
            Tile::StartUp => Direction::Up,
            Tile::StartDown => Direction::Down,
            Tile::StartLeft => Direction::Left,
            Tile::StartRight => Direction::Right,
            Tile::Empty => panic!("should not start from empty"),
        }
    }
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn step_forward(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let delta = match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        pos.0
            .checked_add_signed(delta.0)
            .and_then(|x| pos.1.checked_add_signed(delta.1).map(|y| (x, y)))
    }
}

struct Visited([bool; 4]);

impl From<&Tile> for Visited {
    fn from(_: &Tile) -> Self {
        Visited([false; 4])
    }
}

impl Visited {
    fn visited(&self) -> bool {
        self.0.iter().any(|b| *b)
    }
}

impl Index<Direction> for Visited {
    type Output = bool;

    fn index(&self, index: Direction) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Direction> for Visited {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

fn visit(
    grid: &Grid<Tile, ()>,
    mut pos: (usize, usize),
    mut dir: Direction,
) -> Option<Grid<Visited, ()>> {
    let mut visited: Grid<Visited, ()> = grid.into();

    'main: loop {
        if visited[pos][dir] {
            return None;
        }
        visited[pos][dir] = true;
        loop {
            if let Some(new_pos) = dir.step_forward(pos) {
                if new_pos.0 < grid.width() && new_pos.1 < grid.height() {
                    if grid[new_pos] == Tile::Empty {
                        pos = new_pos;
                        break;
                    } else {
                        dir = dir.turn_right();
                        continue;
                    }
                }
            }
            break 'main;
        }
    }

    Some(visited)
}

fn part1(input: &str) -> usize {
    let mut grid: Grid<Tile, ()> = Grid::try_from(input).unwrap();

    let idx = grid
        .iter()
        .position(|t| {
            *t == Tile::StartUp
                || *t == Tile::StartDown
                || *t == Tile::StartLeft
                || *t == Tile::StartRight
        })
        .unwrap();

    let pos = grid.idx_to_pos(idx);
    let dir = Direction::from(grid[pos]);
    grid[pos] = Tile::Empty;

    visit(&grid, pos, dir)
        .unwrap()
        .iter()
        .filter(|v| v.visited())
        .count()
}

impl From<&Tile> for bool {
    fn from(_value: &Tile) -> Self {
        false
    }
}

fn part2(input: &str) -> usize {
    let mut grid: Grid<Tile, ()> = Grid::try_from(input).unwrap();

    let idx = grid
        .iter()
        .position(|t| {
            *t == Tile::StartUp
                || *t == Tile::StartDown
                || *t == Tile::StartLeft
                || *t == Tile::StartRight
        })
        .unwrap();

    let orig = grid.idx_to_pos(idx);
    let mut pos = orig;
    let mut dir = Direction::from(grid[pos]);
    grid[pos] = Tile::Empty;

    let mut loops: Grid<bool, ()> = (&grid).into();
    // can't put blocks on pos we already visited, so keep track of that
    let mut visited: Grid<bool, ()> = (&grid).into();

    'main: loop {
        loop {
            visited[pos] = true;
            if let Some(new_pos) = dir.step_forward(pos) {
                if new_pos.0 < grid.width() && new_pos.1 < grid.height() {
                    if grid[new_pos] == Tile::Empty {
                        if !visited[new_pos] {
                            // try to block it
                            grid[new_pos] = Tile::Block;
                            if visit(&grid, pos, dir).is_none() {
                                loops[new_pos] = true;
                            }
                            grid[new_pos] = Tile::Empty;
                        }

                        pos = new_pos;
                        break;
                    } else {
                        dir = dir.turn_right();
                        continue;
                    }
                }
            }
            break 'main;
        }
    }

    loops[orig] = false;

    loops.iter().filter(|b| **b).count()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test_case(TEST_INPUT, 41; "test input")]
    #[test_case(INPUT, 4722; "input")]
    fn test_part1(input: &str, steps: usize) {
        assert_eq!(steps, part1(input));
    }

    #[test_case(TEST_INPUT, 6; "test input")]
    #[test_case(INPUT, 1602; "input")]
    fn test_part2(input: &str, loops: usize) {
        assert_eq!(loops, part2(input));
    }
}
