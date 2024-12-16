use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    ops::{Index, IndexMut},
};

use aoc_utils::grid::Grid;

const INPUT: &str = include_str!("input.txt");

enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Tile::Wall,
            b'.' => Tile::Empty,
            b'S' => Tile::Start,
            b'E' => Tile::End,
            _ => panic!("unknown tile '{}'", value as char),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Start => 'S',
            Tile::End => 'E',
        };
        write!(f, "{c}")
    }
}

impl Tile {
    fn is_empty(&self) -> bool {
        !matches!(self, Tile::Wall)
    }

    fn is_end(&self) -> bool {
        matches!(self, Tile::End)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }

    // check that the move is valid before attempting it
    fn move_from(&self, pos: (usize, usize)) -> (usize, usize) {
        let d = self.delta();
        (
            pos.0.checked_add_signed(d.0).unwrap(),
            pos.1.checked_add_signed(d.1).unwrap(),
        )
    }

    fn turn_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }

    fn turn_counterclockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }
}

fn part1(input: &str) -> usize {
    let grid: Grid<Tile, ()> = Grid::try_from(input).unwrap();

    let idx = grid
        .iter()
        .enumerate()
        .find_map(|(idx, t)| {
            if matches!(t, Tile::Start) {
                Some(idx)
            } else {
                None
            }
        })
        .unwrap();
    let pos = grid.idx_to_pos(idx);

    // we'll get the states in order of rising cost, so if a state is already in seen, that means
    // we already have a cheaper path for the same state
    let mut seen: Grid<Seen, ()> = (&grid).into();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), pos, Direction::East));

    while let Some((Reverse(cost), pos, direction)) = queue.pop() {
        seen[pos][direction] = true;

        if grid[pos].is_end() {
            return cost;
        }
        let forward = direction.move_from(pos);

        if grid[forward].is_empty() && !seen[forward][direction] {
            queue.push((Reverse(cost + 1), forward, direction));
        }

        if !seen[pos][direction.turn_clockwise()] {
            queue.push((Reverse(cost + 1000), pos, direction.turn_clockwise()));
        }

        if !seen[pos][direction.turn_counterclockwise()] {
            queue.push((Reverse(cost + 1000), pos, direction.turn_counterclockwise()));
        }
    }
    unreachable!()
}

struct Seen([bool; 4]);

impl From<&Tile> for Seen {
    fn from(_value: &Tile) -> Self {
        Seen([false; 4])
    }
}

impl Index<Direction> for Seen {
    type Output = bool;

    fn index(&self, index: Direction) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Direction> for Seen {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

fn part2(input: &str) -> usize {
    let grid: Grid<Tile, ()> = Grid::try_from(input).unwrap();

    let idx = grid
        .iter()
        .enumerate()
        .find_map(|(idx, t)| {
            if matches!(t, Tile::Start) {
                Some(idx)
            } else {
                None
            }
        })
        .unwrap();
    let pos = grid.idx_to_pos(idx);

    // we'll get the states in order of rising cost, so if a state is already in seen, that means
    // we already have a cheaper path for the same state
    let mut seen: Grid<Seen, ()> = (&grid).into();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), pos, Direction::East, vec![pos]));

    // actually faster than tagging each pos in best path on grid then counting
    // at least faster in debug mode
    let mut best_seats = HashSet::new();
    let mut best_path_cost = None;

    while let Some((Reverse(cost), pos, direction, path)) = queue.pop() {
        seen[pos][direction] = true;
        if best_path_cost
            .map(|best_path_cost| best_path_cost < cost)
            .unwrap_or(false)
        {
            break;
        }
        if grid[pos].is_end() {
            best_path_cost = Some(cost);
            for p in path {
                best_seats.insert(p);
            }
            continue;
        }
        let forward = direction.move_from(pos);

        if grid[forward].is_empty() && !seen[forward][direction] {
            let mut path = path.clone();
            path.push(forward);
            queue.push((Reverse(cost + 1), forward, direction, path));
        }

        if !seen[pos][direction.turn_clockwise()] {
            queue.push((
                Reverse(cost + 1000),
                pos,
                direction.turn_clockwise(),
                path.clone(),
            ));
        }

        if !seen[pos][direction.turn_counterclockwise()] {
            queue.push((
                Reverse(cost + 1000),
                pos,
                direction.turn_counterclockwise(),
                path,
            ));
        }
    }

    best_seats.len()
}
fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST_INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test_case(TEST_INPUT_1, 7036; "test input 1")]
    #[test_case(TEST_INPUT_2, 11048; "test input 2")]
    #[test_case(INPUT, 104516; "input")]
    fn test_part1(input: &str, score: usize) {
        assert_eq!(score, part1(input));
    }

    #[test_case(TEST_INPUT_1, 45; "test input 1")]
    #[test_case(TEST_INPUT_2, 64; "test input 2")]
    #[test_case(INPUT, 545; "input")]
    fn test_part2(input: &str, best_seats: usize) {
        assert_eq!(best_seats, part2(input));
    }
}
