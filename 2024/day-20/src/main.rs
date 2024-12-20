use std::collections::{HashMap, HashSet, VecDeque};

use aoc_utils::grid::{Distance, Grid, Taxicab};

const INPUT: &str = include_str!("input.txt");

enum Tile {
    Track,
    Wall,
    Start,
    End,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Track,
            b'#' => Tile::Wall,
            b'S' => Tile::Start,
            _ => Tile::End,
        }
    }
}

type Pos = (usize, usize);

impl From<&Tile> for usize {
    fn from(_value: &Tile) -> Self {
        0
    }
}

impl From<&Tile> for bool {
    fn from(_value: &Tile) -> Self {
        false
    }
}

fn solve_with_cheating(
    grid: &Grid<Tile, Taxicab>,
    start: Pos,
    end: Pos,
    cutoff: usize,
    distance: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front((end, 0));
    let mut seen: Grid<bool, Taxicab> = grid.into();

    let mut distance_from_end: Grid<usize, Taxicab> = grid.into();

    while let Some((pos, steps)) = queue.pop_back() {
        seen[pos] = true;
        distance_from_end[pos] = steps;

        for n in grid.neighbours(pos) {
            if seen[n] {
                continue;
            }
            if !matches!(grid[n], Tile::Wall) {
                queue.push_front((n, steps + 1));
            }
        }
    }
    let min_path = distance_from_end[start];

    queue.push_front((start, 0));

    seen.iter_mut().for_each(|s| {
        *s = false;
    });

    let mut best_paths = HashMap::new();

    while let Some((pos, steps)) = queue.pop_back() {
        seen[pos] = true;

        for n in grid.neighbours(pos) {
            if seen[n] {
                continue;
            }
            if !matches!(grid[n], Tile::Wall) && distance_from_end[n] == (min_path - steps - 1) {
                queue.push_front((n, steps + 1));
            }
        }
        // don't need to just start at walls, so don't need to iterate over neighbours
        // now we'll need to check neighbours at distance of at most distance
        let bound_x = (
            pos.0.saturating_sub(distance + 1),
            (pos.0 + distance + 1).min(grid.width() - 1),
        );
        let bound_y = (
            pos.1.saturating_sub(distance + 1),
            (pos.1 + distance + 1).min(grid.height() - 1),
        );
        for y in bound_y.0..=bound_y.1 {
            for x in bound_x.0..=bound_x.1 {
                let t = (x, y);
                if Taxicab::distance(pos, t) > distance {
                    continue;
                }
                if !matches!(grid[t], Tile::Wall)
                    && steps + Taxicab::distance(pos, t) + distance_from_end[t] <= min_path - cutoff
                {
                    best_paths
                        .entry(
                            min_path - (steps + Taxicab::distance(pos, t) + distance_from_end[t]),
                        )
                        .or_insert(HashSet::new())
                        .insert((pos, t));
                }
            }
        }
    }

    best_paths.values().map(|bs| bs.len()).sum()
}

fn part1(input: &str, cutoff: usize) -> usize {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    let start_idx = grid.iter().position(|t| matches!(t, Tile::Start)).unwrap();
    let start = grid.idx_to_pos(start_idx);
    let end_idx = grid.iter().position(|t| matches!(t, Tile::End)).unwrap();
    let end = grid.idx_to_pos(end_idx);

    solve_with_cheating(&grid, start, end, cutoff, 2)
}

fn part2(input: &str, cutoff: usize) -> usize {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    let start_idx = grid.iter().position(|t| matches!(t, Tile::Start)).unwrap();
    let start = grid.idx_to_pos(start_idx);
    let end_idx = grid.iter().position(|t| matches!(t, Tile::End)).unwrap();
    let end = grid.idx_to_pos(end_idx);

    solve_with_cheating(&grid, start, end, cutoff, 20)
}

fn main() {
    println!("part 1: {}", part1(INPUT, 100));
    println!("part 2: {}", part2(INPUT, 100));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test_case(TEST_INPUT, 44, 2; "test input")]
    #[test_case(INPUT, 1293, 100; "input")]
    fn test_part1(input: &str, better_paths: usize, cutoff: usize) {
        assert_eq!(better_paths, part1(input, cutoff));
    }

    #[test_case(TEST_INPUT, 285, 50; "test input")]
    #[test_case(INPUT, 977747, 100; "input")]
    fn test_part2(input: &str, better_paths: usize, cutoff: usize) {
        assert_eq!(better_paths, part2(input, cutoff));
    }
}
