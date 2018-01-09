extern crate aoc_utils;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use aoc_utils::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        let mut v = Vec::new();
        v.push(Position(self.0 - 1, self.1));
        v.push(Position(self.0 + 1, self.1));
        v.push(Position(self.0, self.1 - 1));
        v.push(Position(self.0, self.1 + 1));
        v
    }
}

struct Maze {
    targets: HashMap<u8, Position>,
    blocks: HashMap<Position, u8>,
    last_target: u8,
}

impl Maze {
    // that would be a package private function if I cared
    fn new() -> Maze {
        Maze {
            targets: HashMap::new(),
            blocks: HashMap::new(),
            last_target: b'0',
        }
    }

    fn is_target(&self, p: &Position) -> Option<u8> {
        self.blocks.get(p).and_then(|&b| {
            if b'0' <= b && b <= b'9' {
                Some(b)
            } else {
                None
            }
        })
    }

    fn is_open(&self, p: &Position) -> bool {
        self.blocks.get(p).map(|&b| b != b'#').unwrap_or(false)
    }

    fn children(&self, p: &Position) -> Vec<Position> {
        p.neighbours().into_iter().filter(|p| self.is_open(p)).collect()
    }
}

struct MazeBuilder {
    maze: Maze,
    current_row: isize,
}

impl MazeBuilder {
    fn new() -> MazeBuilder {
        MazeBuilder {
            maze: Maze::new(),
            current_row: 0,
        }
    }

    fn add_row(&mut self, row: &str) {
        for (p, b) in row.as_bytes().iter().cloned().enumerate() {
            self.maze.blocks.insert(Position(self.current_row, p as isize), b);
            if b'0' <= b && b <= b'9' {
                self.maze.targets.insert(b, Position(self.current_row, p as isize));
                self.maze.last_target = self.maze.last_target.max(b);
            }
        }
        self.current_row += 1;
    }

    fn build(self) -> Maze {
        self.maze
    }
}

#[derive(Debug)]
struct ShortestPathState {
    position: Position,
    distance: usize,
}

// a BFS search
struct ShortestPaths<'a> {
    maze: &'a Maze,
    seen: HashSet<Position>,
    fringe: VecDeque<ShortestPathState>,
}

impl<'a> ShortestPaths<'a> {
    fn start_from(maze: &'a Maze, origin: u8) -> Self {
        let mut fringe = VecDeque::new();
        let origin_pos = maze.targets.get(&origin).cloned().expect(&format!("Unknow origin {}", origin));
        fringe.push_back(ShortestPathState { position: origin_pos, distance: 0 });
        ShortestPaths {
            maze,
            seen: HashSet::new(),
            fringe,
        }
    }

    fn process_fringe(&mut self) -> Option<(Position, usize)> {
        while let Some(state) = self.fringe.pop_front() {
            if !self.seen.contains(&state.position) {
                self.seen.insert(state.position);
                for child in self.maze.children(&state.position) {
                    if !self.seen.contains(&child) {
                        self.fringe.push_back(ShortestPathState { position: child, distance: state.distance + 1 });
                    }
                }
                return Some((state.position, state.distance))
            }
        }
        None
    }

    fn next_target(&mut self) -> Option<(u8, usize)> {
        while let Some((p, d)) = self.process_fringe() {
            if let Some(t) = self.maze.is_target(&p) {
                return Some((t, d));
            }
        }
        None
    }
}

impl<'a> Iterator for ShortestPaths<'a> {
    type Item = (u8, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.next_target()
    }
}

fn shortest_distances(maze: &Maze) -> HashMap<(u8, u8), usize> {
    let mut distances = HashMap::new();

    for &origin in maze.targets.keys() {
        for (t, d) in ShortestPaths::start_from(&maze, origin) {
            if origin == t {
                assert_eq!(d, 0);
            } else {
                // sanity check
                let from = origin.min(t);
                let to = origin.max(t);

                if let Some(&dist) = distances.get(&(from, to)) {
                    assert_eq!(dist, d);
                }

                distances.insert((origin, t), d);
            }
        }
    }

    distances
}

fn next_permutation(v: &mut Vec<u8>) -> bool {
    let mut k = None;
    for sk in 1..v.len() {
        let sk = v.len() - 1 - sk;
        if v[sk] < v[sk+1] {
            k = Some(sk);
            break;
        }
    }

    if let Some(k) = k {
        for l in 0..(v.len() - k - 1) {
            let l = v.len() - l - 1;
            if v[k] < v[l] {
                v.swap(k, l);
                v[k+1..].reverse();
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut maze_builder = MazeBuilder::new();

    for line in get_input().lines() {
        let line = line.unwrap();
        maze_builder.add_row(&line);
    }

    let maze = maze_builder.build();

    let distances = shortest_distances(&maze);

    let mut v: Vec<u8> = (b'1'..maze.last_target + 1).collect();

    let mut min_dist = std::usize::MAX;

    loop {
        let mut dist = *distances.get(&(b'0', v[0])).unwrap();

        for pair in v.windows(2) {
            dist += *distances.get(&(pair[0], pair[1])).unwrap();
        }

        if min_dist > dist {
            println!("0 then {:?}", v);
        }

        min_dist = min_dist.min(dist);

        if !next_permutation(&mut v) {
            break;
        }
    }

    println!("min dist from 0: {}", min_dist);

    let mut v: Vec<u8> = (b'1'..maze.last_target + 1).collect();

    let mut min_dist = std::usize::MAX;

    loop {
        let mut dist = *distances.get(&(b'0', v[0])).unwrap();

        for pair in v.windows(2) {
            dist += *distances.get(&(pair[0], pair[1])).unwrap();
        }

        dist += *distances.get(&(v[v.len() -1], b'0')).unwrap();

        if min_dist > dist {
            println!("0 then {:?}", v);
        }

        min_dist = min_dist.min(dist);

        if !next_permutation(&mut v) {
            break;
        }
    }

    println!("min dist from 0 and back: {}", min_dist);
}
