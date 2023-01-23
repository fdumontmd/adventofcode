use anyhow::anyhow;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum AcreType {
    Open,
    Tree,
    Lumberyard,
}

impl Display for AcreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            AcreType::Open => '.',
            AcreType::Tree => '|',
            AcreType::Lumberyard => '#',
        };
        write!(f, "{a}")
    }
}

impl TryFrom<char> for AcreType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(AcreType::Open),
            '|' => Ok(AcreType::Tree),
            '#' => Ok(AcreType::Lumberyard),
            _ => Err(anyhow!("Unknown Acre character {}", value)),
        }
    }
}

struct LumberArea {
    map: Vec<AcreType>,
    width: usize,
    height: usize,
}

impl FromStr for LumberArea {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let map = s
            .lines()
            .filter(|l| !l.trim().is_empty())
            .flat_map(|l| {
                width = l.len();
                l.chars().map(AcreType::try_from)
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        let height = map.len() / width;
        Ok(Self { map, width, height })
    }
}

impl Display for LumberArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, &a) in self.map.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{a}")?;
        }
        writeln!(f)
    }
}

impl Index<(usize, usize)> for LumberArea {
    type Output = AcreType;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.map[index.0 + index.1 * self.width]
    }
}

impl IndexMut<(usize, usize)> for LumberArea {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.map[index.0 + index.1 * self.width]
    }
}

impl LumberArea {
    fn would_pos_change(&self, index: (usize, usize)) -> Option<((usize, usize), AcreType)> {
        let mut trees = 0;
        let mut lumberyards = 0;
        for y in index.1.saturating_add_signed(-1)..self.height.min(index.1 + 2) {
            for x in index.0.saturating_add_signed(-1)..self.width.min(index.0 + 2) {
                if (x, y) != index {
                    match self[(x, y)] {
                        AcreType::Open => {}
                        AcreType::Tree => trees += 1,
                        AcreType::Lumberyard => lumberyards += 1,
                    }
                }
            }
        }

        match self[index] {
            AcreType::Open => {
                if trees >= 3 {
                    Some((index, AcreType::Tree))
                } else {
                    None
                }
            }
            AcreType::Tree => {
                if lumberyards >= 3 {
                    Some((index, AcreType::Lumberyard))
                } else {
                    None
                }
            }
            AcreType::Lumberyard => {
                if lumberyards >= 1 && trees >= 1 {
                    None
                } else {
                    Some((index, AcreType::Open))
                }
            }
        }
    }

    fn one_minute(&mut self) {
        let changes: Vec<_> = self
            .map
            .iter()
            .enumerate()
            .filter_map(|(idx, _)| self.would_pos_change((idx % self.width, idx / self.width)))
            .collect();

        for (index, a) in changes {
            self[index] = a
        }
    }

    fn score(&self) -> usize {
        let trees = self.map.iter().filter(|a| a == &&AcreType::Tree).count();
        let lumberyards = self
            .map
            .iter()
            .filter(|a| a == &&AcreType::Lumberyard)
            .count();
        trees * lumberyards
    }

    fn cycles(&mut self) -> (usize, usize) {
        let mut seen = HashMap::new();
        seen.insert(self.map.clone(), 0);
        for m in 1.. {
            self.one_minute();
            if let Some(m0) = seen.get(&self.map) {
                return (m, m - m0);
            }
            seen.insert(self.map.clone(), m);
        }
        unreachable!()
    }
}

fn part_01(input: &str) -> usize {
    let mut la = LumberArea::from_str(input).unwrap();
    for _ in 0..10 {
        la.one_minute();
    }
    la.score()
}

fn part_02(input: &str) -> usize {
    let mut la = LumberArea::from_str(input).unwrap();
    let (m, l) = la.cycles();
    let delta = (1000000000 - m) % l;
    for _ in 0..delta {
        la.one_minute();
    }
    la.score()
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_01, part_02, INPUT};
    use test_case::test_case;

    static TEST_INPUT: &str = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test_case(TEST_INPUT, 1147)]
    #[test_case(INPUT, 646437)]
    fn test_01(input: &str, score: usize) {
        assert_eq!(score, part_01(input));
    }

    #[test_case[INPUT, 208080]]
    fn test_02(input: &str, score: usize) {
        assert_eq!(score, part_02(input));
    }
}
