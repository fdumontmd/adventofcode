extern crate regex;

const MAX_TIME: u64 = 2503;

use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Reindeer {
    speed: u64,
    duration: u64,
    rest: u64,
}

impl Reindeer {
    fn new(speed: u64, duration: u64, rest: u64) -> Self {
        Reindeer{speed: speed, duration: duration, rest: rest}
    }

    fn distance_after(&self, time: u64) -> u64 {
        let cycle_duration = self.duration + self.rest;
        let cycles = time / cycle_duration;
        let remaining_time = time % cycle_duration;

        let remaining_time = if remaining_time > self.duration {
            self.duration
        } else {
            remaining_time
        };

        self.speed * self.duration * cycles + remaining_time * self.speed
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds?, but then must rest for (\d+) seconds?.").unwrap();

    let mut reindeers = Vec::new();

    for line in buffer.lines() {
        if let Some(ref caps) = re.captures(&line) {
            let reindeer = String::from(caps.get(1).unwrap().as_str());
            let speed = u64::from_str(caps.get(2).unwrap().as_str()).unwrap();
            let duration = u64::from_str(caps.get(3).unwrap().as_str()).unwrap();
            let rest = u64::from_str(caps.get(4).unwrap().as_str()).unwrap();

            reindeers.push((reindeer, Reindeer::new(speed, duration, rest)));
        } else {
            println!("Cannot parse {}", line);
            panic!();
        }
    }

    println!("Best after {}: {:?}", MAX_TIME,
             reindeers.iter().map(|p| (p.1.distance_after(MAX_TIME), p.0.clone())).max());

    let mut score: HashMap<String, u64> = HashMap::new();

    for time in 1..(MAX_TIME + 1) {
        let race: Vec<(u64, String)> = reindeers.iter().map(|p| (p.1.distance_after(time), p.0.clone())).collect();
        if let Some(ref best_dist) = race.iter().max() {
            for best in race.iter().filter(|p| p.0 == best_dist.0) {
                *score.entry(best.1.clone()).or_insert(0) += 1;
            }
        } else {
            println!("No winner at time {}", time);
            panic!();
        }
    }

    println!("Scores: {:?}", score);
}

#[test]
fn test() {
    let comet = Reindeer::new(14, 10, 127);
    let dancer = Reindeer::new(16, 11, 162);

    assert_eq!(1120, comet.distance_after(1000));
    assert_eq!(1056, dancer.distance_after(1000));
}
