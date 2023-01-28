use log::info;
use log::warn;
use std::{cmp::Reverse, fmt::Display};

use lazy_static::lazy_static;
use regex::Regex;

static INPUT: &str = include_str!("input.txt");

lazy_static! {
    static ref GROUP_REGEX: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (\((.+)\) )?with an attack that does (\d+) (.+) damage at initiative (\d+)").unwrap();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Side {
    ImmuneSystem,
    Infection,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Side::ImmuneSystem => "Immune System",
                Side::Infection => "Infection",
            }
        )
    }
}

#[derive(Debug)]
struct Group {
    id: u8,
    side: Side,
    unit: u64,
    hit_point: u64,
    immunity: Vec<String>,
    weakness: Vec<String>,
    damage: u64,
    damage_kind: String,
    initiative: u64,
}

impl Group {
    fn from_str(input: &str, id: u8, side: Side) -> Self {
        let group = GROUP_REGEX.captures(input).unwrap();

        let unit = group[1].parse().unwrap();
        let hit_point = group[2].parse().unwrap();

        let mut immunity = vec![];
        let mut weakness = vec![];

        if let Some(imwe) = group.get(4) {
            for s in imwe.as_str().split("; ") {
                if let Some(w) = s.strip_prefix("weak to ") {
                    weakness.extend(w.split(", ").map(|s| s.to_string()));
                } else if let Some(i) = s.strip_prefix("immune to ") {
                    immunity.extend(i.split(", ").map(|s| s.to_string()));
                } else {
                    panic!("cannot parse {s}");
                }
            }
        }

        let damage = group[5].parse().unwrap();
        let damage_kind = group[6].to_string();
        let initiative = group[7].parse().unwrap();

        Self {
            id,
            side,
            unit,
            hit_point,
            immunity,
            weakness,
            damage,
            damage_kind,
            initiative,
        }
    }

    fn effective_power(&self) -> u64 {
        self.unit * self.damage
    }

    fn effective_damage(&self, other: &Group) -> u64 {
        assert_ne!(self.side, other.side);
        if other.immunity.contains(&self.damage_kind) {
            0
        } else {
            let base_damage = self.effective_power();
            if other.weakness.contains(&self.damage_kind) {
                2 * base_damage
            } else {
                base_damage
            }
        }
    }

    fn apply_damage(&mut self, damage: u64) {
        self.unit = self.unit.saturating_sub(damage / self.hit_point)
    }

    fn is_dead(&self) -> bool {
        self.unit == 0
    }
}

struct Armies {
    groups: Vec<Group>,
    stuck: bool,
}

impl Armies {
    fn from_str(input: &str) -> Self {
        // immune system group id
        let mut isgi = 0;
        // infection group id
        let mut igi = 0;
        let mut gi = &mut isgi;
        let mut side = Side::ImmuneSystem;
        let mut groups = vec![];
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line == "Immune System:" {
                side = Side::ImmuneSystem;
                gi = &mut isgi;
            } else if line == "Infection:" {
                side = Side::Infection;
                gi = &mut igi;
            } else {
                *gi += 1;
                groups.push(Group::from_str(line, *gi, side));
            }
        }

