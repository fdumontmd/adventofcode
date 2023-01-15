use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

// refactor:
// keep unit type into its own enum; then simplify to tile:Unit(UnitType, i32) to simplify hit and
// hitpoints calculations
// attack power a bit adhoc; take a function from unittype to attack power?

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    OpenCavern,
    Goblin(i32),
    Elf(i32),
}

impl Tile {
    fn from_u8(byte: u8) -> anyhow::Result<Tile> {
        match byte {
            b'#' => Ok(Tile::Wall),
            b'.' => Ok(Tile::OpenCavern),
            b'G' => Ok(Tile::Goblin(200)),
            b'E' => Ok(Tile::Elf(200)),
            _ => anyhow::bail!("Unknown character {}", byte),
        }
    }

    fn is_unit(&self) -> bool {
        self != &Tile::Wall && self != &Tile::OpenCavern
    }

    fn is_elf(&self) -> bool {
        matches!(self, &Tile::Elf(_))
    }

    fn is_goblin(&self) -> bool {
        matches!(self, &Tile::Goblin(_))
    }

    fn is_dead(&self) -> bool {
        let hit = match self {
            Tile::Wall | Tile::OpenCavern => 0,
            Tile::Elf(h) => *h,
            Tile::Goblin(h) => *h,
        };
        hit <= 0 && self.is_unit()
    }

    fn hitpoints(&self) -> i32 {
        assert!(self.is_unit());
        match self {
            Tile::Elf(h) => *h,
            Tile::Goblin(h) => *h,
            _ => unreachable!(),
        }
    }

    fn hit(&mut self, power: i32) {
        assert!(self.is_unit());
        *self = match self {
            Tile::Elf(h) => Tile::Elf(*h - power),
            Tile::Goblin(h) => Tile::Goblin(*h - power),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    elf_attack_power: i32,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let tiles = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.trim())
            .flat_map(|l| {
                width = width.max(l.len());
                l.bytes().map(Tile::from_u8)
            })
            .collect::<anyhow::Result<Vec<Tile>>>()?;
        Ok(Self {
            tiles,
            width,
            elf_attack_power: 3,
        })
    }
}

// row, col like unit_positions
impl Index<(usize, usize)> for Map {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.tiles[index.1 + index.0 * self.width]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[index.1 + index.0 * self.width]
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, t) in self.tiles.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            match t {
                Tile::Wall => write!(f, "#")?,
                Tile::OpenCavern => write!(f, ".")?,
                Tile::Goblin(_) => write!(f, "G")?,
                Tile::Elf(_) => write!(f, "E")?,
            }
        }
        Ok(())
    }
}

impl Map {
    // by construction already in the correct order
    // row, col to simplify further sorting if needed
    fn unit_positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.tiles.iter().enumerate().filter_map(|(idx, t)| {
            if t.is_unit() {
                Some((idx / self.width, idx % self.width))
            } else {
                None
            }
        })
    }

    fn adjacent_positions_matching<F>(
        &self,
        pos: (usize, usize),
        f: F,
    ) -> impl Iterator<Item = (usize, usize)> + '_
    where
        F: Fn(Tile) -> bool + 'static,
    {
        const DELTA: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
        // we have walls all around and pos will always be inside, so no need to care about getting
        // out of index
        DELTA.iter().filter_map(move |d| {
            let apos = (
                pos.0.checked_add_signed(d.0).unwrap(),
                pos.1.checked_add_signed(d.1).unwrap(),
            );
            if f(self[apos]) {
                Some(apos)
            } else {
                None
            }
        })
    }

    fn adjacent_open_caverns(
        &self,
        pos: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.adjacent_positions_matching(pos, |t| t == Tile::OpenCavern)
    }

    fn adjacent_units(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.adjacent_positions_matching(pos, |t| t.is_unit())
    }

    fn has_any_elf(&self) -> bool {
        self.unit_positions().any(|p| self[p].is_elf())
    }

    fn elf_count(&self) -> usize {
        self.unit_positions().filter(|p| self[*p].is_elf()).count()
    }

    fn is_fight_finished(&self) -> bool {
        let elves = self.has_any_elf();
        let goblins = self.unit_positions().any(|p| self[p].is_goblin());
        !elves && !goblins
    }

    // return true if the round completed - each unit could act or be killed
    fn round(&mut self) -> bool {
        // get list of unit positions in the right order
        let unit_positions: Vec<_> = self.unit_positions().collect();

        for p in &unit_positions {
            let mut p = *p;
            // is the unit dead already?
            if !self[p].is_unit() {
                continue;
            }
            let targets: Vec<_> = if self[p].is_elf() {
                self.unit_positions()
                    .filter(|p| self[*p].is_goblin())
                    .collect()
            } else {
                self.unit_positions()
                    .filter(|p| self[*p].is_elf())
                    .collect()
            };
            if targets.is_empty() {
                return false;
            }
            // use a temp to avoid spurious borrowing
            // need to sort by hitpoints then position order...
            let au = self.adjacent_units(p).find(|au| targets.contains(au));
            // if not in range, try and move
            if au.is_none() {
                let target_adjacent_positions: BTreeSet<_> = targets
                    .iter()
                    .flat_map(|t| self.adjacent_open_caverns(*t))
                    .collect();
                // for each adjacent_open_caverns pos, compute the minimum number of steps required
                // to reach a target or None if impossible
                // then, move to the adjacent pos that has the minimum number of steps to a target,
                // using pos order to break ties
                let mut dists: Vec<_> = self
                    .adjacent_open_caverns(p)
                    .filter_map(|aocp| {
                        // try and reach a target if possible
                        let mut seen = HashSet::new();
                        let mut boundary = HashSet::new();
                        boundary.insert(aocp);
                        let mut idx: usize = 0;
                        loop {
                            if boundary.is_empty() {
                                return None;
                            }
                            for p in &boundary {
                                if target_adjacent_positions.contains(p) {
                                    return Some((idx, aocp));
                                }
                                seen.insert(*p);
                            }
                            boundary = boundary
                                .into_iter()
                                .flat_map(|p| self.adjacent_open_caverns(p))
                                .filter(|p| !seen.contains(&p))
                                .collect();
                            idx += 1;
                        }
                    })
                    .collect();
                dists.sort(); // sort by dist then pos order
                if let Some(&(_, np)) = dists.get(0) {
                    let unit = self[p];
                    self[p] = Tile::OpenCavern;
                    self[np] = unit;
                    p = np;
                }
            }
            // now try to attack if a target is reachable
            let mut all_adjacent_units: Vec<_> = self
                .adjacent_units(p)
                .filter(|au| targets.contains(au))
                .collect();
            all_adjacent_units.sort_by_key(|&au| (self[au].hitpoints(), au));
            if let Some(&au) = all_adjacent_units.get(0) {
                let attack_power = if self[p].is_elf() {
                    self.elf_attack_power
                } else {
                    3
                };
                self[au].hit(attack_power);
                if self[au].is_dead() {
                    self[au] = Tile::OpenCavern;
                }
            }
        }
        true
    }

    fn fight(&mut self) -> usize {
        let mut r: usize = 0;
        loop {
            if self.is_fight_finished() {
                return r;
            }
            if !self.round() {
                return r;
            }
            r += 1;
        }
    }

    fn fight_for_elves(&mut self) -> Option<usize> {
        let elf_count = self.elf_count();
        let mut r: usize = 0;
        loop {
            if self.is_fight_finished() {
                return Some(r);
            }
            if !self.round() {
                if self.elf_count() < elf_count {
                    return None;
                }
                return Some(r);
            }
            if self.elf_count() < elf_count {
                return None;
            }
            r += 1;
        }
    }

    fn fight_score(&mut self) -> usize {
        let complete_rounds = self.fight();

        let total_hit_points = self.total_hit_points();

        complete_rounds * total_hit_points
    }

    fn total_hit_points(&mut self) -> usize {
        let total_hit_points = self
            .unit_positions()
            .map(|p| self[p].hitpoints() as usize)
            .sum::<usize>();
        total_hit_points
    }
}

