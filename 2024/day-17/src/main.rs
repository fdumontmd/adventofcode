use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    ip: usize,
    output: Vec<u8>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut program = vec![];

        for line in input.lines() {
            if let Some(n) = line.strip_prefix("Register A: ") {
                a = n.parse().unwrap();
            }
            if let Some(n) = line.strip_prefix("Register B: ") {
                b = n.parse().unwrap();
            }
            if let Some(n) = line.strip_prefix("Register C: ") {
                c = n.parse().unwrap();
            }
            if let Some(p) = line.strip_prefix("Program: ") {
                program = p.split(',').map(|n| n.parse().unwrap()).collect();
            }
        }

        Computer {
            a,
            b,
            c,
            program,
            ip: 0,
            output: vec![],
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.output.clear();
    }

    fn output_is_suffix(&self) -> bool {
        self.program.ends_with(&self.output)
    }

    fn output_is_program(&self) -> bool {
        self.program == self.output
    }

    fn op(&self) -> Op {
        self.program[self.ip].into()
    }

    fn operand(&self) -> u8 {
        self.program[self.ip + 1]
    }

    fn combo(&self) -> u64 {
        let operand = self.operand();
        match operand {
            0..=3 => operand.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("reversed, should never be used!"),
        }
    }

    fn is_halted(&self) -> bool {
        self.ip >= self.program.len()
    }

    fn execute(&mut self) {
        if !self.is_halted() {
            match self.op() {
                Op::Adv => {
                    let denominator = 2u64.pow(self.combo() as u32);
                    self.a /= denominator;
                    self.ip += 2;
                }
                Op::Bxl => {
                    self.b ^= self.operand() as u64;
                    self.ip += 2;
                }
                Op::Bst => {
                    self.b = self.combo() % 8;
                    self.ip += 2;
                }
                Op::Jnz => {
                    if self.a == 0 {
                        self.ip += 2;
                    } else {
                        self.ip = self.operand() as usize;
                    }
                }
                Op::Bxc => {
                    self.b ^= self.c;
                    self.ip += 2;
                }
                Op::Out => {
                    self.output.push((self.combo() % 8) as u8);
                    self.ip += 2;
                }
                Op::Bdv => {
                    let denominator = 2u64.pow(self.combo() as u32);
                    self.b = self.a / denominator;
                    self.ip += 2;
                }
                Op::Cdv => {
                    let denominator = 2u64.pow(self.combo() as u32);
                    self.c = self.a / denominator;
                    self.ip += 2;
                }
            }
        }
    }
}

enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            _ => Op::Cdv,
        }
    }
}

fn part1(input: &str) -> String {
    let mut computer = Computer::parse(input);
    while !computer.is_halted() {
        computer.execute();
    }
    let output: Vec<String> = computer
        .output
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    output.join(",")
}

// both test input and input consume A one byte at a time;
// work from end to find out the bytes
// more than one byte can match output; use a queue to
// do a BFS
fn part2(input: &str) -> u64 {
    let mut computer = Computer::parse(input);
    let mut candidates = VecDeque::new();
    candidates.push_front(0);

    while let Some(a) = candidates.pop_back() {
        computer.reset();
        for byte in 0..8 {
            let a = a * 8 + byte;
            computer.reset();
            computer.a = a;
            while !computer.is_halted() {
                computer.execute();
            }
            if computer.output_is_suffix() {
                if computer.output_is_program() {
                    return a;
                } else {
                    candidates.push_front(a);
                }
            }
        }
    }
    panic!()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test_case(TEST_INPUT, "4,6,3,5,6,3,5,2,1,0"; "test input")]
    #[test_case(INPUT, "1,3,7,4,6,4,2,3,5"; "input")]
    fn test_part1(input: &str, output: &str) {
        assert_eq!(output, &part1(input));
    }

    const TEST_INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test_case(TEST_INPUT_2, 117440; "test input")]
    #[test_case(INPUT, 202367025818154; "input")]
    fn test_part2(input: &str, a: u64) {
        assert_eq!(a, part2(input));
    }
}
