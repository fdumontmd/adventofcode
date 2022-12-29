extern crate permutohedron;
extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::str::FromStr;

use permutohedron::Heap;
use regex::Regex;

struct DistanceMap {
    distances: HashMap<(String, String), usize>,
    destinations: HashSet<String>,
}

impl DistanceMap {
    fn new() -> Self {
        DistanceMap{
            distances: HashMap::new(),
            destinations: HashSet::new(),
        }
    }

    fn push(&mut self, from: &str, to: &str, dist: usize) {
        self.destinations.insert(String::from(from));
        self.destinations.insert(String::from(to));
        if from < to {
            self.distances.insert((String::from(from), String::from(to)), dist);
        } else {
            self.distances.insert((String::from(to), String::from(from)), dist);
        }
    }

    fn get(&self, from: &str, to: &str) -> Option<usize> {
        if from < to {
            self.distances.get(&(String::from(from), String::from(to))).cloned()
        } else {
            self.distances.get(&(String::from(to), String::from(from))).cloned()
        }
    }

    fn path_distance(&self, path: &Vec<&String>) -> Option<usize> {
        let mut total = 0;
        for segment in path.as_slice().windows(2) {
            if let Some(dist) = self.get(&segment[0], &segment[1]) {
                total += dist;
            } else {
                return None;
            }
        }
        Some(total)
    }

    fn possible_paths(&self) -> Vec<Vec<&String>> {
        let mut locations: Vec<&String> = self.destinations.iter().collect();
        let mut possible_paths = Vec::new();
        // will have to use unstable...
        let heap = Heap::new(&mut locations);
        for perm in heap {
            possible_paths.push(Vec::from_iter(perm.iter().cloned()));
        }
        possible_paths
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut distance_map = DistanceMap::new();

    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    for line in buffer.lines() {
        if let Some(caps) = re.captures(line) {
            let from = caps.get(1).unwrap().as_str();
            let to = caps.get(2).unwrap().as_str();
            let dist = usize::from_str(caps.get(3).unwrap().as_str()).unwrap();
            distance_map.push(from, to, dist);
        }
    }

    let possible_paths: Vec<(usize, Vec<&String>)> = distance_map.possible_paths().into_iter().map(|v| (distance_map.path_distance(&v), v)).filter(|p| p.0.is_some()).map(|p| (p.0.unwrap(), p.1)).collect();
    println!("Minimum path: {:?}", possible_paths.iter().min());
    println!("Maximum path: {:?}", possible_paths.iter().max());
}
