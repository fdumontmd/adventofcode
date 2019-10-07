extern crate regex;
extern crate permutohedron;

use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use permutohedron::Heap;
use regex::Regex;

type HappinessAdjustment = HashMap<(String, String), i64>;

struct Table {
    attendees: HashSet<String>,
    adjustment: HappinessAdjustment,
}

impl Table {
    fn new() -> Table {
        Table {
            attendees: HashSet::new(),
            adjustment: HappinessAdjustment::new(),
        }
    }

    fn change_happiness(&mut self, attendee: &str,
                        change: i64, neighbour: &str) {
        self.attendees.insert(String::from(attendee));
        self.adjustment.insert((String::from(attendee),
                                String::from(neighbour)), change);
    }

    fn gain(&mut self, attendee: &str, gain: i64, neighbour: &str) {
        self.change_happiness(attendee, gain, neighbour);
    }

    fn loss(&mut self, attendee: &str, loss: i64, neighbour: &str) {
        self.change_happiness(attendee, -loss, neighbour);
    }

    fn happiness_change(&self, sitting: &Vec<&String>) -> i64 {
        let mut change = 0;
        for chunks in sitting.as_slice().windows(3) {
            change += *self.adjustment.get(&(chunks[1].clone(), chunks[0].clone())).unwrap();
            change += *self.adjustment.get(&(chunks[1].clone(), chunks[2].clone())).unwrap();
        }

        // do the first and last attendees
        change += *self.adjustment.get(&(sitting[0].clone(), sitting[1].clone())).unwrap();
        change += *self.adjustment.get(&(sitting[0].clone(), sitting[sitting.len() - 1].clone())).unwrap();
        change += *self.adjustment.get(&(sitting[sitting.len() - 1].clone(), sitting[0].clone())).unwrap();
        change += *self.adjustment.get(&(sitting[sitting.len() - 1].clone(), sitting[sitting.len() - 2].clone())).unwrap();

        change
    }

    fn possible_sittings(&self) -> Vec<Vec<&String>> {
        let mut attendees: Vec<&String> = self.attendees.iter().collect();
        Heap::new(&mut attendees).collect()
    }

    fn possible_sittings_with_happiness(&self) -> Vec<(i64, Vec<&String>)> {
        let mut attendees: Vec<&String> = self.attendees.iter().collect();
        Heap::new(&mut attendees).map(|sitting| (self.happiness_change(&sitting), sitting)).collect()
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();

    let mut table = Table::new();

    for line in buffer.lines() {
        if let Some(ref caps) = re.captures(&line) {
            let attendee = caps.get(1).unwrap().into();
            let neighbour = caps.get(4).unwrap().into();
            let change = i64::from_str(caps.get(3).unwrap().into()).unwrap();
            let mode: &str = caps.get(2).unwrap().into();

            if mode == "gain" {
                table.gain(attendee, change, neighbour);
            } else {
                table.loss(attendee, change, neighbour);
            }
        } else {
            println!("cannot parse {}", line);
            panic!();
        }
    }

    println!("Optimal sitting: {:?}", table.possible_sittings_with_happiness().into_iter().max());

    for attendee in table.attendees.clone() {
        table.gain("Myself", 0, &attendee);
        table.gain(&attendee, 0, "Myself");
    }

    println!("Optimal sitting with me: {:?}", table.possible_sittings_with_happiness().into_iter().max());
}

#[test]
fn test() {
    let mut table = Table::new();
    table.gain("Alice", 54, "Bob");
    table.loss("Alice", 79, "Carol");
    table.loss("Alice", 2, "David");

    table.gain("Bob", 83, "Alice");
    table.loss("Bob", 7, "Carol");
    table.loss("Bob", 63, "David");

    table.loss("Carol", 62, "Alice");
    table.gain("Carol", 60, "Bob");
    table.gain("Carol", 55, "David");

    table.gain("David", 46, "Alice");
    table.loss("David", 7, "Bob");
    table.gain("David", 41, "Carol");


    assert_eq!(table.possible_sittings_with_happiness().into_iter().max().unwrap().0, 330);
}
