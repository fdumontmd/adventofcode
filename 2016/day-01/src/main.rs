use std::default::Default;
use std::io::{self, Read};
use std::str::FromStr;
use std::collections::BTreeSet;
use std::vec::Vec;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        use Direction::*;
        match *self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
    fn right(&self) -> Direction {
        use Direction::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    fn delta(&self) -> (i32, i32) {
        use Direction::*;
        match *self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }
}

struct Position {
    direction: Direction,
    position: (i32, i32),
    history: BTreeSet<(i32, i32)>,
    visited_twice: Vec<(i32, i32)>,
}

trait ManhattanDistance {
    fn distance(&self) -> i32;
}

impl ManhattanDistance for (i32, i32) {
    fn distance(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

impl Position {
    fn forward(&mut self, dist: i32) {
        let delta = self.direction.delta();
        for _ in 0..dist {
            self.position.0 += delta.0;
            self.position.1 += delta.1;

            if self.history.contains(&self.position) {
                self.visited_twice.push(self.position);
            }
            self.history.insert(self.position);
        }
    }
    fn left(&mut self, dist: i32) {
        self.direction = self.direction.left();
        self.forward(dist);
    }
    fn right(&mut self, dist: i32) {
        self.direction = self.direction.right();
        self.forward(dist);
    }
    fn distance(&self) -> i32 {
        self.position.distance()
    }
}

impl Default for Position {
    fn default() -> Self {
        let mut position = Position { direction: Direction::North, position: (0, 0), history: BTreeSet::default(), visited_twice: Vec::default() };
        position.history.insert(position.position);
        position
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let mut position: Position = Position::default();
    for command in buffer.split(",") {
        let command = command.trim();
        let dist = i32::from_str(&command[1..]).unwrap();
        if command.chars().next().unwrap() == 'L' {
            position.left(dist)
        } else {
            position.right(dist)
        }
    }

    println!("final distance: {}", position.distance());

    for pos in position.visited_twice {
        println!("first visited twice: {}", pos.distance());
        break;
    }
}
