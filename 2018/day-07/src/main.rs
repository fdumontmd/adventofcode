use std::error::Error;
use aoc_utils::get_input;

#[derive(Copy, Clone)]
struct Before(u8, u8);
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn input() -> Result<Vec<Before>> {
    use std::io::BufRead;
    let mut v = Vec::new();
    for line in get_input().lines() {
        let line = line?;
        let first = line.as_bytes()[5] - b'A';
        let second = line.as_bytes()[36] - b'A';
        v.push(Before(first, second));
    }
    Ok(v)
}

fn count_vertex(input: &Vec<Before>) -> u8 {
    input.iter().map(|Before(f, _)| *f).chain(
        input.iter().map(|Before(_, s)| *s)
    ).max().unwrap() + 1
}

fn part_one(input: &Vec<Before>) -> String {
    use std::collections::BTreeSet;
    let mut result: Vec<u8> = Vec::new();
    let max_vertex = count_vertex(input);

    let mut candidates: BTreeSet<u8> = (0..max_vertex).collect();

    let mut remaining: Vec<Before> = input.to_vec();

    loop {
        let current_len = result.len();
        if candidates.is_empty() {
            break;
        }

        for c in &candidates {
            if remaining.iter().filter(|Before(_, s)| c == s).count() == 0 {
                result.push(*c);
                break;
            }
        }

        result.iter().skip(current_len).for_each(|c| {
            candidates.remove(c);
            remaining = remaining.iter().cloned().filter(|Before(f, _)| *f != *c).collect();
        });
    }

    String::from_utf8(result.into_iter().map(|v| v + b'A').collect::<Vec<u8>>()).unwrap()
}


use std::cmp::{PartialOrd, Ord, Ordering};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Task(usize, u8);

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// idea: once candidates are selected they are put in heap (up to num of
// available workers)
// once all workers are busy, or if there are no more candidates, pull the
// next item from the heap, add it to result, and clear remaining, then
// check for next task (until either no candidate or no worker)
fn part_two(input: &Vec<Before>, workers: usize, base_time: usize) -> usize {
    use std::collections::BTreeSet;
    use std::collections::BinaryHeap;
    let max_vertex = count_vertex(input);

    let mut candidates: BTreeSet<u8> = (0..max_vertex).collect();

    let mut tasks = BinaryHeap::new();
    let mut available_workers = workers;

    let mut remaining: Vec<Before> = input.to_vec();
    let mut current_time = 0;

    loop {
        if candidates.is_empty() {
            break;
        }

        let mut selected = Vec::new();

        for c in &candidates {
            if remaining.iter().filter(|Before(_, s)| c == s).count() == 0 {
                available_workers -= 1;

                let time = current_time + base_time + *c as usize;

                selected.push(*c);
                let task = Task(time, *c);
                tasks.push(task);

                if available_workers == 0 {
                    break;
                }
            }
        }

        selected.into_iter().for_each(|c| { candidates.remove(&c); });

        if let Some(Task(t, c)) = tasks.pop() {
            current_time = t + 1;
            available_workers += 1;
            remaining = remaining.iter().cloned().filter(|Before(f, _)| *f != c).collect();
        }
    }

    current_time
}

fn main() -> Result<()> {
    let input = input()?;
    println!("first part: {}", part_one(&input));
    println!("second part: {}", part_two(&input, 5, 60));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    fn input() -> Vec<Before> {
        let mut v = Vec::new();
        for line in INPUT.lines() {
            let first = line.as_bytes()[5] - b'A';
            let second = line.as_bytes()[36] - b'A';
            v.push(Before(first, second));
        }
        v
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&input()), "CABDFE");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&input(), 2, 0), 15);
    }
}
