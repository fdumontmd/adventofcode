use std::fmt::Display;

use anyhow::{bail, Error};
use aoc_utils::grid::{Grid, Taxicab};
#[derive(Eq, PartialEq)]
enum Tile {
    East,
    South,
    Empty,
}

static INPUT: &str = include_str!("input.txt");

impl TryFrom<u8> for Tile {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'>' => Tile::East,
            b'v' => Tile::South,
            b'.' => Tile::Empty,
            _ => bail!("unknown tile {}", value as char),
        })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::East => '>',
            Tile::South => 'v',
            Tile::Empty => '.',
        };
        write!(f, "{c}")
    }
}

type Map = Grid<Tile, Taxicab>;

fn parse_map(input: &str) -> Result<Map, Error> {
    Grid::try_from(input)
}

fn move_east(map: &mut Map) -> bool {
    let mut to_move = vec![];
    for (idx, t) in map.iter().enumerate() {
        if t == &Tile::East {
            let from = map.idx_to_pos(idx);
            let to = ((from.0 + 1) % map.width(), from.1);
            if map[to] == Tile::Empty {
                to_move.push((from, to));
            }
        }
    }

    let moved = !to_move.is_empty();

    for (from, to) in to_move {
        map[from] = Tile::Empty;
        map[to] = Tile::East;
    }

    moved
}

fn move_south(map: &mut Map) -> bool {
    let mut to_move = vec![];
    for (idx, t) in map.iter().enumerate() {
        if t == &Tile::South {
            let from = map.idx_to_pos(idx);
            let to = (from.0, (from.1 + 1) % map.height());
            if map[to] == Tile::Empty {
                to_move.push((from, to));
            }
        }
    }

    let moved = !to_move.is_empty();

    for (from, to) in to_move {
        map[from] = Tile::Empty;
        map[to] = Tile::South;
    }

    moved
}

fn step(map: &mut Map) -> bool {
    let moved = move_east(map);
    move_south(map) || moved
}

fn part_1(input: &str) -> usize {
    let mut map = parse_map(input).unwrap();
    for s in 1.. {
        let moved = step(&mut map);
        if !moved {
            return s;
        }
    }
    0
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, INPUT};

    static TEST_INPUT: &str = r"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_part_1() {
        assert_eq!(58, part_1(TEST_INPUT));
        assert_eq!(384, part_1(INPUT));
    }
}
