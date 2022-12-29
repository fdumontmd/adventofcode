use lazy_static::lazy_static;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    ops::{Add, Sub},
};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn new() -> Self {
        Resources::default()
    }

    fn dominates(&self, other: &Self) -> bool {
        self.ore.cmp(&other.ore) != Ordering::Less
            && self.clay.cmp(&other.clay) != Ordering::Less
            && self.obsidian.cmp(&other.obsidian) != Ordering::Less
            && self.geode.cmp(&other.geode) != Ordering::Less
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl Blueprint {
    fn parse(line: &str) -> Self {
        lazy_static! {
            static ref PARSER: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

        }

        let cap = PARSER.captures(line).unwrap();
        Blueprint {
            id: cap[1].parse().unwrap(),
            ore_robot: Resources {
                ore: cap[2].parse().unwrap(),
                ..Resources::new()
            },
            clay_robot: Resources {
                ore: cap[3].parse().unwrap(),
                ..Resources::new()
            },
            obsidian_robot: Resources {
                ore: cap[4].parse().unwrap(),
                clay: cap[5].parse().unwrap(),
                ..Resources::new()
            },
            geode_robot: Resources {
                ore: cap[6].parse().unwrap(),
                obsidian: cap[7].parse().unwrap(),
                ..Resources::new()
            },
        }
    }

    fn can_build_ore_robot(&self, stock: &Resources) -> bool {
        stock.dominates(&self.ore_robot)
    }

    fn can_build_clay_robot(&self, stock: &Resources) -> bool {
        stock.dominates(&self.clay_robot)
    }

    fn can_build_obsidian_robot(&self, stock: &Resources) -> bool {
        stock.dominates(&self.obsidian_robot)
    }

    fn can_build_geode_robot(&self, stock: &Resources) -> bool {
        stock.dominates(&self.geode_robot)
    }

    fn max_geode_in(&self, time: usize) -> usize {
        let mut queue = BinaryHeap::new();

        // never need to know how much ore is needed for ore robots; only care
        // about ore for other robots
        let max_ore_needed = self
            .clay_robot
            .ore
            .max(self.obsidian_robot.ore.max(self.geode_robot.ore));

        // (Reverse(time), robots, stock)
        queue.push((
            Reverse(1),
            Resources {
                ore: 1,
                ..Resources::new()
            },
            Resources::new(),
        ));
        let mut max_geode = 0;

        let mut prev_robots = Resources::new();
        let mut prev_stock = Resources::new();
        let mut prev_time = Reverse(1);

        while let Some((t, robots, initial_stock)) = queue.pop() {
            if prev_time != t {
                // reset for new time
                prev_robots = Resources::new();
                prev_stock = Resources::new();
            }
            prev_time = t;
            if prev_robots == robots && prev_stock.dominates(&initial_stock) {
                continue;
            }
            prev_robots = robots;
            prev_stock = initial_stock;

            // compute stock for this minute
            let current_stock = initial_stock + robots;

            max_geode = max_geode.max(current_stock.geode);
            // reached the end
            if t.0 == time {
                continue;
            }

            let next_time = Reverse(t.0 + 1);

            // do nothing
            queue.push((next_time, robots, current_stock));

            // building check is done against stock at begining of minute
            // always build geode robot if possible
            if self.can_build_geode_robot(&initial_stock) {
                queue.push((
                    next_time,
                    Resources {
                        geode: robots.geode + 1,
                        ..robots
                    },
                    current_stock - self.geode_robot,
                ));
            }

            let remaining_time = time - t.0;

            if self.can_build_obsidian_robot(&initial_stock)
                && remaining_time * robots.obsidian + initial_stock.obsidian
                    < self.geode_robot.obsidian * remaining_time
            {
                queue.push((
                    next_time,
                    Resources {
                        obsidian: robots.obsidian + 1,
                        ..robots
                    },
                    current_stock - self.obsidian_robot,
                ));
            }

            if self.can_build_clay_robot(&initial_stock)
                && robots.clay * remaining_time + initial_stock.clay
                    < self.obsidian_robot.clay * remaining_time
            {
                queue.push((
                    next_time,
                    Resources {
                        clay: robots.clay + 1,
                        ..robots
                    },
                    current_stock - self.clay_robot,
                ));
            }

            if self.can_build_ore_robot(&initial_stock)
                && robots.ore * remaining_time + initial_stock.ore < max_ore_needed * remaining_time
            {
                queue.push((
                    next_time,
                    Resources {
                        ore: robots.ore + 1,
                        ..robots
                    },
                    current_stock - self.ore_robot,
                ));
            }
        }

        max_geode
    }

    fn parse_all(input: &str) -> Vec<Blueprint> {
        input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(Blueprint::parse)
            .collect()
    }
}

fn part_01(input: &str) -> usize {
    Blueprint::parse_all(input)
        .into_par_iter()
        .map(|b| b.max_geode_in(24) * b.id)
        .sum()
}

fn part_02(input: &str) -> usize {
    Blueprint::parse_all(input)
        .into_par_iter()
        .take(3)
        .map(|b| b.max_geode_in(32))
        .product()
}
fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::INPUT;
    use crate::{part_01, part_02};

    static TEST_INPUT: &str = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part01() {
        assert_eq!(33, part_01(TEST_INPUT));
    }

    #[test]
    fn test_part02() {
        assert_eq!(56 * 62, part_02(TEST_INPUT));
    }

    #[test]
    fn real_part01() {
        assert_eq!(1009, part_01(INPUT));
    }

    #[test]
    fn real_part02() {
        assert_eq!(18816, part_02(INPUT));
    }
}
