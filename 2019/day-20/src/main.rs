use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    ops::Index,
};

static INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct GateName {
    name: [u8; 2],
}

impl GateName {
    fn new(f: u8, s: u8) -> Self {
        Self { name: [f, s] }
    }
}

impl Display for GateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0] as char, self.name[1] as char)
    }
}

// hack; make display of containers that are only Debug a bit more readable
impl std::fmt::Debug for GateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
    gates: HashMap<GateName, (usize, usize)>,
    reversed_gates: HashMap<usize, GateName>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GateType {
    Outer,
    Inner,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let mut width = 0;
        let grid: Vec<u8> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .inspect(|l| width = l.len())
            .flat_map(|l| l.bytes())
            .collect();
        let height = grid.len() / width;
        let mut grid = Grid {
            grid,
            width,
            height,
            gates: HashMap::new(),
            reversed_gates: HashMap::new(),
        };
        grid.gates = grid.get_gates();
        grid.reversed_gates = grid
            .gates
            .iter()
            .flat_map(|(&k, &(f, t))| [(f, k), (t, k)])
            .collect();
        grid
    }

    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn pos_to_idx(&self, pos: (usize, usize)) -> usize {
        pos.0 + self.width * pos.1
    }

    fn direct_neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |d| {
                pos.0
                    .checked_add_signed(d.0)
                    .and_then(|x| pos.1.checked_add_signed(d.1).map(|y| (x, y)))
                    .filter(|(x, y)| *x < self.width && *y < self.height && self[(*x, *y)] == b'.')
            })
    }

    fn get_gates(&self) -> HashMap<GateName, (usize, usize)> {
        let mut gates = HashMap::new();

        for (idx, b) in self
            .grid
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, b)| b.is_ascii_uppercase())
        {
            let pos = self.idx_to_pos(idx);
            let neighbours: Vec<_> = self.direct_neighbours(pos).map(|p| (self[p], p)).collect();
            let Some((_, gate)) = neighbours.iter().find(|(b, _)| *b == b'.') else { continue };
            let dx = gate.0 as isize - pos.0 as isize;
            let dy = gate.1 as isize - pos.1 as isize;

            let b2_pos = (
                pos.0.checked_add_signed(-dx).unwrap(),
                pos.1.checked_add_signed(-dy).unwrap(),
            );

            let b2 = self[b2_pos];

            assert!(b2.is_ascii_uppercase());

            let gate = self.pos_to_idx(*gate);

            let (f, s) = if pos < b2_pos { (b, b2) } else { (b2, b) };

            let gate_name = GateName::new(f, s);
            gates.entry(gate_name).or_insert((gate, gate)).1 = gate;
        }

        gates
    }

    // map from idx to [(dist, idx)]
    fn compress_graph(&self) -> HashMap<usize, Vec<(usize, usize)>> {
        let mut gate_graph = HashMap::new();
        for (idx1, idx2) in self.gates.values() {
            gate_graph.insert(*idx1, self.find_reachable_gates(*idx1));
            gate_graph.insert(*idx2, self.find_reachable_gates(*idx2));
        }

        gate_graph
    }

    fn find_reachable_gates(&self, idx: usize) -> Vec<(usize, usize)> {
        let mut reachable_gates = Vec::new();
        let mut fringe = BinaryHeap::new();
        fringe.push((Reverse(0), idx));
        let mut best: HashMap<usize, usize> = HashMap::new();

        while let Some((steps, idx)) = fringe.pop() {
            let best_so_far = best.entry(idx).or_insert(usize::MAX);
            let dist = steps.0;
            if dist < *best_so_far {
                *best_so_far = dist;

                if dist > 0 {
                    // can't && both conditions...
                    if self.reversed_gates.get(&idx).is_some() {
                        reachable_gates.push((dist, idx));
                        continue;
                    }
                }
                for n in self.direct_neighbours(self.idx_to_pos(idx)) {
                    let new_idx = self.pos_to_idx(n);
                    if !best.contains_key(&new_idx) {
                        fringe.push((Reverse(dist + 1), new_idx));
                    }
                }
            }
        }

        reachable_gates
    }

    fn steps_to_exit(&self) -> usize {
        let start = self.gates.get(&GateName::new(b'A', b'A')).unwrap().0;
        let end = self.gates.get(&GateName::new(b'Z', b'Z')).unwrap().0;

        let graph = self.compress_graph();

        let mut fringe = BinaryHeap::new();
        let mut best = HashMap::new();

        fringe.push((Reverse(0), start));

        while let Some((steps, idx)) = fringe.pop() {
            let dist = steps.0;
            if idx == end {
                return steps.0;
            }

            let best_so_far = best.entry(idx).or_insert(usize::MAX);
            if dist < *best_so_far {
                *best_so_far = dist;

                let gate = self
                    .gates
                    .get(self.reversed_gates.get(&idx).unwrap())
                    .unwrap();
                let other = if gate.0 == idx { gate.1 } else { gate.0 };
                fringe.push((Reverse(dist + 1), other));

                for (d, g) in graph.get(&idx).unwrap() {
                    fringe.push((Reverse(dist + d), *g));
                }
            }
        }

        panic!()
    }

    fn gate_type(&self, idx: usize) -> GateType {
        let pos = self.idx_to_pos(idx);
        if pos.0 == 2 || pos.1 == 2 || pos.0 == self.width - 3 || pos.1 == self.height - 3 {
            GateType::Outer
        } else {
            GateType::Inner
        }
    }

    fn steps_to_exit_nested(&self) -> usize {
        let start = self.gates.get(&GateName::new(b'A', b'A')).unwrap().0;
        let end = self.gates.get(&GateName::new(b'Z', b'Z')).unwrap().0;

        let graph = self.compress_graph();

        let mut fringe = BinaryHeap::new();
        let mut best = HashMap::new();

        fringe.push((Reverse(0), Reverse(0), start));

        while let Some((steps, nesting, idx)) = fringe.pop() {
            let dist = steps.0;
            if idx == end && nesting.0 == 0 {
                return dist;
            }

            let best_so_far = best.entry((idx, nesting.0)).or_insert(usize::MAX);
            if dist < *best_so_far {
                *best_so_far = dist;

                let gate = self
                    .gates
                    .get(self.reversed_gates.get(&idx).unwrap())
                    .unwrap();
                let other = if gate.0 == idx { gate.1 } else { gate.0 };
                if idx != start && other != end {
                    let new_nesting = match self.gate_type(idx) {
                        GateType::Outer => nesting.0 - 1,
                        GateType::Inner => nesting.0 + 1,
                    };
                    if new_nesting >= 0 {
                        fringe.push((Reverse(dist + 1), Reverse(new_nesting), other));
                    }
                }

                for (d, g) in graph.get(&idx).unwrap() {
                    fringe.push((Reverse(dist + d), nesting, *g));
                }
            }
        }

        panic!()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.grid[self.pos_to_idx(pos)]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}x{}", self.width, self.height)?;
        for (idx, c) in self.grid.iter().enumerate() {
            if idx > 1 && idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", *c as char)?;
        }
        Ok(())
    }
}

fn part_1(input: &str) -> usize {
    let grid = Grid::from_str(input);
    grid.steps_to_exit()
}

fn part_2(input: &str) -> usize {
    let grid = Grid::from_str(input);
    grid.steps_to_exit_nested()
}
fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};
    use test_case::test_case;
    static SMALL_TEST_INPUT: &str = r"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

    static LARGER_TEST_INPUT: &str = r"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    #[test_case(SMALL_TEST_INPUT, 23)]
    #[test_case(LARGER_TEST_INPUT, 58)]
    #[test_case(INPUT, 644)]
    fn test_part_1(input: &str, steps: usize) {
        assert_eq!(steps, part_1(input));
    }

    static INTERESTING_TEST_INPUT: &str = r"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    #[test_case(SMALL_TEST_INPUT, 26)]
    #[test_case(INTERESTING_TEST_INPUT, 396)]
    #[test_case(INPUT, 7798)]
    fn test_part_2(input: &str, steps: usize) {
        assert_eq!(steps, part_2(input));
    }
}
