use std::collections::HashMap;

use aoc_utils::grid::{Grid, Taxicab};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let mut grid: Grid<u8, Taxicab> = Grid::try_from(input).unwrap();

    let mut seen = HashMap::new();

    seen.insert(grid.clone(), 0);
    for idx in 1.. {
        cycle(&mut grid);

        if let Some(prev) = seen.get(&grid) {
            let cycle_len = idx - prev;

            let remainder = (1000000000usize - prev).rem_euclid(cycle_len);

            for _ in 0..remainder {
                cycle(&mut grid);
            }

            let load = load(&grid);

            return Ok(format!("{load}"));
        }
        seen.insert(grid.clone(), idx);
    }
    unreachable!()
}

fn cycle(grid: &mut Grid<u8, Taxicab>) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn roll_north(grid: &mut Grid<u8, Taxicab>) {
    for c in 0..grid.width() {
        let mut empty = 0;
        for r in 0..grid.height() {
            match grid[(c, r)] {
                b'#' => empty = r + 1,
                b'O' => {
                    // need to swap
                    grid[(c, r)] = b'.';
                    grid[(c, empty)] = b'O';
                    empty += 1;
                }
                _ => {}
            }
        }
    }
}

fn roll_south(grid: &mut Grid<u8, Taxicab>) {
    for c in 0..grid.width() {
        let mut empty = (grid.height() - 1) as isize;
        for r in (0..grid.height()).rev() {
            match grid[(c, r)] {
                b'#' => empty = r as isize - 1,
                b'O' => {
                    grid[(c, r)] = b'.';
                    grid[(c, empty as usize)] = b'O';
                    empty -= 1;
                }
                _ => {}
            }
        }
    }
}

fn roll_west(grid: &mut Grid<u8, Taxicab>) {
    for r in 0..grid.height() {
        let mut empty = 0;
        for c in 0..grid.width() {
            match grid[(c, r)] {
                b'#' => empty = c + 1,
                b'O' => {
                    grid[(c, r)] = b'.';
                    grid[(empty, r)] = b'O';
                    empty += 1;
                }
                _ => {}
            }
        }
    }
}

fn roll_east(grid: &mut Grid<u8, Taxicab>) {
    for r in 0..grid.height() {
        let mut empty = (grid.width() - 1) as isize;
        for c in (0..grid.width()).rev() {
            match grid[(c, r)] {
                b'#' => empty = c as isize - 1,
                b'O' => {
                    grid[(c, r)] = b'.';
                    grid[(empty as usize, r)] = b'O';
                    empty -= 1;
                }
                _ => {}
            }
        }
    }
}

fn load(grid: &Grid<u8, Taxicab>) -> usize {
    let mut load = 0;

    for c in 0..grid.width() {
        // for each column, iterate from the first (North) row
        for r in 0..grid.height() {
            if grid[(c, r)] == b'O' {
                load += grid.height() - r;
            }
        }
    }

    load
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[rstest]
    #[case(INPUT, "64")]
    #[case(include_str!("../input.txt"), "99118")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }

    #[test]
    fn test_roll_north() {
        let mut grid = Grid::<u8, Taxicab>::try_from(INPUT).unwrap();
        roll_north(&mut grid);

        assert_eq!(136, load(&grid));
    }
}
