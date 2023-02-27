use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
};

use anyhow::{bail, Error};
use aoc_utils::grid::{Grid, Taxicab};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Tile {
    Nothing,
    Empty,
    Wall,
    A,
    B,
    C,
    D,
}

impl Tile {
    fn is_amphipod(&self) -> bool {
        matches!(self, Tile::A | Tile::B | Tile::C | Tile::D)
    }

    fn amphipod_column(&self) -> usize {
        match self {
            Tile::A => 3,
            Tile::B => 5,
            Tile::C => 7,
            Tile::D => 9,
            _ => panic!("Tile {self} is not an amphipod"),
        }
    }

    fn amphipod_energy(&self) -> usize {
        match self {
            Tile::A => 1,
            Tile::B => 10,
            Tile::C => 100,
            Tile::D => 1000,
            _ => panic!("Tile {self} is not an amphipod"),
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b' ' => Tile::Nothing,
            b'.' => Tile::Empty,
            b'#' => Tile::Wall,
            b'A' => Tile::A,
            b'B' => Tile::B,
            b'C' => Tile::C,
            b'D' => Tile::D,
            _ => bail!("unknown tile {}", value as char),
        })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Nothing => ' ',
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::A => 'A',
            Tile::B => 'B',
            Tile::C => 'C',
            Tile::D => 'D',
        };
        write!(f, "{c}")
    }
}

type Burrow = Grid<Tile, Taxicab>;

fn parse_input(input: &str) -> Burrow {
    let lines = Vec::from_iter(input.lines());
    let mut buf = String::new();
    let line_len = lines.iter().map(|l| l.len()).max().unwrap();
    for line in lines {
        buf += line;
        if line.len() < line_len {
            buf += "  ";
        }
        buf += "\n";
    }
    Grid::try_from(&*buf).unwrap()
}

static FOLD: &str = r"  #D#C#B#A#  
  #D#B#A#C#  ";

fn parse_input_part_2(input: &str) -> Burrow {
    let lines = Vec::from_iter(input.lines());
    let mut buf = String::new();
    let line_len = lines.iter().map(|l| l.len()).max().unwrap();
    for line in lines {
        buf += line;
        if line.len() < line_len {
            buf += "  ";
        }
        if buf.len() > 3 * line_len && buf.len() < 4 * line_len {
            buf += "\n";
            buf += FOLD;
        }
        buf += "\n";
    }
    Grid::try_from(&*buf).unwrap()
}

fn reachable_tiles(grid: &Burrow, pos: (usize, usize)) -> Vec<(usize, (usize, usize))> {
    let mut reachables = vec![];
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), pos));
    let mut seen = HashSet::new();

    while let Some((d, p)) = queue.pop() {
        seen.insert(p);
        for n in grid.neighbours(p) {
            if grid[n] == Tile::Empty && !seen.contains(&n) {
                let d = d.0 + 1;
                queue.push((Reverse(d), n));
                reachables.push((d, n));
            }
        }
    }
    reachables
}

const HALLWAY_ROW: usize = 1;

// valid for part 1
fn valid_reachable_tiles(grid: &Burrow, pos: (usize, usize)) -> Vec<(usize, (usize, usize))> {
    assert!(grid[pos].is_amphipod());

    // already solved? Don't move anymore
    let col = grid[pos].amphipod_column();
    if pos.0 == col
        && pos.1 > HALLWAY_ROW
        && (pos.1..grid.height() - 1).all(|r| grid[(pos.0, r)] == grid[pos])
    {
        return vec![];
    }

    Vec::from_iter(
        reachable_tiles(grid, pos)
            .into_iter()
            .filter(|(_, p)| {
                if p.1 == pos.1 {
                    // must change row
                    false
                } else if pos.1 == HALLWAY_ROW {
                    let col = grid[pos].amphipod_column();
                    if p.0 != col {
                        false
                    } else {
                        p.1 > HALLWAY_ROW
                            && (p.1 + 1..grid.height() - 1).all(|r| grid[(p.0, r)] == grid[pos])
                    }
                } else if p.1 == HALLWAY_ROW {
                    p.0 != 3 && p.0 != 5 && p.0 != 7 && p.0 != 9
                } else {
                    false
                }
            })
            .map(|(e, p)| (e * grid[pos].amphipod_energy(), p)),
    )
}

fn successors(grid: &Burrow, pos: (usize, usize)) -> Vec<(usize, Burrow)> {
    let mut successors = vec![];

    for (e, p) in valid_reachable_tiles(grid, pos) {
        let mut succ_grid = grid.clone();
        succ_grid[p] = succ_grid[pos];
        succ_grid[pos] = Tile::Empty;
        successors.push((e, succ_grid));
    }
    successors
}

fn is_burrow_sorted(grid: &Burrow) -> bool {
    for (idx, t) in grid.iter().enumerate() {
        if t.is_amphipod() {
            let pos = grid.idx_to_pos(idx);
            if pos.0 != t.amphipod_column() || pos.1 == HALLWAY_ROW {
                return false;
            }
        }
    }
    true
}

// each "game" is shallow... probably a DFS is better here?
fn part_1(input: &str) -> usize {
    let grid = parse_input(input);

    mimimum_energy_to_sort(grid)
}

fn mimimum_energy_to_sort(grid: Grid<Tile, Taxicab>) -> usize {
    let mut queue = VecDeque::new();
    let mut best = HashMap::new();
    queue.push_front((Reverse(0), grid));
    let mut best_so_far = usize::MAX;

    while let Some((te, g)) = queue.pop_front() {
        if is_burrow_sorted(&g) {
            best_so_far = best_so_far.min(te.0);
        }
        best.insert(g.clone(), te.0);

        for (idx, t) in g.iter().enumerate() {
            if t.is_amphipod() {
                let p = g.idx_to_pos(idx);
                for (e, succ) in successors(&g, p) {
                    if let Some(bsf) = best.get(&succ) {
                        if best_so_far < te.0 + e || *bsf <= te.0 + e {
                            continue;
                        }
                    }
                    queue.push_front((Reverse(te.0 + e), succ));
                }
            }
        }
    }
    best_so_far
}

fn part_2(input: &str) -> usize {
    let grid = parse_input_part_2(input);
    mimimum_energy_to_sort(grid)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{is_burrow_sorted, parse_input, part_1, part_2, INPUT};

    static SORTED: &str = r"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";

    #[test]
    fn test_sorted() {
        let grid = parse_input(SORTED);
        assert!(is_burrow_sorted(&grid));
    }

    static ONE_STEP: &str = r"#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########";

    #[test]
    fn one_step() {
        assert_eq!(8, part_1(ONE_STEP));
    }

    static THREE_STEPS: &str = r"#############
#.....D.D.A.#
###.#B#C#.###
  #A#B#C#.#
  #########";

    #[test]
    fn three_steps() {
        assert_eq!(7008, part_1(THREE_STEPS));
    }

    static TEST_INPUT: &str = r"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_part_1() {
        assert_eq!(12521, part_1(TEST_INPUT));
        assert_eq!(14467, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(44169, part_2(TEST_INPUT));
        assert_eq!(48759, part_2(INPUT));
    }
}
