use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut counters: Vec<HashMap<char, isize>> = Vec::new();

    for line in buffer.lines() {
        for (p, c) in line.chars().enumerate() {
            if counters.len() <= p {
                counters.push(HashMap::new());
            }

            *counters[p].entry(c).or_insert(0) += 1;
        } 
    }

    let mut simple = String::new();
    let mut complex = String::new();

    for mut h in counters.into_iter() {
        let mut v: Vec<(char, isize)> = h.drain().collect();
        v.sort_by_key(|&(_, c)| -c);
        simple.push(v[0].0);
        complex.push(v[v.len()-1].0);
    }

    println!("Simple key: {}", simple);
    println!("Modified key: {}", complex);
}
