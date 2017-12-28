extern crate regex;

use std::collections::HashMap;

struct Firewall {
    layers: HashMap<usize, usize>,
    max_layer: usize,
}

impl Firewall {
    fn new() -> Self {
        Firewall {
            layers: HashMap::new(),
            max_layer: 0,
        }
    }

    fn add_scanner(&mut self, depth: usize, range: usize) {
        self.max_layer = self.max_layer.max(depth);
        self.layers.insert(depth, range);
    }

    // could have a delay parameter, but not used in this project anyway
    fn severity(&self) -> usize {
        let mut s = 0;
        for p in 0..self.max_layer+1 {
            s += match self.layers.get(&p) {
                Some(&r) => {
                    if scanner_position(r, p) == 0 {
                        r * p
                    } else {
                        0
                    }
                }
                None => 0,
            }
        }
        s
    }

    fn safe(&self, delay: usize) -> bool {
        for p in 0..self.max_layer+1 {
            match self.layers.get(&p) {
                Some(&r) => {
                    if scanner_position(r, p + delay) == 0 {
                        return false;
                    }
                }
                None => {}
            }
        }
        true
    }
}

// for a given range, the position will be reset after
// range + range - 2, or (range -1) * 2
fn scanner_position(range: usize, time: usize) -> usize {
    if range > 1 {
        let cycle = (range - 1) * 2;
        let pos = time % cycle;
        if pos < range {
            pos
        } else {
            let pos = pos - range;
            range - pos - 2
        }
    } else {
        0
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::env::args;

    use regex::Regex;

    assert!(args().len() > 1);
    let path = args().nth(1).unwrap();
    let input = File::open(&path).unwrap();
    let buf = BufReader::new(input);

    let mut fw = Firewall::new();

    let re = Regex::new(r"(\d+): (\d+)").unwrap();

    for line in buf.lines() {
        let line = line.unwrap();

        let cap = re.captures(&line).unwrap();
        let d = cap[1].parse().unwrap();
        let r = cap[2].parse().unwrap();

        fw.add_scanner(d, r);
    }

    println!("severity: {}", fw.severity());

    for d in 0.. {
        if fw.safe(d) {
            println!("Safe at {}", d);
            break;
        }
    }
}

#[test]
fn test_scanner_position() {
    assert_eq!(scanner_position(1, 5), 0);
    assert_eq!(scanner_position(2, 0), 0);
    assert_eq!(scanner_position(2, 1), 1);
    assert_eq!(scanner_position(2, 2), 0);
    assert_eq!(scanner_position(3, 0), 0);
    assert_eq!(scanner_position(3, 1), 1);
    assert_eq!(scanner_position(3, 2), 2);
    assert_eq!(scanner_position(3, 3), 1);
    assert_eq!(scanner_position(3, 4), 0);
    assert_eq!(scanner_position(4, 0), 0);
    assert_eq!(scanner_position(4, 1), 1);
    assert_eq!(scanner_position(4, 2), 2);
    assert_eq!(scanner_position(4, 3), 3);
    assert_eq!(scanner_position(4, 4), 2);
    assert_eq!(scanner_position(4, 5), 1);
    assert_eq!(scanner_position(4, 6), 0);
}

#[test]
fn test_severity() {
    let mut fw = Firewall::new();
    fw.add_scanner(0, 3);
    fw.add_scanner(1, 2);
    fw.add_scanner(4, 4);
    fw.add_scanner(6, 4);

    assert_eq!(fw.severity(), 24);
    assert!(fw.safe(10));
}
