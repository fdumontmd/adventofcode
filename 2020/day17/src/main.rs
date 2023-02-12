use itertools::{Itertools, MinMaxResult};
use std::{collections::BTreeSet, fmt::Display};

static INPUT: &str = include_str!("input.txt");

trait Coordinate: Sized + Copy + Clone + Eq + PartialEq + Ord + PartialOrd {
    fn neighbours(&self) -> Vec<Self>;
    fn new(col: usize, row: usize) -> Self;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Pos3D(isize, isize, isize);

impl Coordinate for Pos3D {
    fn neighbours(&self) -> Vec<Self> {
        let cur_pos = *self;
        let Pos3D(x, y, z) = *self;
        (-1..2)
            .flat_map(move |dx| {
                (-1..2)
                    .flat_map(move |dy| (-1..2).map(move |dz| Pos3D(x + dx, y + dy, z + dz)))
                    .filter(move |p| p != &cur_pos)
            })
            .collect()
    }
    fn new(col: usize, row: usize) -> Self {
        Pos3D(col as isize, row as isize, 0)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Pos4D(isize, isize, isize, isize);

impl Coordinate for Pos4D {
    fn neighbours(&self) -> Vec<Self> {
        let cur_pos = *self;
        let Pos4D(x, y, z, w) = *self;
        (-1..2)
            .flat_map(move |dw| {
                (-1..2).flat_map(move |dx| {
                    (-1..2)
                        .flat_map(move |dy| {
                            (-1..2).map(move |dz| Pos4D(x + dx, y + dy, z + dz, w + dw))
                        })
                        .filter(move |p| p != &cur_pos)
                })
            })
            .collect()
    }
    fn new(col: usize, row: usize) -> Self {
        Pos4D(col as isize, row as isize, 0, 0)
    }
}

struct ConwayCube<P: Coordinate>(BTreeSet<P>);

impl<P: Coordinate> ConwayCube<P> {
    fn parse_str(input: &str) -> Self {
        let mut conway_cube = BTreeSet::new();
        input.lines().enumerate().for_each(|(row, line)| {
            line.bytes().enumerate().for_each(|(col, b)| {
                if b == b'#' {
                    conway_cube.insert(P::new(col, row));
                }
            })
        });

        ConwayCube(conway_cube)
    }

    fn count_active_neighbours(&self, pos: P) -> usize {
        pos.neighbours()
            .into_iter()
            .filter(|p| self.0.contains(p))
            .count()
    }

    fn all_neighbours(&self) -> BTreeSet<P> {
        let mut neighbours = BTreeSet::new();
        for p in &self.0 {
            neighbours.extend(p.neighbours());
        }
        neighbours
    }

    fn cycle(&mut self) {
        let neighbours = self.all_neighbours();

        let mut conway_cube = BTreeSet::new();

        for p in neighbours {
            let count = self.count_active_neighbours(p);
            if self.0.contains(&p) {
                if count == 2 || count == 3 {
                    conway_cube.insert(p);
                }
            } else if count == 3 {
                conway_cube.insert(p);
            }
        }

        self.0 = conway_cube;
    }

    fn count_active(&self) -> usize {
        self.0.len()
    }
}

impl ConwayCube<Pos3D> {
    fn bounding_box(&self) -> ((isize, isize), (isize, isize), (isize, isize)) {
        let x_bounds = match self.0.iter().map(|p| p.0).minmax() {
            MinMaxResult::NoElements => panic!("no x dimension"),
            MinMaxResult::OneElement(x) => (x, x),
            MinMaxResult::MinMax(min, max) => (min, max),
        };

        let y_bounds = match self.0.iter().map(|p| p.1).minmax() {
            MinMaxResult::NoElements => panic!("no x dimension"),
            MinMaxResult::OneElement(y) => (y, y),
            MinMaxResult::MinMax(min, max) => (min, max),
        };

        let z_bounds = match self.0.iter().map(|p| p.2).minmax() {
            MinMaxResult::NoElements => panic!("no x dimension"),
            MinMaxResult::OneElement(z) => (z, z),
            MinMaxResult::MinMax(min, max) => (min, max),
        };

        (x_bounds, y_bounds, z_bounds)
    }
}

impl Display for ConwayCube<Pos3D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounding_box();

        for z in bounds.2 .0..=bounds.2 .1 {
            writeln!(f, "z={z}")?;
            for y in bounds.1 .0..=bounds.1 .1 {
                for x in bounds.0 .0..=bounds.0 .1 {
                    let c = if self.0.contains(&Pos3D(x, y, z)) {
                        '#'
                    } else {
                        '.'
                    };
                    write!(f, "{c}")?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn part_1(input: &str) -> usize {
    let mut conway_cube: ConwayCube<Pos3D> = ConwayCube::parse_str(input);
    println!("Before any cycles:\n\n{conway_cube}");
    for c in 1..=6 {
        conway_cube.cycle();
        println!(
            "After {c} cycle{}:\n\n{conway_cube}",
            if c > 1 { "s" } else { "" }
        );
    }
    conway_cube.count_active()
}

fn part_2(input: &str) -> usize {
    let mut conway_cube: ConwayCube<Pos4D> = ConwayCube::parse_str(input);
    for _ in 1..=6 {
        conway_cube.cycle();
    }
    conway_cube.count_active()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r".#.
..#
###";

    #[test]
    fn test_part_1() {
        assert_eq!(112, part_1(TEST_INPUT));
        assert_eq!(426, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(848, part_2(TEST_INPUT));
        assert_eq!(1892, part_2(INPUT));
    }
}
