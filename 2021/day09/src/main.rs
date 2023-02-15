use anyhow::{bail, Error};
use aoc_utils::{
    grid::{Grid, Taxicab},
    union_find::UnionFind,
};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Level(u64);

impl TryFrom<u8> for Level {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() {
            Ok(Level((value - b'0').into()))
        } else {
            bail!("invalid level {}", value as char)
        }
    }
}

fn parse_grid(input: &str) -> Result<Grid<Level, Taxicab>, Error> {
    Grid::try_from(input)
}

fn part_1(input: &str) -> Result<u64, Error> {
    let grid = parse_grid(input)?;

    Ok(grid
        .iter()
        .enumerate()
        .filter_map(|(idx, l)| {
            if grid.neighbours(grid.idx_to_pos(idx)).all(|n| grid[n] > *l) {
                Some(l.0 + 1)
            } else {
                None
            }
        })
        .sum())
}

fn part_2(input: &str) -> Result<usize, Error> {
    let grid = parse_grid(input)?;
    let mut basins = UnionFind::new();
    grid.iter().enumerate().for_each(|(idx, l)| {
        if l.0 != 9 {
            for n in grid.neighbours(grid.idx_to_pos(idx)) {
                if grid[n].0 != 9 {
                    basins.join(idx, grid.pos_to_idx(n));
                }
            }
        }
    });

    let mut basins_sizes: Vec<usize> = basins
        .groups()
        .iter()
        .map(|g| basins.same_group(*g).len())
        .collect();

    basins_sizes.sort();
    basins_sizes.reverse();
    Ok(basins_sizes[0] * basins_sizes[1] * basins_sizes[2])
}

fn main() -> Result<(), Error> {
    println!("Part 1: {}", part_1(INPUT)?);
    println!("Part 2: {}", part_2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part_1() {
        assert_eq!(15, part_1(TEST_INPUT).unwrap());
        assert_eq!(537, part_1(INPUT).unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1134, part_2(TEST_INPUT).unwrap());
        assert_eq!(1142757, part_2(INPUT).unwrap());
    }
}
