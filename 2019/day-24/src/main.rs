use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
};

static INPUT: &str = include_str!("input.txt");

const fn adjacent_indices(idx: usize) -> u32 {
    const fn idx_to_pos(idx: usize) -> (usize, usize) {
        (idx % 5, idx / 5)
    }

    const fn dist(idx1: usize, idx2: usize) -> usize {
        let pos1 = idx_to_pos(idx1);
        let pos2 = idx_to_pos(idx2);
        pos1.0.abs_diff(pos2.0) + (pos1.1.abs_diff(pos2.1))
    }
    let mut res = 0;
    let mut i = 0;
    while i < 25 {
        if dist(idx, i) == 1 {
            res |= 1 << i;
        }
        i += 1;
    }

    res
}

const fn build_adjacent_array<const N: usize>() -> [u32; N] {
    let mut res = [0; N];
    let mut i = 0;
    while i < N {
        res[i] = adjacent_indices(i);
        i += 1;
    }
    res
}

const NEIGHBOURS: [u32; 25] = build_adjacent_array();

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct TinyGrid(u32);

impl TinyGrid {
    fn new() -> Self {
        Self(0)
    }

    fn is_empty(&self) -> bool {
        self.0.count_ones() == 0
    }

    fn from_str(input: &str) -> Self {
        Self(
            input
                .bytes()
                .filter(|b| *b != 10)
                .fold(0, |c, b| c << 1 | (b == b'#') as u32),
        )
    }

    fn count_neighbours(&self, idx: usize) -> u32 {
        (self.0 & NEIGHBOURS[idx]).count_ones()
    }

    fn minute(&self) -> Self {
        let mut c = 0;
        for idx in 0..25 {
            let count = self.count_neighbours(idx);
            let on = if self.0 & (1 << idx) != 0 {
                count == 1
            } else {
                count == 1 || count == 2
            };
            if on {
                c |= 1 << idx;
            }
        }
        Self(c)
    }

    fn biodiversity_rating(&self) -> usize {
        (0..25)
            .into_iter()
            .map(|idx| {
                let on = self.0 & (1 << (24 - idx)) != 0;
                if on {
                    1 << idx
                } else {
                    0
                }
            })
            .sum()
    }
}

impl Display for TinyGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for idx in (0..25).rev() {
            write!(f, "{}", if self.0 & (1 << idx) != 0 { '#' } else { '.' })?;
            if idx % 5 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn part_1(input: &str) -> usize {
    let mut grid = TinyGrid::from_str(input);
    let mut seen = HashSet::new();
    seen.insert(grid);

    loop {
        grid = grid.minute();
        if seen.contains(&grid) {
            return grid.biodiversity_rating();
        }
        seen.insert(grid);
    }
}

struct NestedGrid {
    grids: BTreeMap<isize, TinyGrid>,
}

fn outer_neighbours(idx: usize) -> u32 {
    match idx {
        0 => (1 << 7) | (1 << 11),
        1 | 2 | 3 => 1 << 7,
        4 => (1 << 7) | (1 << 13),
        5 | 10 | 15 => 1 << 11,
        20 => (1 << 11) | (1 << 17),
        21 | 22 | 23 => 1 << 17,
        24 => (1 << 13) | (1 << 17),
        9 | 14 | 19 => 1 << 13,
        _ => 0,
    }
}

fn inner_neighbours(idx: usize) -> u32 {
    match idx {
        7 => (1 << 5) - 1,
        11 => 1 | (1 << 5) | (1 << 10) | (1 << 15) | (1 << 20),
        13 => (1 << 4) | (1 << 9) | (1 << 14) | (1 << 19) | (1 << 24),
        17 => (1 << 20) | (1 << 21) | (1 << 22) | (1 << 23) | (1 << 24),
        _ => 0,
    }
}

impl NestedGrid {
    fn from_str(input: &str) -> Self {
        let grid = TinyGrid::from_str(input);
        let mut grids = BTreeMap::new();
        grids.insert(0, grid);
        Self { grids }
    }

    fn update_grid(&mut self, level: isize, grid: &TinyGrid) -> TinyGrid {
        let mut c = 0;
        for idx in 0..25 {
            // keep bit 12 off
            if idx == 12 {
                continue;
            }
            let mut count = grid.count_neighbours(idx);
            let inner = inner_neighbours(idx);
            let outer = outer_neighbours(idx);
            if inner != 0 {
                count +=
                    (self.grids.entry(level + 1).or_insert(TinyGrid::new()).0 & inner).count_ones();
            }
            if outer != 0 {
                count +=
                    (self.grids.entry(level - 1).or_insert(TinyGrid::new()).0 & outer).count_ones();
            }

            if grid.0 & (1 << idx) != 0 {
                if count == 1 {
                    c |= 1 << idx;
                }
            } else if count == 1 || count == 2 {
                c |= 1 << idx;
            }
        }
        TinyGrid(c)
    }

    fn minute(&mut self) {
        let mut new_grids = BTreeMap::new();

        // add padding; this is enough for one minute
        let min = *self.grids.first_entry().unwrap().key();
        let max = *self.grids.last_entry().unwrap().key();
        self.grids.insert(min - 1, TinyGrid::new());
        self.grids.insert(max + 1, TinyGrid::new());

        let k_vs: Vec<_> = self.grids.iter().map(|(k, v)| (*k, *v)).collect();
        for (k, v) in k_vs {
            new_grids.insert(k, self.update_grid(k, &v));
        }

        while let Some(min) = new_grids.first_entry() {
            if min.get().is_empty() {
                min.remove();
            } else {
                break;
            }
        }

        while let Some(max) = new_grids.last_entry() {
            if max.get().is_empty() {
                max.remove();
            } else {
                break;
            }
        }

        if new_grids.is_empty() {
            new_grids.insert(0, TinyGrid::new());
        }

        self.grids = new_grids
    }

    fn count_bugs(&self) -> u32 {
        self.grids.values().map(|g| g.0.count_ones()).sum()
    }
}

impl Display for NestedGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.grids {
            writeln!(f, "Depth {k}")?;
            writeln!(f, "{v}")?;
        }
        Ok(())
    }
}

fn part_2(input: &str, minutes: usize) -> u32 {
    let mut grid = NestedGrid::from_str(input);
    for _ in 0..minutes {
        grid.minute();
    }
    grid.count_bugs()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT, 200));
}

#[cfg(test)]
mod tests {

    use crate::{part_1, part_2, TinyGrid, INPUT};

    static TEST_INPUT: &str = r"....#
#..#.
#..##
..#..
#....
";

    #[test]
    fn test_biodiversity_rating() {
        assert_eq!(
            2129920,
            TinyGrid::from_str(
                r".....
.....
.....
#....
.#..."
            )
            .biodiversity_rating()
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(2129920, part_1(TEST_INPUT));
        assert_eq!(18842609, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(99, part_2(TEST_INPUT, 10));
        assert_eq!(2059, part_2(INPUT, 200));
    }
}
