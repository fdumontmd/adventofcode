extern crate regex;

use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::env::args;
use regex::Regex;

fn main() {
    assert!(args().len() > 1);
    let path = args().nth(1).unwrap();
    let input = File::open(path).unwrap();
    let input = BufReader::new(input);

    let mut programs = HashSet::new();
    let mut supported = HashSet::new();

    let mut program_weights = HashMap::new();
    let mut program_discs = HashMap::new();
    let re = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.*))?$").unwrap();

    for line in input.lines() {
        let line = line.unwrap();
        for cap in re.captures_iter(&line) {
            assert!(cap.len() > 2);

            programs.insert(cap[1].to_owned());
            program_weights.insert(cap[1].to_owned(), cap[2].parse::<u32>().unwrap());
            let mut vec = Vec::new();
            if cap.get(3).is_some() {
                for prog in cap[3].split(", ") {
                    supported.insert(prog.to_owned());
                    vec.push(prog.to_owned());
                }
            }
            program_discs.insert(cap[1].to_owned(), vec);
        }
    }

    for p in programs.difference(&supported) {
        println!("root: {}", p);
    }

    let mut unbalanced = Vec::new();
    let mut diff: i32 = 0;
    for (p, v) in program_discs.iter() {
        if !v.is_empty() {
            let mut w: Vec<(&String, u32)> = v.iter().map(|p| (p, compute_weight(p, &program_weights, &program_discs))).collect();
            w.sort_by(|&(_, w1), &(_, w2)| w1.cmp(&w2));
            if w[0].1 != w[w.len() - 1].1 {
                assert!(w.len() > 2);
                if w[0].1 == w[1].1 {
                    diff = w[0].1 as i32 - w[w.len() - 1].1 as i32;
                    unbalanced.push(w[w.len() - 1]);
                } else {
                    diff = w[0].1 as i32 - w[1].1 as i32;
                    unbalanced.push(w[0]);
                }
                println!("balance problem on disc of {} - {:?}", p, w);
            }
        }
    }
    unbalanced.sort_by(|&(_, w1), &(_, w2)| w1.cmp(&w2));
    let p = unbalanced[0];
    let w = program_weights.get(p.0).unwrap();
    println!("{} should have weight {}", p.0, *w as i32 + diff);
}

fn compute_weight(p: &String, pw: &HashMap<String, u32>, pd: &HashMap<String, Vec<String>>) -> u32 {
    *pw.get(p).unwrap() + pd.get(p).unwrap().iter().map(|p| compute_weight(p, pw, pd)).sum::<u32>()
}