fn part_01(input: &str) -> anyhow::Result<usize> {
    let mut map = Map::from_str(input)?;

    Ok(map.fight_score())
}

fn part_02(input: &str) -> anyhow::Result<usize> {
    let map = Map::from_str(input)?;
    for attack_power in 4.. {
        let mut map = map.clone();
        map.elf_attack_power = attack_power;
        if let Some(complete_rounds) = map.fight_for_elves() {
            return Ok(complete_rounds * map.total_hit_points());
        }
    }
    anyhow::bail!("how did we get here?")
}

fn main() -> anyhow::Result<()> {
    println!("Part 1: {}", part_01(INPUT)?);
    println!("Part 2: {}", part_02(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{part_01, part_02, Map, INPUT};

    static TEST_MAP_INPUT: &str = r"#######
#.G.E.#
#E.G.E#
#.G.E.#
#######";

    static TEST_INPUT_01: &str = r"#######   
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    static TEST_INPUT_02: &str = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    static TEST_INPUT_03: &str = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    static TEST_INPUT_04: &str = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    static TEST_INPUT_05: &str = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    static TEST_INPUT_06: &str = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn test_map_parsing() {
        let map = Map::from_str(TEST_MAP_INPUT).unwrap();
        assert_eq!(TEST_MAP_INPUT, format!("{}", map));
    }

    #[test]
    fn test_unit_positions() {
        let map = Map::from_str(TEST_MAP_INPUT).unwrap();
        for p in map.unit_positions() {
            assert!(map[p].is_unit());
        }
    }

    use test_case::test_case;

    #[test_case(TEST_INPUT_01, 27730)]
    #[test_case(TEST_INPUT_02, 36334)]
    #[test_case(TEST_INPUT_03, 39514)]
    #[test_case(TEST_INPUT_04, 27755)]
    #[test_case(TEST_INPUT_05, 28944)]
    #[test_case(TEST_INPUT_06, 18740)]
    #[test_case(INPUT, 198531)]
    fn test_part_01(input: &str, res: usize) {
        assert_eq!(res, part_01(input).unwrap());
    }

    #[test_case(TEST_INPUT_01, 4988, 15)]
    #[test_case(TEST_INPUT_03, 31284, 4)]
    #[test_case(TEST_INPUT_04, 3478, 15)]
    #[test_case(TEST_INPUT_05, 6474, 12)]
    #[test_case(TEST_INPUT_06, 1140, 34)]
    fn test_elf_attack_power_helps(input: &str, res: usize, attack_power: i32) {
        let mut map = Map::from_str(input).unwrap();
        let count_elves = map.elf_count();
        map.elf_attack_power = attack_power;
        assert_eq!(res, map.fight_score());
        assert_eq!(count_elves, map.elf_count());
    }

    #[test_case(TEST_INPUT_01, 4988)]
    #[test_case(TEST_INPUT_03, 31284)]
    #[test_case(TEST_INPUT_04, 3478)]
    #[test_case(TEST_INPUT_05, 6474)]
    #[test_case(TEST_INPUT_06, 1140)]
    #[test_case(INPUT, 90420)]
    fn test_part_02(input: &str, res: usize) {
        assert_eq!(res, part_02(input).unwrap());
    }
}
