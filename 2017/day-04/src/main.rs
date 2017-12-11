use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    assert!(std::env::args().len() > 1);
    let path = std::env::args().nth(1).unwrap();

    let input = File::open(&path).unwrap();

    let buffered = BufReader::new(input);

    let mut count = 0;

    for line in buffered.lines() {
        let mut valid = true;
        let line = line.unwrap();
        let mut words = HashSet::new();

        for word in line.split_whitespace() {
            if word.len() > 0 {
                if words.contains(&word) {
                    valid = false;
                    break;
                }
                words.insert(word);
            }
        }

        if valid {
            count += 1;
        }
    }

    println!("Valid passphrases: {}", count);

    let input = File::open(&path).unwrap();

    let buffered = BufReader::new(input);

    let mut count = 0;

    for line in buffered.lines() {
        let mut valid = true;
        let line = line.unwrap();
        let mut words = HashSet::new();

        for word in line.split_whitespace() {
            if word.len() > 0 {
                let mut chars: Vec<char> = word.chars().collect();
                chars.sort();
                let word = String::from_iter(chars);
                if words.contains(&word) {
                    valid = false;
                    break;
                }
                words.insert(word);
            }
        }

        if valid {
            count += 1;
        }
    }

    println!("Valid improved passphrases: {}", count);
}
