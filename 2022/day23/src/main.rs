use itertools::Itertools;
use itertools::MinMaxResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Display;

static INPUT: &str = include_str!("input.txt");

enum Directions {
    North,
    South,
    West,
    East,
}

static DIRECTIONS: [Directions; 4] = [
    Directions::North,
    Directions::South,
    Directions::West,
    Directions::East,
];

impl Directions {
    fn adjacent_positions(&self, pos: &(isize, isize)) -> [(isize, isize); 3] {
        match self {
            Directions::North => [
                (pos.0 - 1, pos.1 - 1),
                (pos.0, pos.1 - 1),
                (pos.0 + 1, pos.1 - 1),
            ],
            Directions::South => [
                (pos.0 - 1, pos.1 + 1),
                (pos.0, pos.1 + 1),
                (pos.0 + 1, pos.1 + 1),
            ],
            Directions::West => [
                (pos.0 - 1, pos.1 - 1),
                (pos.0 - 1, pos.1),
                (pos.0 - 1, pos.1 + 1),
            ],
            Directions::East => [
                (pos.0 + 1, pos.1 - 1),
                (pos.0 + 1, pos.1),
                (pos.0 + 1, pos.1 + 1),
            ],
        }
    }
}

fn pos_all_around(pos: &(isize, isize)) -> [(isize, isize); 12] {
    [
        (pos.0 - 1, pos.1 - 1),
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
    ]
}

struct Elves {
    elves: BTreeSet<(isize, isize)>,
}

impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (cols, rows) = self.boundaries();
        for r in rows.0..=rows.1 {
            for c in cols.0..=cols.1 {
                if self.elves.contains(&(c, r)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Elves {
    fn parse(input: &str) -> Self {
        let mut elves = BTreeSet::new();
        input.lines().enumerate().for_each(|(row, line)| {
            line.bytes().enumerate().for_each(|(col, b)| {
                if b == b'#' {
                    elves.insert((col as isize, row as isize));
                }
            })
        });
        Elves { elves }
    }

    fn round(&mut self, round: usize) -> bool {
        let mut proposed: BTreeMap<(isize, isize), Vec<(isize, isize)>> = BTreeMap::new();

        'elves: for pos in &self.elves {
            let around = pos_all_around(pos);

            if around.iter().any(|t| self.elves.contains(t)) {
                for dir in 0..4 {
                    let base = (round + dir) % 4;
                    let targets = &around[base * 3..(base + 1) * 3];
                    if targets.iter().all(|t| !self.elves.contains(t)) {
                        proposed.entry(targets[1]).or_default().push(*pos);
                        continue 'elves;
                    }
                }
            }

            // stay in place
            proposed.entry(*pos).or_default().push(*pos);
        }

        let mut new_elves = BTreeSet::new();

        for (k, v) in proposed {
            if v.len() == 1 {
                new_elves.insert(k);
            } else {
                new_elves.extend(v.into_iter());
            }
        }

        let has_moved = self.elves != new_elves;
        self.elves = new_elves;
        has_moved
    }

    fn len(&self) -> isize {
        self.elves.len() as isize
    }

    fn dimensions(&self) -> (isize, isize) {
        let (cols, rows) = self.boundaries();
        ((cols.1 - cols.0 + 1), (rows.1 - rows.0 + 1))
    }

    fn boundaries(&self) -> ((isize, isize), (isize, isize)) {
        let cols = self.elves.iter().map(|(c, _)| *c).minmax();
        let rows = self.elves.iter().map(|(_, r)| *r).minmax();

        let cols = match cols {
            MinMaxResult::NoElements => panic!("no elves?"),
            MinMaxResult::OneElement(c) => (c, c),
            MinMaxResult::MinMax(l, h) => (l, h),
        };

        let rows = match rows {
            MinMaxResult::NoElements => panic!("no elves?"),
            MinMaxResult::OneElement(r) => (r, r),
            MinMaxResult::MinMax(l, h) => (l, h),
        };
        (cols, rows)
    }

    fn emptiness(&self) -> isize {
        let (cols, rows) = self.dimensions();
        (cols * rows) - self.len()
    }
}

fn part_01(input: &str) -> isize {
    let mut elves = Elves::parse(input);
    //println!("{}\n\n", elves);
    for idx in 0..10 {
        elves.round(idx);
        //   println!("{}\n\n", elves);
    }
    elves.emptiness()
}

fn part_02(input: &str) -> usize {
    let mut elves = Elves::parse(input);
    //println!("{}\n\n", elves);
    for idx in 0.. {
        if !elves.round(idx) {
            return idx + 1;
        }
        //   println!("{}\n\n", elves);
    }
    0
}
fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02};

    static TEST_INPUT: &str = r"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_part_01() {
        assert_eq!(110, part_01(TEST_INPUT));
    }

    #[test]
    fn test_part_02() {
        assert_eq!(20, part_02(TEST_INPUT));
    }
}
