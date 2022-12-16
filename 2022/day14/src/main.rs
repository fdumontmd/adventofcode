use std::{collections::HashMap, fmt::Display, ops::Index};

use itertools::{Itertools, MinMaxResult::MinMax};

static INPUT: &str = include_str!("input.txt");

type Coord = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Rock,
    Sand,
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Settle((Coord, Coord)),
    FallOff,
    Blocked,
}

#[derive(Debug, Clone)]
struct Cave {
    cells: HashMap<(Coord, Coord), Cell>,
    depth: Coord,
    floor: Option<Coord>,
}

impl Cave {
    fn parse(input: &str) -> Self {
        let mut cells = HashMap::new();
        let mut depth = 0;
        input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .for_each(|l| {
                let pairs: Vec<(Coord, Coord)> = l
                    .split(" -> ")
                    .map(|p| {
                        if let Some((x, y)) = p.split(',').tuples().next() {
                            (x.parse().unwrap(), y.parse().unwrap())
                        } else {
                            panic!("cannot parse pair: {}", p);
                        }
                    })
                    .collect();
                for segment in pairs.windows(2) {
                    let p1 = segment[0];
                    let p2 = segment[1];

                    if p1.0 == p2.0 {
                        let from = p1.1.min(p2.1);
                        let to = p1.1.max(p2.1);
                        for y in from..=to {
                            cells.insert((p1.0, y), Cell::Rock);
                        }
                    } else if p1.1 == p2.1 {
                        let from = p1.0.min(p2.0);
                        let to = p1.0.max(p2.0);
                        for x in from..=to {
                            cells.insert((x, p1.1), Cell::Rock);
                        }
                    } else {
                        panic!("need to handle diagonals!");
                    }

                    depth = depth.max(p1.1);
                    depth = depth.max(p2.1);
                }
            });

        Cave {
            cells,
            depth,
            floor: None,
        }
    }

    fn drop_sand_grain(&mut self, mut rock_pos: (Coord, Coord)) -> Outcome {
        if *self.index(rock_pos) != Cell::Empty {
            return Outcome::Blocked;
        }
        'drop_lower: while rock_pos.1 <= self.depth {
            for delta in [(0, 1), (-1, 1), (1, 1)].into_iter() {
                let new_pos = (rock_pos.0 + delta.0, rock_pos.1 + delta.1);
                if *self.index(new_pos) == Cell::Empty {
                    rock_pos = new_pos;
                    continue 'drop_lower;
                }
            }
            self.cells.insert(rock_pos, Cell::Sand);
            return Outcome::Settle(rock_pos);
        }

        Outcome::FallOff
    }

    fn drop_sand(&mut self) -> usize {
        for grains in 0.. {
            if let Outcome::Settle(_) = self.drop_sand_grain((500, 0)) {
                continue;
            }
            return grains;
        }
        0
    }

    fn add_floor(&mut self) {
        let y_max = self.cells.keys().map(|(_, y)| *y).max().unwrap();
        self.floor = Some(y_max + 2);
        self.depth = y_max + 3;
    }
}

impl Index<(Coord, Coord)> for Cave {
    type Output = Cell;

    fn index(&self, index: (Coord, Coord)) -> &Self::Output {
        if let Some(cell) = self.cells.get(&index) {
            cell
        } else if Some(index.1) == self.floor {
            &Cell::Rock
        } else {
            &Cell::Empty
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let MinMax(x_min, x_max) = self.cells.keys().map(|(x, _)| *x).minmax() {
            if let MinMax(_, y_max) = self.cells.keys().map(|(_, y)| *y).minmax() {
                let y_max = y_max.max(self.floor.unwrap_or(0));
                for y in 0..=y_max {
                    for x in x_min..=x_max {
                        match self.index((x, y)) {
                            Cell::Rock => write!(f, "#")?,
                            Cell::Sand => write!(f, "o")?,
                            Cell::Empty => write!(f, ".")?,
                        };
                    }
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut cave = Cave::parse(INPUT);
    println!("Part 1: {}", cave.clone().drop_sand());
    cave.add_floor();

    println!("Part 2: {}", cave.drop_sand());
}

#[cfg(test)]
mod test {
    use crate::{Cave, INPUT};

    static TEST_INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part_01() {
        let mut cave = Cave::parse(TEST_INPUT);
        assert_eq!(24, cave.drop_sand());
    }

    #[test]
    fn real_part_01() {
        let mut cave = Cave::parse(INPUT);
        assert_eq!(745, cave.drop_sand());
    }

    #[test]
    fn test_part_02() {
        let mut cave = Cave::parse(TEST_INPUT);
        cave.add_floor();
        let grains = cave.drop_sand();
        println!("{}", cave);
        assert_eq!(93, grains);
    }

    #[test]
    fn real_part_02() {
        let mut cave = Cave::parse(INPUT);
        cave.add_floor();
        let grains = cave.drop_sand();
        println!("{}", cave);
        assert_eq!(27551, grains);
    }
}
