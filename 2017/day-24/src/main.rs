use std::{collections::HashSet, hash::Hash};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pin(u16, u16);

impl Pin {
    fn strength(&self) -> u16 {
        self.0 + self.1
    }
}

#[derive(Debug, Clone)]
struct Bridge {
    head: u16,
    strength: u16,
    pins: HashSet<Pin>,
}

impl Bridge {
    fn len(&self) -> usize {
        self.pins.len()
    }

    fn new() -> Self {
        Self {
            head: 0,
            strength: 0,
            pins: HashSet::new(),
        }
    }

    fn match_pin(&self, pin: &Pin) -> bool {
        (self.head == pin.0 || self.head == pin.1) && !self.pins.contains(pin)
    }

    fn extend_with(&self, pin: &Pin) -> Self {
        let mut s = self.clone();
        s.head = if self.head == pin.0 { pin.1 } else { pin.0 };
        s.strength += pin.strength();
        s.pins.insert(*pin);

        s
    }
}

fn parse_input(input: &str) -> Vec<Pin> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split('/').collect();
            Pin(parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

fn solve(input: &str) -> (u16, u16) {
    let pins = parse_input(input);

    // DFS
    let mut queue = vec![Bridge::new()];
    let mut best_strength = Bridge::new();
    let mut best_length = Bridge::new();

    while let Some(bridge) = queue.pop() {
        let mut extended = false;
        for pin in &pins {
            if bridge.match_pin(pin) {
                extended = true;
                queue.push(bridge.extend_with(pin));
            }
        }

        if !extended {
            if bridge.strength > best_strength.strength {
                best_strength = bridge.clone();
            }

            if bridge.len() > best_length.len()
                || (bridge.len() == best_length.len() && bridge.strength > best_length.strength)
            {
                best_length = bridge;
            }
        }
    }

    (best_strength.strength, best_length.strength)
}

fn main() {
    let solved = solve(INPUT);
    println!("part 1: {}", solved.0);
    println!("part 2: {}", solved.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
";

    #[test_case(TEST_INPUT, 31, 19)]
    #[test_case(INPUT, 1656, 1642)]
    fn test_solve(input: &str, best_strength: u16, best_length: u16) {
        let solved = solve(input);
        assert_eq!(best_strength, solved.0);
        assert_eq!(best_length, solved.1);
    }
}
