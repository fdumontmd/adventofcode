use std::{collections::HashSet, fmt::Display};

use aoc_utils::grid::{Grid, Taxicab};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    MirrorDown,
    MirrorUp,
    SplitVert,
    SplitHoriz,
    Visited,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Empty,
            b'\\' => Tile::MirrorDown,
            b'/' => Tile::MirrorUp,
            b'|' => Tile::SplitVert,
            b'-' => Tile::SplitHoriz,
            _ => panic!("Unknow tile {}", value as char),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn step(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let delta = match self {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        };

        pos.0
            .checked_add_signed(delta.0)
            .and_then(|x| pos.1.checked_add_signed(delta.1).map(|y| (x, y)))
    }
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        self == &Tile::Empty
    }
    pub fn can_reflect(&self) -> bool {
        self == &Tile::MirrorDown || self == &Tile::MirrorUp
    }

    pub fn reflect(&self, dir: Direction) -> Direction {
        match self {
            Tile::MirrorDown => match dir {
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            },
            Tile::MirrorUp => match dir {
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            },
            _ => panic!("should not call reflect on {self}"),
        }
    }

    pub fn can_split(&self, dir: Direction) -> bool {
        (self == &Tile::SplitHoriz && (dir == Direction::Up || dir == Direction::Down))
            || (self == &Tile::SplitVert && (dir == Direction::Left || dir == Direction::Right))
    }

    pub fn split(&self) -> (Direction, Direction) {
        match self {
            Tile::SplitVert => (Direction::Up, Direction::Down),
            Tile::SplitHoriz => (Direction::Left, Direction::Right),
            _ => panic!("should not call split on {self}"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::MirrorDown => '\\',
                Tile::MirrorUp => '/',
                Tile::SplitVert => '|',
                Tile::SplitHoriz => '-',
                Tile::Visited => '#',
            }
        )
    }
}

pub fn try_step(
    grid: &Grid<Tile, Taxicab>,
    pos: (usize, usize),
    dir: Direction,
) -> Option<(usize, usize)> {
    dir.step(pos)
        .filter(|&(x, y)| x < grid.width() && y < grid.height())
}

pub fn solve_from(grid: &Grid<Tile, Taxicab>, pos: (usize, usize), dir: Direction) -> usize {
    // just
    let mut visited = HashSet::new();
    // path: coord + direction
    let mut paths = HashSet::new();

    let mut stack = Vec::new();

    stack.push((pos, dir));

    'main_loop: while let Some((mut pos, dir)) = stack.pop() {
        if paths.contains(&(pos, dir)) {
            continue;
        }
        visited.insert(pos);
        paths.insert((pos, dir));

        while grid[pos].is_empty() {
            if let Some(step) = try_step(grid, pos, dir) {
                pos = step;
            } else {
                continue 'main_loop;
            }
            if paths.contains(&(pos, dir)) {
                continue 'main_loop;
            }
            visited.insert(pos);
            paths.insert((pos, dir));
        }

        let t = grid[pos];

        if t.can_reflect() {
            let new_dir = t.reflect(dir);
            if let Some(step) = try_step(grid, pos, new_dir) {
                stack.push((step, new_dir));
            }
        } else if t.can_split(dir) {
            let (dir1, dir2) = t.split();
            if let Some(step) = try_step(grid, pos, dir1) {
                stack.push((step, dir1));
            }
            if let Some(step) = try_step(grid, pos, dir2) {
                stack.push((step, dir2));
            }
        } else if let Some(step) = try_step(grid, pos, dir) {
            stack.push((step, dir));
        }
    }

    visited.len()
}
