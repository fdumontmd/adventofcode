const DIVISOR: u64 = 2147483647;
const MULT_A: u64 = 16807;
const MULT_B: u64 = 48271;

const START_A: u64 = 873;
const START_B: u64 = 583;

const LOW_16: u64 = 256 * 256;

struct Generator {
    next_val: u64,
    multiplier: u64,
    filter: u64,
}

impl Generator {
    fn new(value: u64, multiplier: u64, filter: u64) -> Self {
        Generator {
            next_val: value,
            multiplier,
            filter,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.next_val = (self.next_val * self.multiplier) % DIVISOR;
            if self.next_val % self.filter == 0 {
                return Some(self.next_val);
            }
        }
    }
}

fn count_matches(limit: usize, gen_a: Generator, gen_b: Generator) -> usize {
    let mut count = 0;

    for (a, b) in gen_a.zip(gen_b).take(limit) {
        if a % LOW_16 == b % LOW_16 {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("Matches: {}", count_matches(40_000_000,
                                          Generator::new(START_A, MULT_A, 1),
                                          Generator::new(START_B, MULT_B, 1)));
    println!("Matches: {}", count_matches(5_000_000,
                                          Generator::new(START_A, MULT_A, 4),
                                          Generator::new(START_B, MULT_B, 8)));
}

#[test]
fn test_basic() {
    let gen_a = Generator::new(65, MULT_A, 1);
    let gen_b = Generator::new(8921, MULT_B, 1);

    assert_eq!(count_matches(40_000_000, gen_a, gen_b), 588);
}

#[test]
fn test_fancy() {
    let gen_a = Generator::new(65, MULT_A, 4);
    let gen_b = Generator::new(8921, MULT_B, 8);

    assert_eq!(count_matches(5_000_000, gen_a, gen_b), 309);
}
