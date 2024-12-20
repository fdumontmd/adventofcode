use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

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

fn minimum_path(grid: &Grid<Tile, Taxicab>, start: Pos, end: Pos) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front((start, 0));
    let mut seen = HashSet::new();

    while let Some((pos, steps)) = queue.pop_back() {
        if pos == end {
            return steps;
        }
        seen.insert(pos);

        for n in grid.neighbours(pos) {
            if seen.contains(&n) {
                continue;
            }
            if !matches!(grid[n], Tile::Wall) {
                queue.push_front((n, steps + 1))
            }
        }
    }

    panic!("exit not found")
}

impl From<&Tile> for usize {
    fn from(_value: &Tile) -> Self {
        0
    }
}

// new idea:
// - precompute distance from each Track to End
//   - actually same as computing distance from End to all Track
// - then compute distance from Start to each Wall
//   - for each Wall, check Track neighbours and add
//     distance from Start to Wall + 1 + distance from Track neighbour to End
//   - if low enough, add 1
fn solve_with_cheating(
    grid: &Grid<Tile, Taxicab>,
    start: Pos,
    end: Pos,
    min_path: usize,
    cutoff: usize,
    distance: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front((end, 0));
    let mut seen = HashSet::new();

    let mut distance_from_end: Grid<usize, Taxicab> = grid.into();

    while let Some((pos, steps)) = queue.pop_back() {
        seen.insert(pos);
        distance_from_end[pos] = steps;

        for n in grid.neighbours(pos) {
            if seen.contains(&n) {
                continue;
            }
            if !matches!(grid[n], Tile::Wall) {
                queue.push_front((n, steps + 1));
            }
        }
    }

    assert_eq!(distance_from_end[start], min_path);

    queue.push_front((start, 0));

    seen.clear();

    let mut best_paths = BTreeMap::new();

    while let Some((pos, steps)) = queue.pop_back() {
        seen.insert(pos);

        for n in grid.neighbours(pos) {
            if seen.contains(&n) {
                continue;
            }
            seen.insert(pos);
            if !matches!(grid[n], Tile::Wall) {
                queue.push_front((n, steps + 1));
            } else {
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
                            && steps + Taxicab::distance(pos, t) + distance_from_end[t]
                                <= min_path - cutoff
                        {
                            best_paths
                                .entry(
                                    min_path
                                        - (steps
                                            + Taxicab::distance(pos, t)
                                            + distance_from_end[t]),
                                )
                                .or_insert(BTreeSet::new())
                                .insert((pos, t));
                        }
                    }
                }
            }
        }
    }

    let best_paths: BTreeMap<usize, usize> = best_paths
        .into_iter()
        .map(|(savings, cheats)| (savings, cheats.len()))
        .collect();

    dbg!(&best_paths);

    best_paths.values().sum()
}

fn part1(input: &str, cutoff: usize) -> usize {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    let start_idx = grid.iter().position(|t| matches!(t, Tile::Start)).unwrap();
    let start = grid.idx_to_pos(start_idx);
    let end_idx = grid.iter().position(|t| matches!(t, Tile::End)).unwrap();
    let end = grid.idx_to_pos(end_idx);

    let minimum_steps = minimum_path(&grid, start, end);

    dbg!(minimum_steps);

    solve_with_cheating(&grid, start, end, minimum_steps, cutoff, 2)
}

fn part2(input: &str, cutoff: usize) -> usize {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    let start_idx = grid.iter().position(|t| matches!(t, Tile::Start)).unwrap();
    let start = grid.idx_to_pos(start_idx);
    let end_idx = grid.iter().position(|t| matches!(t, Tile::End)).unwrap();
    let end = grid.idx_to_pos(end_idx);

    let minimum_steps = minimum_path(&grid, start, end);

    dbg!(minimum_steps);

    solve_with_cheating(&grid, start, end, minimum_steps, cutoff, 20)
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
