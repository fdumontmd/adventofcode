use std::{collections::HashSet, iter::FromIterator};

static INPUT: &str = include_str!("input.txt");

struct DeliveryIter<I: Iterator<Item = char>> {
    c: I,
    x: i64,
    y: i64,
    start: bool,
}

impl<I: Iterator<Item = char>> DeliveryIter<I> {
    fn new(c: I) -> DeliveryIter<I> {
        DeliveryIter {
            c,
            x: 0,
            y: 0,
            start: true,
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for DeliveryIter<I> {
    type Item = (i64, i64);
    fn next(&mut self) -> Option<(i64, i64)> {
        if self.start {
            self.start = false;
            return Some((0, 0));
        }
        if let Some(c) = self.c.next() {
            match c {
                '<' => {
                    self.x -= 1;
                }
                '>' => {
                    self.x += 1;
                }
                '^' => {
                    self.y -= 1;
                }
                'v' => {
                    self.y += 1;
                }
                _ => unreachable!(),
            }
            Some((self.x, self.y))
        } else {
            None
        }
    }
}

fn main() {
    let visited: HashSet<(i64, i64)> = HashSet::from_iter(DeliveryIter::new(INPUT.trim().chars()));

    println!("Houses visited by single Santa: {}", visited.len());

    let mut visited: HashSet<(i64, i64)> =
        HashSet::from_iter(DeliveryIter::new(INPUT.trim().chars().step_by(2)));
    visited.extend(DeliveryIter::new(INPUT.trim().chars().skip(1).step_by(2)));

    println!("Both Santa and RoboSanta: {}", visited.len());
}

#[cfg(test)]
mod tests {}
