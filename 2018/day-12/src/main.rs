use aoc_utils::*;

use std::collections::HashMap;

const MAX_GEN: i64 = 50_000_000_000;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn input() -> Result<Problem> {
    let mut bufread = get_input();
    let mut initial = String::new();
    bufread.read_line(&mut initial)?;
    let initial: Vec<u8> = initial.split_off(15).trim().bytes().collect();
    // skip next line
    bufread.read_line(&mut String::new())?;

    let mut patterns = HashMap::new();

    for line in bufread.lines() {
        let line = line?;
        let bytes = line.as_bytes();
        let pattern: [u8; 5] =
            [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]];
        let result = bytes[9];

        patterns.insert(pattern, result);
    }

    Ok(Problem::new(initial, patterns))
}

struct Problem {
    current: Vec<u8>,
    left: i64,
    patterns: HashMap<[u8;5], u8>,
}

impl Problem {
    fn new(mut initial: Vec<u8>, patterns: HashMap<[u8;5], u8>) -> Self {
        let mut current = vec![b'.'; 4];
        current.append(&mut initial);
        current.append(&mut vec![b'.'; 4]);
        Problem {
            current,
            left: -4,
            patterns,
        }
    }

    // design: make sure that current always starts and ends with at least 4 .
    // to simplify iterating over it
    fn step(&mut self) -> bool {
        let mut next = vec![b'.'; 4];
        if let Some(first) = self.current.iter().position(|&b| b == b'#') {

            let left = self.left + (first as i64) - 6;
            let last = self.current.iter().rposition(|&b| b == b'#').unwrap();

            for w in self.current.as_slice()[first-4..last+4].windows(5) {
                next.push(*self.patterns.get(w).unwrap_or(&b'.'));
            }

            next.append(&mut vec![b'.'; 4]);
            self.current = next;
            self.left = left;
            true
        } else {
            false
        }

    }

    fn print(&self) {
        println!("{} -> {}", self.left, String::from_utf8_lossy(&self.current));
    }

    fn score(&self) -> i64 {
        self.current.iter().enumerate()
            .filter(|(_, &b)| b == b'#')
            .map(|(i, _)| (i as i64) + self.left)
            .sum::<i64>()
    }

    fn len(&self) -> usize {
        if let Some(first) = self.current.iter().position(|&b| b == b'#') {
            let last = self.current.iter().rposition(|&b| b == b'#').unwrap();
            last - first + 1
        } else {
            0
        }
    }
}

fn main() -> Result<()> {
    let mut prob = input()?;
    for _ in 1..=20 {
        prob.step();
    }
    println!("part one: {}", prob.score());

    let mut prob = input()?;
    // just enough to stabilise the pattern
    let range: std::ops::Range<i64> = 0..20000;
    // check the delta between step and left
    for s in range {
        if (s + 1) % 10000 == 0 {
            println!("step: {} len: {}: left: {} step-left: {}", s+1, prob.len(), prob.left, s - prob.left);
            prob.print();
        }
        if !prob.step() {
            break;
        }
    }
    // move all the way to "last" gen
    prob.left = MAX_GEN - 26;
    println!("part two: {}", prob.score());

    Ok(())
}
