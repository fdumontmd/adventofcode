extern crate knot_hash;
extern crate aoc_utils;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    use knot_hash::Knot;
    use aoc_utils::get_input_path;

    let path = get_input_path();
    let input = File::open(&path).unwrap();
    let buf = BufReader::new(input);

    for line in buf.lines() {
        let line = line.unwrap();
        let v = line.split(",").map(|w| w.parse().unwrap()).collect();
        let r = Knot::knot(256, v);
        println!("result: {}", r[0] as u32 * r[1] as u32);
    }

    let input = File::open(&path).unwrap();

    let buf = BufReader::new(input);

    for line in buf.lines() {
        let line = line.unwrap();

        println!("{}", Knot::hash(&line));
    }
}