        Self {
            groups,
            stuck: false,
        }
    }

    fn boost(&mut self, booster: u64) {
        self.groups
            .iter_mut()
            .filter(|g| g.side == Side::ImmuneSystem)
            .for_each(|g| g.damage += booster);
    }

    fn target_selection(&self) -> Vec<(usize, usize)> {
        let mut targets = vec![];

        let mut attackers: Vec<usize> = (0..self.groups.len()).collect();
        attackers.sort_by_key(|&i| {
            (
                Reverse(self.groups[i].effective_power()),
                Reverse(self.groups[i].initiative),
            )
        });

        let mut remaining_targets = Vec::from_iter(0..self.groups.len());

        for a in &attackers {
            let g = &self.groups[*a];
            let mut possible_targets: Vec<_> = (0..self.groups.len())
                .filter(|i| {
                    remaining_targets.contains(i)
                        && self.groups[*i].side != g.side
                        && g.effective_damage(&self.groups[*i]) > 0
                })
                .collect();

            if possible_targets.is_empty() {
                continue;
            }

            possible_targets.sort_by_key(|i| {
                (
                    Reverse(g.effective_damage(&self.groups[*i])),
                    Reverse(self.groups[*i].effective_power()),
                    Reverse(self.groups[*i].initiative),
                )
            });

            info!(
                "{} group {} would deal defending group {} {} damage",
                g.side,
                g.id,
                self.groups[possible_targets[0]].id,
                g.effective_damage(&self.groups[possible_targets[0]])
            );
            targets.push((*a, possible_targets[0]));
            remaining_targets.retain(|rt| *rt != possible_targets[0]);
        }

        targets
    }

    fn attack(&mut self, mut targets: Vec<(usize, usize)>) -> bool {
        targets.sort_by_key(|(a, _)| Reverse(self.groups[*a].initiative));
        let mut damage_dealt = false;

        for (a, d) in targets {
            if self.groups[a].is_dead() {
                warn!(
                    "{} group {} already dead",
                    self.groups[a].side, self.groups[a].id
                );
            } else {
                let (cu, ed) = {
                    let ag = &self.groups[a];
                    let dg = &self.groups[d];
                    (dg.unit, ag.effective_damage(dg))
                };
                self.groups[d].apply_damage(ed);
                let ag = &self.groups[a];
                let dg = &self.groups[d];
                damage_dealt = damage_dealt || cu > dg.unit;
                info!(
                    "{} group {} attacks defending group {}, killing {}",
                    ag.side,
                    ag.id,
                    dg.id,
                    cu - dg.unit
                );
            }
        }

        self.groups.retain(|g| !g.is_dead());
        damage_dealt
    }

    fn round(&mut self) -> bool {
        info!("Immune System:");
        self.groups
            .iter()
            .filter(|g| g.side == Side::ImmuneSystem)
            .for_each(|g| info!("Group {} contains {} units", g.id, g.unit));
        info!("Infection:");
        self.groups
            .iter()
            .filter(|g| g.side == Side::Infection)
            .for_each(|g| info!("Group {} contains {} units", g.id, g.unit));
        let targets = self.target_selection();

        if !self.attack(targets) {
            warn!("armies are stuck");
            self.stuck = true;
            return true;
        }

        self.groups.iter().all(|g| g.side == Side::ImmuneSystem)
            || self.groups.iter().all(|g| g.side == Side::Infection)
    }

    fn fight(&mut self) {
        while !self.round() {}
    }

    fn remaining_units(&self) -> u64 {
        self.groups.iter().map(|g| g.unit).sum()
    }

    fn remaining_side(&self) -> Option<Side> {
        let mut sides: Vec<_> = self.groups.iter().map(|g| g.side).collect();
        sides.sort();
        sides.dedup();
        if sides.len() == 1 {
            Some(sides[0])
        } else {
            None
        }
    }
}

fn part_01(input: &str) -> u64 {
    let mut armies = Armies::from_str(input);
    armies.fight();
    if armies.remaining_side().is_some() {
        armies.remaining_units()
    } else {
        panic!("fight does not resolve")
    }
}

fn part_02(input: &str) -> u64 {
    let mut min = 0;
    let mut max = 10000000000;
    loop {
        if min == max {
            panic!("cannot find booster in range");
        }
        let booster = min + (max - min) / 2;

        let mut armies = Armies::from_str(input);
        armies.boost(booster);
        armies.fight();
        if armies.stuck {
            min += 1;
        } else if let Some(Side::ImmuneSystem) = armies.remaining_side() {
            // yes, I could implement binary search correctly, or, I can just
            // scan and be done with it
            if max - min < 5 {
                for booster in min..=max {
                    let mut armies = Armies::from_str(input);
                    armies.boost(booster);
                    armies.fight();
                    if armies.stuck {
                        continue;
                    }
                    if let Some(Side::ImmuneSystem) = armies.remaining_side() {
                        return armies.remaining_units();
                    }
                }
            }
            max = booster + 1;
        } else {
            min = booster + 1;
        }
    }
}

fn main() {
    env_logger::init();
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_01, part_02, INPUT};
    use test_case::test_case;

    static TEST_INPUT: &str = r"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test_case(TEST_INPUT, 5216)]
    #[test_case(INPUT, 18280)]
    fn test_part_01(input: &str, units: u64) {
        assert_eq!(units, part_01(input));
    }

    #[test_case(TEST_INPUT, 51)]
    #[test_case(INPUT, 4573)]
    fn test_part_02(input: &str, units: u64) {
        assert_eq!(units, part_02(input));
    }
}
