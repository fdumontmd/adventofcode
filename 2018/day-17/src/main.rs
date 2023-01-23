use std::{collections::BTreeMap, fmt::Display, ops::RangeInclusive};

use itertools::{Itertools, MinMaxResult};

use lazy_static::lazy_static;
use regex::Regex;
// basic solution design:
// keep track of the water input:
// - we start with one, we might add more when overflow
// from each water input:
// 1. go down, marking each spot as | running water as we go
// 2. if reached high enough y, stop (y goes up as we go down)
// 3. if spot below is # clay or ~ still water, search left and right for
// first of
//  a. a wall on the same level
//  b. . sand on the spot below
// if both ends have wall, fill range with ~, then redo analysis
// from previous y
// if or both end have open with . below, fill with |, then create
// new source of water on the open spot(s)

static INPUT: &str = include_str!("input.txt");

lazy_static! {
    static ref INPUT_REGEX: Regex = Regex::new(r"^(.)=(\d+), (.)=(\d+)..(\d+)$").unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    RunningWater,
    StillWater,
}

fn parse_input(input: &str) -> BTreeMap<(usize, usize), Tile> {
    BTreeMap::from_iter(
        input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .flat_map(|l| {
                let cap = INPUT_REGEX.captures(l).unwrap();
                let x_dim = &cap[1] == "x";
                let fdp = cap[2].parse().unwrap();
                let sdr: RangeInclusive<usize> =
                    RangeInclusive::new(cap[4].parse().unwrap(), cap[5].parse().unwrap());
                sdr.into_iter().map(move |s| {
                    if x_dim {
                        ((fdp, s), Tile::Wall)
                    } else {
                        ((s, fdp), Tile::Wall)
                    }
                })
            }),
    )
}

struct Reservoir {
    map: BTreeMap<(usize, usize), Tile>,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
}

impl Reservoir {
    fn parse(input: &str) -> Self {
        let map = parse_input(input);

        let MinMaxResult::MinMax(&y_min, &y_max) = map.keys().map(|(_, y)| y).minmax() else {
            panic!("y_min == y_max");
        };

        let MinMaxResult::MinMax(&x_min, &x_max) = map.keys().map(|(x, _)| x).minmax() else {
            panic!("x_min == x_max");
        };

        Self {
            map,
            x_range: RangeInclusive::new(x_min - 1, x_max + 1),
            y_range: RangeInclusive::new(y_min, y_max),
        }
    }

    // looking for ends:
    // - on either side, open if lower level is empty
    // - on either side, could be wall or |. In the latter case, it's like open but no need to
    // process
    fn fill_horizontal(&mut self, (x_start, y_start): (usize, usize)) {
        let mut x_min = None;
        let mut x_max = None;
        let mut open_left = false;
        let mut open_right = false;
        let mut skip_left = false;
        let mut skip_right = false;

        // look for ends of range
        // only two case on each side:
        // - open below, or | -> open_end, with skip if |
        // - wall -> closed end
        for y in (1..=y_start).rev() {
            for x in (*self.x_range.start()..x_start).rev() {
                let below = self.map.get(&(x, y + 1));
                if below.is_none() || below == Some(&Tile::RunningWater) {
                    x_min = Some(x);
                    open_left = true;
                    skip_left = below.is_some();
                    break;
                } else if self.map.get(&(x, y)) == Some(&Tile::Wall) {
                    x_min = Some(x + 1);
                    open_left = false;
                    break;
                }
            }
            for x in x_start + 1..=*self.x_range.end() {
                let below = self.map.get(&(x, y + 1));
                if below.is_none() || below == Some(&Tile::RunningWater) {
                    x_max = Some(x);
                    open_right = true;
                    skip_right = below.is_some();
                    break;
                } else if self.map.get(&(x, y)) == Some(&Tile::Wall) {
                    x_max = Some(x - 1);
                    open_right = false;
                    break;
                }
            }

            let (x_min, x_max) = (x_min.unwrap(), x_max.unwrap());

            if open_left || open_right {
                for x in x_min..=x_max {
                    assert!(self.map.get(&(x, y)) != Some(&Tile::Wall));
                    *self.map.entry((x, y)).or_insert(Tile::RunningWater) = Tile::RunningWater;
                }
                if open_left && !skip_left {
                    self.fill_from((x_min, y));
                }
                if open_right && !skip_right {
                    self.fill_from((x_max, y));
                }
                break;
            } else {
                for x in x_min..=x_max {
                    assert!(self.map.get(&(x, y)) != Some(&Tile::Wall));
                    *self.map.entry((x, y)).or_insert(Tile::StillWater) = Tile::StillWater;
                }
            }
        }
    }

    fn fill_from(&mut self, (x_start, y_start): (usize, usize)) {
        for y in y_start + 1..=*self.y_range.end() {
            if !self.map.contains_key(&(x_start, y)) {
                self.map.insert((x_start, y), Tile::RunningWater);
            } else {
                match self.map[&(x_start, y)] {
                    Tile::RunningWater => {}
                    Tile::Wall | Tile::StillWater => self.fill_horizontal((x_start, y - 1)),
                }
                break;
            }
        }
        // for y in 1..self.y_max, check (x_pos, y);
        // if empty, replace with '|'
        // if not, switch to horizontal expand on the previous line
        // - look for left and right edges
        //   - edge is either an empty tile below, or a wall
        //   - if walls on both side, fill with water, and step back to do edge again
        //   - if one or both end open, fill with |, then use fill_from from the open ends
    }

    fn water_count(&self) -> usize {
        self.map
            .iter()
            .filter(|((x, y), t)| {
                self.x_range.contains(x) && self.y_range.contains(y) && t != &&Tile::Wall
            })
            .count()
    }

    fn water_at_rest_count(&self) -> usize {
        self.map
            .iter()
            .filter(|((x, y), t)| {
                self.x_range.contains(x) && self.y_range.contains(y) && t == &&Tile::StillWater
            })
            .count()
    }
}

impl Display for Reservoir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 1..=*self.y_range.end() + 2 {
            for x in self.x_range.clone() {
                let c = if let Some(&t) = self.map.get(&(x, y)) {
                    match t {
                        Tile::Wall => '#',
                        Tile::RunningWater => '|',
                        Tile::StillWater => '~',
                    }
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut reservoir = Reservoir::parse(INPUT);
    reservoir.fill_from((500, 0));
    println!("part 1: {}", reservoir.water_count());
    println!("part 2: {}", reservoir.water_at_rest_count());
}

#[cfg(test)]
mod tests {
    use crate::Reservoir;
    use test_case::test_case;

    static TEST_INPUT: &str = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    fn part_01(input: &str) -> usize {
        let mut reservoir = Reservoir::parse(input);
        reservoir.fill_from((500, 0));
        reservoir.water_count()
    }

    fn part_02(input: &str) -> usize {
        let mut reservoir = Reservoir::parse(input);
        reservoir.fill_from((500, 0));
        reservoir.water_at_rest_count()
    }

    #[test_case(TEST_INPUT, 57)]
    #[test_case(crate::INPUT, 40879)]
    fn test_01(input: &str, obj: usize) {
        assert_eq!(obj, part_01(input));
    }

    #[test_case(TEST_INPUT, 29)]
    #[test_case(crate::INPUT, 34693)]
    fn test_02(input: &str, obj: usize) {
        assert_eq!(obj, part_02(input));
    }
}
