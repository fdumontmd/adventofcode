use std::collections::VecDeque;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const PREAMBLE: usize = 25;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

fn part1(numbers: &Vec<u64>, preamble: usize) -> u64 {
    let mut ring = VecDeque::with_capacity(preamble);
    let mut prefix = HashSet::with_capacity(preamble);

    for num in &numbers[..preamble] {
        ring.push_back(*num);
        prefix.insert(*num);
    }

    for num in &numbers[preamble..] {
        let mut found = false;
        for a in &ring {
            if *num > *a && prefix.contains(&(*num - *a)) {
                found = true;
                break;
            }
        }

        if !found {
            return *num;
        }

        let out = ring.pop_front().unwrap();
        prefix.remove(&out);

        ring.push_back(*num);
        prefix.insert(*num);
    }

    panic!()
}

struct RangeSum {
    ring: VecDeque<u64>,
    total: u64,
    target: u64,
}

impl RangeSum {
    fn new(target: u64) -> Self {
        RangeSum {
            ring: VecDeque::new(),
            total: 0,
            target,
        }
    }

    fn drop_one(&mut self) {
        if let Some(num) = self.ring.pop_front() {
            self.total -= num;
        }
    }

    fn push_one(&mut self, num: u64) {
        self.ring.push_back(num);
        self.total += num;
    }

    fn next_number(&mut self, num: u64) -> Option<u64> {
        self.push_one(num);
        while self.total > self.target {
            self.drop_one();
        }

        if self.total == self.target {
            let min = self.ring.iter().min().unwrap();
            let max = self.ring.iter().max().unwrap();
            Some(min + max)
        } else {
            None
        }
    }
}

fn part2(data: &Vec<u64>, key: u64) -> u64 {
    let mut r = RangeSum::new(key);

    for i in data {
        if let Some(num) = r.next_number(*i) {
            return num;
        }
    }
    panic!()
}

fn main() {
    let data = parse_input(INPUT);
    let key = part1(&data, PREAMBLE);
    println!("part 1: {}", key);
    println!("part 2: {}", part2(&data, key));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#;

    const TEST_PREAMBLE: usize = 5;

    #[test]
    fn check_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT), TEST_PREAMBLE), 127);
    }
    
    #[test]
    fn check_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT), 127), 62);
    }
}
