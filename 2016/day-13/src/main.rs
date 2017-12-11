use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location(i64, i64);

impl Location {
    fn new(x: i64, y: i64) -> Self {
        Location(x, y)
    }

    fn is_open(&self, code: i64) -> bool {
        (self.0 * self.0 + 3*self.0
            + 2*self.0*self.1
            + self.1 + self.1 * self.1
         + code).count_ones() %2 == 0
    }
    fn distance(&self, other: &Location) -> i64 {
        let x = (self.0 - other.0).abs();
        let y = (self.1 - other.1).abs();

        x+y
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    distance: u64,
    code: i64,
    location: Location,
    target: Location,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        let dist = other.distance.cmp(&self.distance);
        if dist == Ordering::Equal {
            other.location.distance(&other.target).cmp(
                &self.location.distance(&self.target))
        } else {
            dist
        }
    }
}

impl State {
    fn new(code: i64, x: i64, y: i64) -> Self {
        State {
            location: Location::new(1, 1),
            code: code,
            distance: 0,
            target: Location::new(x, y),
        }
    }

    fn reachable_states(&self) -> Vec<State> {
        let mut locations = Vec::new();
        locations.push(Location::new(self.location.0 + 1, self.location.1));
        locations.push(Location::new(self.location.0, self.location.1 + 1));

        if self.location.0 > 0 {
            locations.push(Location::new(self.location.0 - 1, self.location.1));
        }

        if self.location.1 > 0 {
            locations.push(Location::new(self.location.0, self.location.1 -1));
        }

        locations.into_iter()
            .filter(|l| l.is_open(self.code))
            .map(|l| State{
                location: l,
                code: self.code,
                distance: self.distance + 1,
                target: self.target,
            })
            .collect()
    }
}

fn print_maze(code: i64, max_x: i64, max_y: i64) {
    for y in 0..max_y {
        for x in 0..max_x {
            if Location::new(x, y).is_open(code) {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}

fn search(code: i64, x: i64, y: i64) -> Option<State> {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    queue.push(State::new(code, x, y));

    while let Some(state) = queue.pop() {
        if seen.contains(&state.location) {
            continue;
        }
        seen.insert(state.location);
        if state.location.0 == x && state.location.1 == y {
            return Some(state)
        }
        queue.extend(state.reachable_states().iter());
    }

    None
}

fn count(code: i64, max_dist: u64) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    queue.push(State::new(code, 1000000, 1000000));

    while let Some(state) = queue.pop() {
        if state.distance > max_dist {
            return Some(seen.len());
        }
        if seen.contains(&state.location) {
            continue;
        }
        seen.insert(state.location);
        queue.extend(state.reachable_states().iter());
    }
    None
}

fn main() {
    if let Some(state) = search(1352, 31, 39) {
        println!("Steps to reach destination: {}", state.distance);
    } else {
        println!("No solution?");
    }
    if let Some(locs) = count(1352, 50) {
        println!("Number of locations reached within 50 steps: {}", locs);
    } else {
        println!("Cannot count locations");
    }
}

#[test]
fn test() {
    let solution = search(10, 7, 4);
    assert!(solution.is_some());
    assert_eq!(11, solution.unwrap().distance);
}
