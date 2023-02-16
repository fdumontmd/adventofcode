use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    fmt::Display,
};

use anyhow::{bail, Error};
use aoc_utils::grid::{Distance, Grid, Taxicab};

struct Risk(u8);

impl Risk {
    fn risk_level(&self) -> u64 {
        self.0.into()
    }

    fn boost(&self, increase: usize) -> Self {
        let level = ((self.0 - 1 + increase as u8) % 9) + 1;
        Self(level)
    }
}

impl TryFrom<u8> for Risk {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() {
            Ok(Risk(value - b'0'))
        } else {
            bail!("Unknown risk level {}", value as char)
        }
    }
}

impl Display for Risk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn part_1(input: &str) -> Result<u64, Error> {
    let grid: Grid<Risk, Taxicab> = Grid::try_from(input)?;

    find_lowest_risk_path(grid)
}

fn find_lowest_risk_path(grid: Grid<Risk, Taxicab>) -> Result<u64, Error> {
    let dest = (grid.width() - 1, grid.height() - 1);

    let mut queue = BinaryHeap::new();

    queue.push((
        Reverse(0u64),
        Reverse(Taxicab::distance(grid.idx_to_pos(0), dest)),
        0,
    ));

    let mut best = BTreeMap::new();

    while let Some((total_risk, _, idx)) = queue.pop() {
        let pos = grid.idx_to_pos(idx);
        if pos == dest {
            return Ok(total_risk.0);
        }

        let best_so_far = best.get(&idx).unwrap_or(&u64::MAX);
        if *best_so_far > total_risk.0 {
            best.insert(idx, total_risk.0);

            for n in grid.neighbours(pos) {
                let nidx = grid.pos_to_idx(n);
                queue.push((
                    Reverse(total_risk.0 + grid[n].risk_level()),
                    Reverse(Taxicab::distance(n, dest)),
                    nidx,
                ));
            }
        }
    }

    bail!("could not find a path?")
}

fn part_2(input: &str) -> Result<u64, Error> {
    let grid: Grid<Risk, Taxicab> = Grid::try_from(input)?;

    let width = grid.width() * 5;
    let height = grid.height() * 5;

    let init = vec![b'0'; width]
        .into_iter()
        .chain(vec![b'\n'].into_iter())
        .cycle()
        .take(width * height + height)
        .collect();
    let init = String::from_utf8(init)?;
    let mut large_grid: Grid<Risk, Taxicab> = Grid::try_from(init.as_ref())?;

    for my in 0..5 {
        for mx in 0..5 {
            for y in 0..grid.height() {
                for x in 0..grid.width() {
                    large_grid[(x + mx * grid.width(), y + my * grid.height())] =
                        grid[(x, y)].boost(mx + my);
                }
            }
        }
    }

    find_lowest_risk_path(large_grid)
}

static INPUT: &str = include_str!("input.txt");
fn main() -> Result<(), Error> {
    println!("Part 1: {}", part_1(INPUT)?);
    println!("Part 2: {}", part_2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};
    use test_case::test_case;

    static TEST_INPUT: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test_case(TEST_INPUT, 40)]
    #[test_case(crate::INPUT, 755)]
    fn test_part_1(input: &str, total_risk: u64) {
        assert_eq!(total_risk, part_1(input).unwrap());
    }

    #[test_case(TEST_INPUT, 315)]
    #[test_case(crate::INPUT, 3016)]
    fn test_part_2(input: &str, total_risk: u64) {
        assert_eq!(total_risk, part_2(input).unwrap());
    }
}
