extern crate crypto;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

use crypto::md5::Md5;
use crypto::digest::Digest;

#[derive(Copy, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn to_char(&self) -> char {
        match *self {
            Direction::U => 'U',
            Direction::D => 'D',
            Direction::L => 'L',
            Direction::R => 'R',
        }
    }

    fn from_hash(hash: &str) -> Vec<Direction> {
        assert!(hash.len() > 4, "Hash length is not at least 4");
        let mut iter = hash.chars();
        let mut result = Vec::new();

        if is_open(iter.next().unwrap()) {
            result.push(Direction::U);
        }
        if is_open(iter.next().unwrap()) {
            result.push(Direction::D);
        }
        if is_open(iter.next().unwrap()) {
            result.push(Direction::L);
        }
        if is_open(iter.next().unwrap()) {
            result.push(Direction::R);
        }
        result
    }
}

fn is_open(c: char) -> bool {
    match c {
        'b'...'f' => true,
        _ => false,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Location(u8, u8);

impl Location {
    fn is_valid_direction(&self, d: Direction) -> bool {
        match d {
            Direction::U => self.1 > 0,
            Direction::D => self.1 < 3,
            Direction::L => self.0 > 0,
            Direction::R => self.0 < 3,
        }
    }

    fn move_to(&self, d: Direction) -> Self {
        match d {
            Direction::U => Location(self.0, self.1 - 1),
            Direction::D => Location(self.0, self.1 + 1),
            Direction::L => Location(self.0 - 1, self.1),
            Direction::R => Location(self.0 + 1, self.1),
        }
    }

}

#[derive(PartialEq, Eq)]
struct State {
    path: String,
    location: Location,
}

impl State {
    fn new() -> Self {
        State {
            path: String::new(),
            location: Location(0, 0),
        }
    }

    fn at_exit(&self) -> bool {
        self.location.0 == 3 && self.location.1 == 3
    }

    fn move_to(&self, d: Direction) -> Self {
        let mut path = self.path.clone();
        path.push(d.to_char());
        State { path: path, location: self.location.move_to(d) }
    }

    fn children(&self, key: &str) -> Vec<State> {
        // once at exit, there's no way back into maze
        if self.at_exit() {
            return Vec::new();
        }
        let mut md5 = Md5::new();
        md5.input_str(key);
        md5.input_str(&self.path);

        Direction::from_hash(&md5.result_str()).into_iter()
            .filter(|d| self.location.is_valid_direction(*d))
            .map(|d| self.move_to(d))
            .collect()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.path.len().cmp(&self.path.len()) {
            Ordering::Equal => self.location.cmp(&other.location),
            o @ _ => o,
        }
    }
}

struct Search<'s> {
    key: &'s str,
    fringe: BinaryHeap<State>
}

impl<'s> Search<'s> {
    fn new(key: &'s str) -> Self {
        let mut fringe = BinaryHeap::new();
        fringe.push(State::new());
        Search{ key: key, fringe: fringe }
    }
}

impl<'s> Iterator for Search<'s> {
    type Item = State;

    fn next(&mut self) -> Option<State> {
        if let Some(state) = self.fringe.pop() {
            self.fringe.extend(state.children(self.key).into_iter());

            Some(state)
        } else {
            None
        }

    }
}

fn find(key: &str) -> Option<String> {
    for s in Search::new(key) {
        if s.at_exit() {
            return Some(s.path);
        }
    }

    None
}

fn find_longest(key: &str) -> Option<String> {
    let mut sol = None;
    for s in Search::new(key) {
        if s.at_exit() {
            sol = Some(s.path);
        }
    }
    sol
}

fn main() {
    println!("Path to exit: {:?}", find("bwnlcvfs"));
    println!("Longuest solution: {:?}", find_longest("bwnlcvfs").map(|s| s.len()));
}

#[test]
fn test_no_solution() {
    assert_eq!(None, find("hijkl"));
    assert_eq!(None, find_longest("hijkl"));
}

#[test]
fn test_solutions() {
    assert_eq!(Some(String::from("DDRRRD")), find("ihgpwlah"));
    assert_eq!(Some(String::from("DDUDRLRRUDRD")), find("kglvqrro"));
    assert_eq!(Some(String::from("DRURDRUDDLLDLUURRDULRLDUUDDDRR")), find("ulqzkmiv"));
}

#[test]
fn test_longest() {
    assert_eq!(Some(370), find_longest("ihgpwlah").map(|s| s.len()));
    assert_eq!(Some(492), find_longest("kglvqrro").map(|s| s.len()));
    assert_eq!(Some(830), find_longest("ulqzkmiv").map(|s| s.len()));
}
