extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::io::{self, Read};
use std::str::FromStr;

use regex::Regex;

type GridIndex = (i32, i32);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Node<'s> {
    index: GridIndex,
    name: &'s str,
    total: usize,
    avail: usize,
    used: usize,
}

fn parse_name(name: &str) -> GridIndex {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"node-x(\d+)-y(\d+)").unwrap();
    }

    if let Some(ref caps) = RE.captures(name) {
        (i32::from_str(caps.get(1).unwrap().as_str()).unwrap(),
         i32::from_str(caps.get(2).unwrap().as_str()).unwrap())
    } else {
        panic!("Cannot parse name {}", name);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Grid<'s>(HashMap<GridIndex, Node<'s>>);

impl<'s> Grid<'s> {
    fn new() -> Self {
        Grid(HashMap::new())
    }

    fn push(&mut self, n: Node<'s>) {
        self.0.insert(n.index, n);
    }

    fn index_of<P>(&self, mut p: P) -> GridIndex
        where P: FnMut(&Node) -> bool {
        for (k, v) in &self.0 {
            if p(&v) {
                return *k;
            }
        }
        unreachable!()
    }
}

fn find_empty_pos(grid: &Grid) -> GridIndex {
    grid.index_of (|n| n.used == 0)
}

fn find_data_pos(grid: &Grid) -> GridIndex {
    grid.index_of(|n| n.index.1 == 0 && n.index.0 == 31)
}

fn are_neighbours(n1: &Node, n2: &Node) -> bool {
    n1.index.0 == n2.index.0 && (n1.index.1 - n2.index.1).abs() == 1 
        || n1.index.1 == n2.index.1 && (n1.index.0 - n2.index.0).abs() == 1
}

// part 2: ideas
// - only track the empty node, and use it to move the data around
// - creates two additional empty nodes and try to use them as well

#[derive(Eq, PartialEq)]
struct SearchState<'s> {
    depth: i32,
    heuristic: i32,
    data_pos: GridIndex,
    empty_pos: GridIndex,
    grid: &'s Grid<'s>,
}

impl<'s> SearchState<'s> {
    fn new(g: &'s Grid<'s>) -> Self {
        let data_pos = find_data_pos(&g);
        let empty_pos = find_empty_pos(&g);
        let heuristic = data_pos.0;
        SearchState{
            depth: 0,
            heuristic: heuristic,
            data_pos: data_pos,
            empty_pos: empty_pos,
            grid: g,
        }
    }

    fn is_final(&self) -> bool {
        self.data_pos == (0, 0)
    }

    fn children(&self) -> Vec<SearchState<'s>> {
        let mut children = Vec::new();

        let x = self.empty_pos.0;
        let y = self.empty_pos.1;

        let free_node = self.grid.0.get(&self.empty_pos).unwrap();

        static DELTAS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

        for &(dx, dy) in &DELTAS { 
            if let Some(ref node) = self.grid.0.get(&(x+dx, y+dy)) {
                assert!(node.index.0 != self.empty_pos.0 || node.index.1 != self.empty_pos.1);
                if free_node.total >= node.used {
                    let data_pos = if node.index == self.data_pos {
                        free_node.index
                    } else {
                        self.data_pos
                    };
                    let heuristic = data_pos.0 + data_pos.1;

                    children.push(SearchState{
                        depth: self.depth + 1,
                        heuristic: heuristic + self.depth,
                        data_pos: data_pos,
                        empty_pos: node.index,
                        grid: self.grid,
                    });
                }
            }
        }

        children
    }

    fn get_key(&self) -> (GridIndex, GridIndex) {
        (self.empty_pos, self.data_pos)
    }
}

impl<'s> PartialOrd for SearchState<'s> {
    fn partial_cmp(&self, o: &SearchState) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl<'s> Ord for SearchState<'s> {
    fn cmp(&self, other: &SearchState) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

// gives the right answer by accident
// need to keep track of lowest depth for items not
// inserted into fringe
struct Search<'s> {
    depths: HashMap<((i32, i32), (i32, i32)), i32>,
    fringe: BinaryHeap<SearchState<'s>>,
}

impl<'s> Search<'s> {
    fn new(grid: &'s Grid) -> Self {
        let start = SearchState::new(grid);

        let mut depths = HashMap::new();
        depths.insert(start.get_key(), 0);

        let mut fringe = BinaryHeap::new();
        fringe.push(start);

        Search{
            depths,
            fringe,
        }
    }
}

impl<'s> Iterator for Search<'s> {
    type Item = SearchState<'s>;

    fn next(&mut self) -> Option<SearchState<'s>> {
        self.fringe.pop().map(|mut ss| {
            ss.depth = ss.depth.min(*self.depths.get(&ss.get_key()).unwrap());
            for new_state in ss.children() {
                let mut insert = false;
                let depth = self.depths.entry(new_state.get_key())
                    .or_insert_with(|| {
                        insert = true;
                        new_state.depth
                    });
                if *depth > new_state.depth {
                    *depth = new_state.depth;
                }
                if insert {
                    self.fringe.push(new_state);
                }
            }

            ss
        })
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(\S+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+\d+%").unwrap();

    let mut nodes = Vec::new();

    for line in buffer.lines() {
        if let Some(ref caps) = re.captures(&line) {
            let name = caps.get(1).unwrap().as_str();
            let total = usize::from_str(caps.get(2).unwrap().as_str()).unwrap();
            let used = usize::from_str(caps.get(3).unwrap().as_str()).unwrap();
            let avail = usize::from_str(caps.get(4).unwrap().as_str()).unwrap();

            let (x, y) = parse_name(name);
            nodes.push(Node{ name: name, index: (x, y), total: total, avail: avail, used: used});
        } else {
            println!("Cannot parse: {}", line);
        }
    }

    let mut count = 0;

    {
        let mut iter = nodes.as_slice().iter();

        while let Some(n) = iter.next() {
            for o in iter.as_slice().iter() {
                assert!(n.name != o.name);
                if o.used > 0 && n.avail >= o.used {
                    count += 1;
                    if are_neighbours(n, o) {
                        println!("Nodes {:?} and {:?} are neighbours", n, o);
                    }
                }
                if n.used > 0 && o.avail >= n.used {
                    count += 1;
                }
            }
        }
    }

    println!("Viable pairs: {}", count);

    let mut depth = 0;
    let mut grid = Grid::new();

    for n in nodes {
        grid.push(n);
    }

    let search = Search::new(&grid);

    for ss in search {
        if ss.is_final() {
            println!("Data moved after {} moves", ss.depth);
            break;
        } else {
            if depth < ss.depth {
                depth = ss.depth;
            }
        }

    }
}
