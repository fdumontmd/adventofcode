use std::{collections::HashMap, fmt::Display};

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, PartialOrd, Debug, Default, Copy, Clone)]
enum Part1Tile {
    #[default]
    Clean,
    Infected,
}

impl From<char> for Part1Tile {
    fn from(value: char) -> Self {
        if value == '#' {
            Part1Tile::Infected
        } else {
            Part1Tile::Clean
        }
    }
}

impl Display for Part1Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Part1Tile::Clean => '.',
            Part1Tile::Infected => '#',
        };
        write!(f, "{c}")
    }
}

// the assumption is that burst_node will change the node, so
// any Infected output is a change to Infected.
// If Infected nodes could stay Infected, we'd have to track
// changes instead of just output
trait Burster {
    fn burst_node(&self) -> Self;
    fn burst_direction(&self, d: Direction) -> Direction;
}

impl Burster for Part1Tile {
    fn burst_node(&self) -> Self {
        match self {
            Part1Tile::Clean => Part1Tile::Infected,
            Part1Tile::Infected => Part1Tile::Clean,
        }
    }

    fn burst_direction(&self, d: Direction) -> Direction {
        match self {
            Part1Tile::Clean => d.turn_left(),
            Part1Tile::Infected => d.turn_right(),
        }
    }
}

struct Grid<T>(HashMap<(isize, isize), T>);

impl<T: From<char>> Grid<T> {
    fn from_str(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let mut map = HashMap::new();
        let width = lines[0].len();
        let height = lines.len();

        let mid_width = (width / 2) as isize;
        let mid_height = (height / 2) as isize;

        for (row, line) in lines.into_iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                map.insert(
                    (col as isize - mid_width, row as isize - mid_height),
                    T::from(c),
                );
            }
        }

        Self(map)
    }
}

impl<T: Clone + Display + Default> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0.is_empty() {
            let min_col = self.0.keys().map(|(c, _)| *c).min().unwrap();
            let max_col = self.0.keys().map(|(c, _)| *c).max().unwrap();

            let min_row = self.0.keys().map(|(_, r)| *r).min().unwrap();
            let max_row = self.0.keys().map(|(_, r)| *r).max().unwrap();

            for r in min_row..=max_row {
                for c in min_col..=max_col {
                    let t = self.0.get(&(c, r)).cloned().unwrap_or_default();
                    write!(f, "{t}")?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        }
    }

    fn forward(&self, pos: (isize, isize)) -> (isize, isize) {
        let d = self.delta();
        (pos.0 + d.0, pos.1 + d.1)
    }
}

struct Virus<T> {
    grid: Grid<T>,
    pos: (isize, isize),
    direction: Direction,
}

impl<T> Virus<T> {
    fn new(grid: Grid<T>) -> Self {
        Self {
            grid,
            pos: (0, 0),
            direction: Direction::Up,
        }
    }
}

impl<T: Burster + Default + Copy> Virus<T> {
    fn burst(&mut self) -> T {
        let t = self.grid.0.get(&self.pos).cloned().unwrap_or_default();
        let new_t = t.burst_node();
        let new_dir = t.burst_direction(self.direction);

        self.grid.0.insert(self.pos, new_t);
        self.direction = new_dir;
        self.pos = new_dir.forward(self.pos);

        new_t
    }
}

impl<T: Burster + Default + Copy> Iterator for Virus<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.burst())
    }
}

fn part1(input: &str) -> usize {
    let grid: Grid<Part1Tile> = Grid::from_str(input);
    Virus::new(grid)
        .take(10000)
        .filter(|&t| t == Part1Tile::Infected)
        .count()
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Default, Copy, Clone)]
enum Part2Tile {
    #[default]
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl From<char> for Part2Tile {
    fn from(value: char) -> Self {
        if value == '#' {
            Part2Tile::Infected
        } else {
            Part2Tile::Clean
        }
    }
}

impl Burster for Part2Tile {
    fn burst_node(&self) -> Self {
        match self {
            Part2Tile::Clean => Part2Tile::Weakened,
            Part2Tile::Weakened => Part2Tile::Infected,
            Part2Tile::Infected => Part2Tile::Flagged,
            Part2Tile::Flagged => Part2Tile::Clean,
        }
    }

    fn burst_direction(&self, d: Direction) -> Direction {
        match self {
            Part2Tile::Clean => d.turn_left(),
            Part2Tile::Weakened => d,
            Part2Tile::Infected => d.turn_right(),
            Part2Tile::Flagged => d.turn_left().turn_left(),
        }
    }
}

fn part2(input: &str) -> usize {
    let grid: Grid<Part2Tile> = Grid::from_str(input);
    Virus::new(grid)
        .take(10000000)
        .filter(|&t| t == Part2Tile::Infected)
        .count()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "..#
#..
...";

    #[test_case(TEST_INPUT, 5587)]
    #[test_case(INPUT, 5404)]
    fn test_part1(input: &str, count: usize) {
        assert_eq!(count, part1(input));
    }

    #[test_case(TEST_INPUT, 2511944)]
    #[test_case(INPUT, 2511672)]
    fn test_part2(input: &str, count: usize) {
        assert_eq!(count, part2(input));
    }
}
