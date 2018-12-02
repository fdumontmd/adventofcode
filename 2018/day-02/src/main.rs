use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
use aoc_utils::get_input;

fn input() -> Result<Vec<String>, Box<dyn Error>> {
    let mut v = Vec::new();
    for line in get_input().lines() {
        let line = line?;
        v.push(line);
    }
    Ok(v)
}

fn checksum(s: &str) -> HashMap<char, usize> {
    let mut cs = HashMap::new();

    s.chars().for_each(|c| *cs.entry(c).or_default() += 1);

    cs
}

struct Filter {
    two: bool,
    three: bool,
}

impl Filter {
    fn from_checksum(cs: HashMap<char, usize>) -> Self {
        Filter {
            two: cs.values().any(|&s| s == 2),
            three: cs.values().any(|&s| s == 3),
        }
    }
}

fn part_one(v: &Vec<String>) -> usize {
    let filters: Vec<_> = v.iter().map(|s| Filter::from_checksum(checksum(&s))).collect();
    let count_two = filters.iter().filter(|f| f.two).count();
    let count_three = filters.iter().filter(|f| f.three).count();
    count_two * count_three
}

fn distance(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 != c2).count()
}

fn same_letters(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 == c2).map(|(c, _)| c).collect()
}

fn part_two(v: &Vec<String>) -> String {
    for (idx, s1) in v.iter().enumerate() {
        for s2 in &v[(idx + 1)..] {
            if distance(s1, s2) == 1 {
                return same_letters(s1, s2);
            }
        }
    }
    return "not found".into();
}

fn main() -> Result<(), Box<dyn Error>> {
    let v = input()?;
    println!("checksum: {}", part_one(&v));
    println!("candidate letters: {}", part_two(&v));
    Ok(())
}
