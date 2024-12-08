use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc_utils::{
    grid::{Grid, Taxicab},
    num::extended_gcd,
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Antenna(u8),
    Empty,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        if value == b'.' {
            Tile::Empty
        } else {
            Tile::Antenna(value)
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Antenna(v) => write!(f, "{}", *v as char),
            Tile::Empty => write!(f, "."),
        }
    }
}

fn part1(input: &str) -> usize {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();
    let mut unique_pos = HashSet::new();
    let mut antennas = HashMap::new();

    for (idx, tile) in grid.iter().enumerate() {
        if let Tile::Antenna(v) = *tile {
            antennas.entry(v).or_insert(vec![]).push(idx);
        }
    }

    for (_, idxs) in antennas {
        for (i1, idx1) in idxs.iter().enumerate() {
            for (i2, idx2) in idxs.iter().enumerate() {
                if i1 != i2 {
                    let pos1 = grid.idx_to_pos(*idx1);
                    let pos2 = grid.idx_to_pos(*idx2);
                    let pos1 = (pos1.0 as isize, pos1.1 as isize);
                    let pos2 = (pos2.0 as isize, pos2.1 as isize);

                    let delta = (pos2.0 - pos1.0, pos2.1 - pos1.1);

                    let antinode = (pos2.0 + delta.0, pos2.1 + delta.1);

                    if antinode.0 >= 0
                        && antinode.1 >= 0
                        && antinode.0 < grid.width() as isize
                        && antinode.1 < grid.height() as isize
                    {
                        unique_pos.insert(antinode);
                    }
                }
            }
        }
    }

    unique_pos.len()
}

fn part2(input: &str) -> usize {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();
    let mut unique_pos = HashSet::new();
    let mut antennas = HashMap::new();

    for (idx, tile) in grid.iter().enumerate() {
        if let Tile::Antenna(v) = *tile {
            antennas.entry(v).or_insert(vec![]).push(idx);
        }
    }

    for (_, idxs) in antennas {
        for (i1, idx1) in idxs.iter().enumerate() {
            for (i2, idx2) in idxs.iter().enumerate() {
                if i1 != i2 {
                    let pos1 = grid.idx_to_pos(*idx1);
                    let pos2 = grid.idx_to_pos(*idx2);
                    let pos1 = (pos1.0 as i64, pos1.1 as i64);
                    let pos2 = (pos2.0 as i64, pos2.1 as i64);

                    let delta = (pos2.0 - pos1.0, pos2.1 - pos1.1);
                    // changes: simplify delta by dividing by gcd; then repeat
                    // adding until out of bound
                    let gcd = extended_gcd(delta.0, delta.1);
                    let delta = (delta.0 / gcd.0.abs(), delta.1 / gcd.0.abs());

                    let mut antinode = pos2;

                    while antinode.0 >= 0
                        && antinode.1 >= 0
                        && antinode.0 < grid.width() as i64
                        && antinode.1 < grid.height() as i64
                    {
                        unique_pos.insert(antinode);
                        antinode = (antinode.0 + delta.0, antinode.1 + delta.1);
                    }
                }
            }
        }
    }

    unique_pos.len()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test_case(TEST_INPUT, 14; "test input")]
    #[test_case(INPUT, 295; "input")]
    fn test_part1(input: &str, antinodes: usize) {
        assert_eq!(antinodes, part1(input));
    }

    #[test_case(TEST_INPUT, 34; "test input")]
    #[test_case(INPUT, 1034; "input")]
    fn test_part2(input: &str, antinodes: usize) {
        assert_eq!(antinodes, part2(input));
    }
}
