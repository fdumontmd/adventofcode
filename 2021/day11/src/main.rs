use colorized::Colors;
use std::{collections::HashSet, fmt::Display};

use anyhow::{bail, Error};
use aoc_utils::grid::{Grid, MaxDist};
static INPUT: &str = include_str!("input.txt");

struct Octopus(u8);

impl TryFrom<u8> for Octopus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() {
            Ok(Octopus(value - b'0'))
        } else {
            bail!("Unknown Octopus level {}", value as char)
        }
    }
}

impl Octopus {
    fn increase_energy(&mut self) {
        self.0 += 1
    }

    fn has_flashed(&mut self) -> bool {
        let res = self.0 > 9;
        if res {
            self.0 = 0;
        }
        res
    }
}

struct OctopusGrid(Grid<Octopus, MaxDist>);

impl Display for OctopusGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, b) in self.0.iter().enumerate() {
            if idx > 0 && idx % self.0.width() == 0 {
                writeln!(f)?;
            }
            if b.0 > 9 || b.0 == 0 {
                write!(
                    f,
                    "{}0{}",
                    Colors::BrightWhiteFg.value(),
                    Colors::Reset.value()
                )?;
            } else {
                write!(f, "{}", b.0)?;
            }
        }
        Ok(())
    }
}

impl OctopusGrid {
    fn step(&mut self) -> usize {
        // increase everything by 1
        self.0.iter_mut().for_each(|o| o.increase_energy());

        let mut flashed = HashSet::new();

        for (idx, o) in self.0.iter_mut().enumerate() {
            if o.has_flashed() {
                flashed.insert(idx);
            }
        }

        let mut to_process: Vec<usize> = flashed.iter().cloned().collect();

        while !to_process.is_empty() {
            let mut tmp = Vec::new();

            for idx in to_process {
                for n in self.0.neighbours(self.0.idx_to_pos(idx)) {
                    let nidx = self.0.pos_to_idx(n);
                    if !flashed.contains(&nidx) {
                        self.0[n].increase_energy();
                        if self.0[n].has_flashed() {
                            flashed.insert(nidx);
                            tmp.push(nidx);
                        }
                    }
                }
            }

            to_process = tmp;
        }

        flashed.len()
    }
}

fn part_1(input: &str) -> Result<usize, Error> {
    let grid: Grid<Octopus, MaxDist> = Grid::try_from(input)?;
    let mut grid = OctopusGrid(grid);
    let mut count = 0;

    for _ in 0..100 {
        count += grid.step();
    }

    Ok(count)
}

fn part_2(input: &str) -> Result<usize, Error> {
    let grid: Grid<Octopus, MaxDist> = Grid::try_from(input)?;
    let len = grid.width() * grid.height();
    let mut grid = OctopusGrid(grid);

    for round in 1.. {
        if grid.step() == len {
            return Ok(round);
        }
    }
    bail!("how did we get here?")
}
fn main() -> Result<(), Error> {
    println!("Part 1: {}", part_1(INPUT)?);
    println!("Part 2: {}", part_2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part_1() {
        assert_eq!(1656, part_1(TEST_INPUT).unwrap());
        assert_eq!(1723, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(195, part_2(TEST_INPUT).unwrap());
        assert_eq!(327, part_2(INPUT).unwrap());
    }
}
