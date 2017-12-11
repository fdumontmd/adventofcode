use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Memory(Vec<u32>);

impl Memory {
    fn new(v: Vec<u32>) -> Self {
        Memory(v)
    }

    fn balance(&self) -> Self {
        let mut v = self.0.clone();

        let (i, n) = self.0.iter().enumerate().max_by(|&(i1, n1), &(i2, n2)| n1.cmp(n2).then(i2.cmp(&i1))).unwrap();
        v[i] = 0;
        for idx in 0..*n {
            v[(1 + i+idx as usize) % self.0.len()] += 1;
        }

        Memory::new(v)
    }
}

fn main() {
    assert!(std::env::args().len() > 1);
    let path = std::env::args().nth(1).unwrap();
    let input = File::open(&path).unwrap();
    let buffered = BufReader::new(input);

    'main_block:
    for line in buffered.lines() {
        let v = line.unwrap().split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();

        let mut m = Memory::new(v);

        let mut seen = std::collections::HashSet::new();

        for steps in 0.. {
            if seen.contains(&m) {
                println!("Loop after {} steps", steps);
                let mut loop_m = m.balance();

                for steps in 1.. {
                    if loop_m == m {
                        println!("Loop size: {}", steps);
                        break 'main_block;
                    }

                    loop_m = loop_m.balance();
                }
            }

            let new_m = m.balance();

            seen.insert(m);

            m = new_m;

        }

        break;
    }

}
