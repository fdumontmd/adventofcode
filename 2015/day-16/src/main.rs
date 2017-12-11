extern crate regex;

use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

use regex::Regex;

fn main() {
    let mut info = HashMap::new();
    info.insert("children", 3);
    info.insert("cats", 7);
    info.insert("samoyeds", 2);
    info.insert("pomeranians", 3);
    info.insert("akitas", 0);
    info.insert("vizslas", 0);
    info.insert("goldfish", 5);
    info.insert("trees", 3);
    info.insert("cars", 2);
    info.insert("perfumes", 1);

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(\w+): (\d+)").unwrap();

    for line in buffer.lines() {
        let mut is_match = true;
        for caps in re.captures_iter(&line) {
            let item = caps.at(1).unwrap();
            let count = usize::from_str(caps.at(2).unwrap()).unwrap();
            if count != 0 {
                let info_count = *info.get(item).unwrap();
                is_match &= info_count == count;
            }

        }
        if is_match {
            println!("{}", line);
        }
    }

    println!("");
    println!("Real Ant Sue");
    println!("");

    for line in buffer.lines() {
        let mut is_match = true;
        for caps in re.captures_iter(&line) {
            let item = caps.at(1).unwrap();
            let count = usize::from_str(caps.at(2).unwrap()).unwrap();
            let info_count = *info.get(item).unwrap();
            is_match &= if item == "cats" || item == "trees" {
                    info_count < count
                } else if item == "pomeranians" || item == "goldfish" {
                    info_count > count
                } else {
                    info_count == count
                }
        }

        if is_match {
            println!("{}", line);
        }
    }
}
