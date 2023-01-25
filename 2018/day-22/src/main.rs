#![feature(binary_heap_retain)]
#![feature(never_type)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    ops::{Index, IndexMut},
};

static INPUT: &str = include_str!("input.txt");

const EROSION_LEVEL_MODULO: usize = 20183;

type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum AreaType {
    Rocky,
    Wet,
    Narrow,
}

impl From<usize> for AreaType {
    fn from(value: usize) -> Self {
        match value % 3 {
            0 => AreaType::Rocky,
            1 => AreaType::Wet,
            _ => AreaType::Narrow,
        }
    }
}

impl Display for AreaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            AreaType::Rocky => '.',
            AreaType::Wet => '=',
            AreaType::Narrow => '|',
        };
        write!(f, "{c}")
    }
}

impl AreaType {
    fn risk(&self) -> usize {
        match self {
            AreaType::Rocky => 0,
            AreaType::Wet => 1,
            AreaType::Narrow => 2,
        }
    }

    fn is_tool_compatible(&self, tool: Tool) -> bool {
        match self {
            AreaType::Rocky => tool == Tool::ClimbingGear || tool == Tool::Torch,
            AreaType::Wet => tool == Tool::ClimbingGear || tool == Tool::None,
            AreaType::Narrow => tool == Tool::Torch || tool == Tool::None,
        }
    }
}

// TODO: move grid to aoc_lib
struct Grid<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self.grid[index.0 + index.1 * self.width]
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.grid[index.0 + index.1 * self.width]
    }
}

impl<T> Grid<T> {
    fn from<U>(value: Grid<U>) -> Self
    where
        T: From<U>,
    {
        Self {
            grid: value.grid.into_iter().map(T::from).collect(),
            width: value.width,
            height: value.height,
        }
    }
}

impl<T: Default> Grid<T> {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Vec::from_iter((0..(width * height)).map(|_| T::default())),
            width,
            height,
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, c) in self.grid.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

// the subtlety is that the fastest path can go beyond the (0,0)xtarget grid,
// so that the number of nodes is not fixed.
struct SearchMap {
    grid: HashMap<Position, usize>,
    depth: usize,
    target: Position,
}

impl SearchMap {
    fn geological_index(&mut self, pos: Position) -> usize {
        if pos == (0, 0) || pos == self.target {
            0
        } else if pos.1 == 0 {
            pos.0 * 16807
        } else if pos.0 == 0 {
            pos.1 * 48271
        } else {
            // introduces recursion, but that does not seem to be a problem
            let left = self.erosion_level((pos.0 - 1, pos.1));
            let top = self.erosion_level((pos.0, pos.1 - 1));
            // must compute so that we now the values ahead of time -> col y == 0, row x == 0, then the
            // rest
            left * top
        }
    }

    fn erosion_level(&mut self, pos: Position) -> usize {
        let mut adjust = false;
        let mut el = *self.grid.entry(pos).or_insert_with(|| {
            adjust = true;
            0
        });
        if adjust {
            el = (self.geological_index(pos) + self.depth) % EROSION_LEVEL_MODULO;
            self.grid.insert(pos, el);
        }
        el
    }

    fn from_str(input: &str) -> Self {
        let (depth, target) = parse_input(input);

        Self {
            grid: HashMap::new(),
            depth,
            target,
        }
    }

    fn area_at(&mut self, pos: Position) -> AreaType {
        AreaType::try_from(self.erosion_level(pos) % 3).unwrap()
    }

