use std::fmt;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    ops::Index,
};

static INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Keys(u32);

impl Keys {
    fn new() -> Self {
        Self(0)
    }

    fn has_key(&self, key: u8) -> bool {
        assert!(key.is_ascii_lowercase());
        self.0 & (2 << (key - b'a')) != 0
    }

    fn add_key(&self, key: u8) -> Self {
        assert!(key.is_ascii_lowercase());
        let mut other = *self;
        other.0 |= 2 << (key - b'a');
        other
    }

    fn key_count(&self) -> u32 {
        self.0.count_ones()
    }

    fn has_any(&self, other: Keys) -> bool {
        other.0 == 0 || self.0 & other.0 != 0
    }
}

impl fmt::Debug for Keys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys: String = (b'a'..=b'z')
            .filter(|k| self.has_key(*k))
            .map(|b| b as char)
            .collect();
        f.debug_tuple("Keys").field(&keys).finish()
    }
}

struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
    key_count: u32,
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
        let key_count = grid.iter().filter(|b| b.is_ascii_lowercase()).count() as u32;
        Self {
            grid,
            width,
            height,
            key_count,
        }
    }

    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn pos_to_idx(&self, pos: (usize, usize)) -> usize {
        pos.0 + self.width * pos.1
    }

    fn keys(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, b)| b.is_ascii_lowercase())
            .map(|(idx, _)| self.idx_to_pos(idx))
    }

    fn doors(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, b)| b.is_ascii_uppercase())
            .map(|(idx, _)| self.idx_to_pos(idx))
    }

    fn origin(&self) -> (usize, usize) {
        let idx = self.grid.iter().position(|&b| b == b'@').unwrap();
        self.idx_to_pos(idx)
    }

    fn neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |d| {
                pos.0
                    .checked_add_signed(d.0)
                    .and_then(|x| pos.1.checked_add_signed(d.1).map(|y| (x, y)))
                    .filter(|(x, y)| *x < self.width && *y < self.height && self[(*x, *y)] != b'#')
            })
    }
    // precompute step: make a graph between non-wall and non-empty, where we only expand
    // neighbourds of empty pos -> edges are between things that can be reached (from key to door,
    // or key to key without any obstacle), and then we can reason on this graph

    // return a map from significant idx (non-empty, non-wall) to directly reachable
    // in the minimum number of steps. Directly here means not going over another key or
    // throug a door
    fn compress_graph(&self) -> Graph {
        let mut neighbours = HashMap::new();
        for (idx, b) in self.grid.iter().enumerate().filter_map(|(idx, b)| {
            if b.is_ascii_alphabetic() || *b == b'@' {
                Some((idx, *b))
            } else {
                None
            }
        }) {
            neighbours.insert(b, self.find_neighbours(idx));
        }
        Graph {
            graph: neighbours,
            cache: HashMap::new(),
        }
    }

    fn find_neighbours(&self, orig_idx: usize) -> Vec<(usize, u8)> {
        let mut fringe = BinaryHeap::new();
        let mut seen = HashSet::new();
        let mut neighbours = Vec::new();

        fringe.push((Reverse(0), orig_idx));

        while let Some((steps, idx)) = fringe.pop() {
            seen.insert(idx);
            let b = self.grid[idx];
            if orig_idx != idx && b.is_ascii_alphabetic() {
                neighbours.push((steps.0, b));
            } else {
                for n in self.neighbours(self.idx_to_pos(idx)) {
                    if seen.contains(&self.pos_to_idx(n)) {
                        continue;
                    }
                    let new_idx = self.pos_to_idx(n);
                    fringe.push((Reverse(steps.0 + 1), new_idx));
                }
            }
        }

        neighbours
    }

    // unacceptably slow
    fn collect_all(&self) -> Option<usize> {
        let mut graph = self.compress_graph();
        let mut fringe = BinaryHeap::new();

        let mut best = HashMap::new();

        fringe.push((Reverse(0), self[self.origin()], Keys::new()));

        while let Some((steps, b, keys)) = fringe.pop() {
            if keys.key_count() == self.key_count {
                return Some(steps.0);
            }

            if let Some(&best_so_far) = best.get(&(b, keys)) {
                if steps.0 > best_so_far {
                    continue;
                }
            }

            for (dist, b) in graph.neighbours(b, keys) {
                let keys = keys.add_key(*b);
                let dist = steps.0 + dist;
                let best_entry = best.entry((*b, keys)).or_insert(usize::MAX);
                if dist < *best_entry {
                    *best_entry = dist;
                    fringe.push((Reverse(dist), *b, keys));
                }
            }
        }

        None
    }
}

#[derive(Debug)]
struct Graph {
    graph: HashMap<u8, Vec<(usize, u8)>>,
    cache: HashMap<(u8, Keys), Vec<(usize, u8)>>,
}

impl Graph {
    fn neighbours(&mut self, key: u8, keys: Keys) -> &Vec<(usize, u8)> {
        self.cache.entry((key, keys)).or_insert({
            let mut fringe = BinaryHeap::new();
            let mut distance = HashMap::new();

            fringe.push((Reverse(0), key));
            let mut neighbours = vec![];
            distance.insert(key, 0);

            while let Some((steps, b)) = fringe.pop() {
                let best_dist = distance.entry(b).or_insert(usize::MAX);
                if steps.0 > *best_dist {
                    continue;
                }
                *best_dist = steps.0;
                for (dist, b) in self.graph.get(&b).unwrap() {
                    // if upper but no key, cannot pass
                    if b.is_ascii_uppercase() && !keys.has_key(b.to_ascii_lowercase()) {
                        continue;
                    }
                    // if lower, but no key, add it as neighbour
                    if b.is_ascii_lowercase() && !keys.has_key(*b) {
                        neighbours.push((steps.0 + dist, *b));
                        continue;
                    }
                    let next_dist = distance.entry(*b).or_insert(usize::MAX);
                    if steps.0 + dist < *next_dist {
                        *next_dist = steps.0 + dist;

                        fringe.push((Reverse(steps.0 + dist), *b));
                    }
                    // otherwise keep searching
                }
            }
            neighbours
        })
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

fn main() {
    let grid = Grid::from_str(INPUT);
    println!("{:?}", grid.collect_all());
}

#[cfg(test)]
mod tests {
    use crate::Grid;
    use test_case::test_case;

    #[test_case(
        r"#########
#b.A.@.a#
#########",
        8
    )]
    #[test_case(
        r"########################
    #f.D.E.e.C.b.A.@.a.B.c.#
    ######################.#
    #d.....................#
    ########################",
        86
    )]
    #[test_case(
        r"########################
    #...............b.C.D.f#
    #.######################
    #.....@.a.B.c.d.A.e.F.g#
    ########################",
        132
    )]
    #[test_case(
        r"#################
    #i.G..c...e..H.p#
    ########.########
    #j.A..b...f..D.o#
    ########@########
    #k.E..a...g..B.n#
    ########.########
    #l.F..d...h..C.m#
    #################",
        136
    )]
    #[test_case(
        r"########################
    #@..............ac.GI.b#
    ###d#e#f################
    ###A#B#C################
    ###g#h#i################
    ########################",
        81
    )]
    fn test_part_1(input: &str, steps: usize) {
        assert_eq!(steps, Grid::from_str(input).collect_all().unwrap());
    }
}
