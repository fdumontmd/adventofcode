use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    assert!(std::env::args().len() > 1);
    let path = std::env::args().nth(1).unwrap();

    let input = File::open(&path).unwrap();
    let buffered = BufReader::new(input);

    let mut jmps = Vec::new();

    for line in buffered.lines() {
        let line = line.unwrap();

        match line.parse::<isize>() {
            Ok(jmp) => jmps.push(jmp),
            Err(_) => {}
        }
    }

    let bck = jmps.clone();

    let mut steps = 0;
    let mut curr_idx: usize = 0;

    loop {
        if curr_idx >= jmps.len() {
            break;
        }
        let next_idx = curr_idx as isize + jmps[curr_idx];
        jmps[curr_idx] += 1;

        assert!(next_idx >= 0);
        curr_idx = next_idx as usize;
        steps += 1;
    }

    println!("Escape after {}", steps);

    let mut steps = 0;
    let mut curr_idx: usize = 0;
    let mut jmps = bck;

    loop {
        if curr_idx >= jmps.len() {
            break;
        }
        let next_idx = curr_idx as isize + jmps[curr_idx];
        if jmps[curr_idx] >= 3 {
            jmps[curr_idx] -= 1;
        } else {
            jmps[curr_idx] += 1;
        }
        assert!(next_idx >= 0);
        curr_idx = next_idx as usize;
        steps += 1;
    }

    println!("Escape after {}", steps);
}
