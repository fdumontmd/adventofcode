extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env::args;
use regex::Regex;

struct UnionFind {
    leaders: Vec<usize>,
    heights: Vec<usize>,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            leaders: Vec::new(),
            heights: Vec::new(),
        }
    }

    fn leader(&self, i: usize) -> usize {
        assert!(i < self.leaders.len());
        if self.leaders[i] == i {
            i
        } else {
            self.leader(self.leaders[i])
        }
    } 

    fn ensure_capacity(&mut self, i: usize) {
        if i >= self.leaders.len() {
            let len = self.leaders.len();
            self.leaders.extend(len..i+1);
            self.heights.extend((len..i+1).map(|_| 1));
        }
        assert!(self.leaders.len() > i && self.heights.len() > i);
    }

    fn join(&mut self, i: usize, j: usize) {
        self.ensure_capacity(i);
        self.ensure_capacity(j);

        let li = self.leader(i);
        let lj = self.leader(j);

        if li != lj {
            let hi = self.heights[li];
            let hj = self.heights[lj];

            if hi > hj {
                self.leaders[lj] = li;
                // heights cannot change here
            } else {
                self.leaders[li] = lj;
                self.heights[li] = hi.max(hj + 1);
            }
        }
    }

    fn same_group(&self, i: usize) -> Vec<usize> {
        let l = self.leader(i);

        (0..self.leaders.len()).filter(|&j| self.leader(j) == l).collect()
    }

    fn groups(&self) -> Vec<usize> {
        let mut g: Vec<usize> = (0..self.leaders.len()).map(|j| self.leader(j)).collect();
        g.sort();
        g.dedup();
        g
    }
}

fn main() {
    assert!(args().len() > 1);
    let path = args().nth(1).unwrap();
    let input = File::open(&path).unwrap();
    let buf = BufReader::new(input);

    let re = Regex::new(r"(\d+) <-> (.+)").unwrap();

    let mut uf = UnionFind::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let cap = re.captures(&line).unwrap();

        let i = cap[1].parse::<usize>().unwrap();
        for j in cap[2].split(", ") {
            let j = j.parse::<usize>().unwrap();
            uf.join(i, j);
        }
    }

    let g0 = uf.same_group(0);
    println!("Size of group containing 0: {}", g0.len());
    println!("Number of disjoint groups: {}", uf.groups().len());
}

#[test]
fn test() {
    let mut uf = UnionFind::new();

    uf.join(0, 2);
    uf.join(1, 1);
    uf.join(2, 0);
    uf.join(2, 3);
    uf.join(2, 4);
    uf.join(3, 2);
    uf.join(3, 4);
    uf.join(4, 2);
    uf.join(4, 3);
    uf.join(4, 6);
    uf.join(5, 6);
    uf.join(6, 4);
    uf.join(6, 5);

    assert_eq!(uf.same_group(0), vec![0, 2, 3, 4, 5, 6]);
    assert_eq!(uf.groups().len(), 2);
}
