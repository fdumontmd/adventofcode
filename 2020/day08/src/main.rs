use std::result::Result;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum ExecStatus {
    Running,
    Complete(i64),
    Failed(i64),
}

#[derive(Debug, Clone)]
enum Instr {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
    Fail,
}

impl Instr {
    fn can_flip(&self) -> bool {
        match self {
            Instr::Nop(_) | Instr::Jmp(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    accumulator: i64,
    ic: i64,
    instructions: Vec<Instr>,
}

fn parse(input: &str) -> Computer {
    let instructions: Vec<Instr> = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            match &l[0..3] {
                "nop" => Instr::Nop(l[4..].parse().unwrap()),
                "acc" => Instr::Acc(l[4..].parse().unwrap()),
                "jmp" => Instr::Jmp(l[4..].parse().unwrap()),
                _ => unreachable!(),
            }
        }).collect();

    Computer {
        accumulator: 0,
        ic: 0,
        instructions,
    }
}

impl Computer {
    fn step(&mut self) -> ExecStatus {
        if self.ic as usize >= self.instructions.len() {
            ExecStatus::Complete(self.accumulator)
        } else {
            match std::mem::replace(&mut self.instructions[self.ic as usize], Instr::Fail) {
                Instr::Nop(_) => {
                    self.ic += 1;
                    ExecStatus::Running
                }
                Instr::Acc(v) => {
                    self.accumulator += v;
                    self.ic += 1;
                    ExecStatus::Running
                }
                Instr::Jmp(offset) => {
                    self.ic += offset;
                    ExecStatus::Running
                }
                Instr::Fail => {
                    ExecStatus::Failed(self.accumulator)
                }
            }
        }
    }

    fn run(&mut self) -> Result<i64, i64> {
        loop {
            match self.step() {
                ExecStatus::Running => {}
                ExecStatus::Complete(v) => {
                    return Ok(v);
                }
                ExecStatus::Failed(v) => {
                    return Err(v);
                }
            }
        }
    }

    fn flip(&mut self, ic: usize) {
        if self.instructions[ic].can_flip() {
            self.instructions[ic] = match self.instructions[ic] {
                Instr::Jmp(v) => Instr::Nop(v),
                Instr::Nop(v) => Instr::Jmp(v),
                _ => unreachable!(), 
            }
        }
    }
}

// TODO check reddit for the smart way
// but with rust, dumb is often fast enough
// bound: n^2 where n is number of instructions
fn repair(computer: &Computer) -> i64 {
    let candidates: Vec<_> = computer.instructions
        .iter()
        .enumerate()
        .filter(|(_, i)| i.can_flip())
        .map(|(idx, _)| idx)
        .collect();

    for idx in candidates {
        let mut test = computer.clone();
        test.flip(idx);

        if let Ok(v) = test.run() {
            return v;
        }
    }
    panic!()
}

fn part1(input: &str) -> i64 {
    let mut computer = parse(input);
    computer.run().err().unwrap()
}

fn part2(input: &str) -> i64 {
    let computer = parse(input);
    repair(&computer)
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#;

    #[test]
    fn check_part1() {
        assert_eq!(part1(TEST), 5);
    }

    #[test]
    fn check_part2() {
        assert_eq!(part2(TEST), 8);
    }
}