    fn total_risk(&mut self) -> usize {
        let mut risk = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                risk += self.area_at((x, y)).risk();
            }
        }
        risk
    }

    fn neighbours(&mut self, pos: Position, tool: Tool) -> Vec<(Position, Tool, usize)> {
        let mut neighbours = Vec::new();

        // swap equiped tool
        for t in [Tool::None, Tool::ClimbingGear, Tool::Torch].into_iter() {
            if self.area_at(pos).is_tool_compatible(t) && t != tool {
                neighbours.push((pos, t, 7));
            }
        }

        // move to tool compatible neighbour
        if pos.0 > 0 && self.area_at((pos.0 - 1, pos.1)).is_tool_compatible(tool) {
            neighbours.push(((pos.0 - 1, pos.1), tool, 1));
        }

        if self.area_at((pos.0 + 1, pos.1)).is_tool_compatible(tool) {
            neighbours.push(((pos.0 + 1, pos.1), tool, 1));
        }

        if pos.1 > 0 && self.area_at((pos.0, pos.1 - 1)).is_tool_compatible(tool) {
            neighbours.push(((pos.0, pos.1 - 1), tool, 1));
        }

        if self.area_at((pos.0, pos.1 + 1)).is_tool_compatible(tool) {
            neighbours.push(((pos.0, pos.1 + 1), tool, 1));
        }

        neighbours
    }
    fn time_to_target(&mut self) -> usize {
        let mut fringe = BinaryHeap::new();

        fn dist(p1: Position, p2: Position) -> Reverse<usize> {
            Reverse(p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1))
        }

        // heuristic: by min time, then by min distance to target
        fringe.push((
            Reverse(0usize),
            dist((0, 0), self.target),
            (0usize, 0usize),
            Tool::Torch,
        ));

        let mut seen = HashSet::new();

        while let Some((cur_dist, _, cur_pos, cur_tool)) = fringe.pop() {
            let cur_dist = cur_dist.0;
            if seen.contains(&(cur_pos, cur_tool)) {
                continue;
            }
            seen.insert((cur_pos, cur_tool));

            if (cur_pos, cur_tool) == (self.target, Tool::Torch) {
                return cur_dist;
            }

            for (n_pos, n_tool, n_delta) in self.neighbours(cur_pos, cur_tool) {
                if !seen.contains(&(n_pos, n_tool)) {
                    fringe.push((
                        Reverse(cur_dist + n_delta),
                        dist(cur_pos, self.target),
                        n_pos,
                        n_tool,
                    ));
                }
            }
        }

        unreachable!()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Tool {
    None = 0,
    ClimbingGear = 1,
    Torch = 2,
}

impl TryFrom<usize> for Tool {
    type Error = !;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tool::None),
            1 => Ok(Tool::ClimbingGear),
            2 => Ok(Tool::Torch),
            _ => unreachable!(),
        }
    }
}

fn geological_index(pos: Position, target: Position, erosion_levels: &Grid<usize>) -> usize {
    if pos == (0, 0) || pos == target {
        0
    } else if pos.1 == 0 {
        pos.0 * 16807
    } else if pos.0 == 0 {
        pos.1 * 48271
    } else {
        // must compute so that we now the values ahead of time -> col y == 0, row x == 0, then the
        // rest
        erosion_levels[(pos.0 - 1, pos.1)] * erosion_levels[(pos.0, pos.1 - 1)]
    }
}

fn erosion_level(
    cave_depth: usize,
    pos: Position,
    target: Position,
    erosion_levels: &Grid<usize>,
) -> usize {
    (geological_index(pos, target, erosion_levels) + cave_depth) % EROSION_LEVEL_MODULO
}

fn parse_input(input: &str) -> (usize, Position) {
    let lines: Vec<_> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    let Some(d) = lines[0].strip_prefix("depth: ") else { panic!("cannot parse depth: {}", lines[0]) };
    let Some(p) = lines[1].strip_prefix("target: ") else { panic!("cannot parse target: {}", lines[1]) };

    let d = d.parse().unwrap();
    let p: Vec<_> = p.split(',').collect();
    let p = (p[0].parse().unwrap(), p[1].parse().unwrap());
    (d, p)
}

fn main() {
    let mut map = SearchMap::from_str(INPUT);
    println!("part 01: {}", map.total_risk());
    println!("part 02: {}", map.time_to_target());
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r"depth: 510
target: 10,10
";
    use crate::{SearchMap, INPUT};
    use test_case::test_case;

    #[test_case(TEST_INPUT, 114)]
    #[test_case(INPUT, 7402)]
    fn test_part_1(input: &str, risk: usize) {
        let mut map = SearchMap::from_str(input);
        assert_eq!(risk, map.total_risk());
    }

    #[test_case(TEST_INPUT, 45)]
    #[test_case(INPUT, 1025)]
    fn test_part_2(input: &str, time: usize) {
        let mut map = SearchMap::from_str(input);
        assert_eq!(time, map.time_to_target());
    }
}
