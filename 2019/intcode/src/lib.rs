use std::ops::{Index, Range};
use std::str::FromStr;
use anyhow::{Context, Error, Result};

pub type MemItem = u64;

pub struct Computer {
    memory: Vec<MemItem>,
    ic: MemItem,
}

impl Computer {
    pub fn new(memory: Vec<MemItem>) -> Self {
        Computer { memory, ic: 0 }
    }

    pub fn is_stopped(&self) -> bool {
        self[self.ic] == 99
    }

    pub fn step(&mut self) {
        match self[self.ic] {
            1 => {
                if let [op1, op2, target] = self[self.ic+1..self.ic+4] {
                    self.memory[target as usize] = self[op1] + self[op2];
                    self.ic += 4;
                } else {
                    unreachable!();
                }
            }
            2 => {
                if let [op1, op2, target] = self[self.ic+1..self.ic+4] {
                    self.memory[target as usize] = self[op1] * self[op2];
                    self.ic += 4;
                } else {
                    unreachable!();
                }
            }
            99 => {
            }
            _ => {
                eprintln!("Unknown opcode {} at addess {}", self[self.ic], self.ic);
            }
        }
    }

    pub fn run(&mut self) {
        while !self.is_stopped() {
            self.step();
        }
    }

    pub fn set_noun(&mut self, noun: MemItem) {
        self.memory[1] = noun;
    }

    pub fn set_verb(&mut self, verb: MemItem) {
        self.memory[2] = verb;
    }
}

impl FromStr for Computer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let memory: Vec<MemItem> = s.split(",").map(|code| code.trim()
                         .parse::<MemItem>()
                         .with_context(|| format!("cannot parse {} as code",
                                                  code )))
            .collect::<Result<Vec<_>>>()?;
        Ok(Computer::new(memory))
    }
}

impl Index<MemItem> for Computer {
    type Output = MemItem;

    fn index(&self, idx: MemItem) -> &Self::Output {
        &self.memory[idx as usize]
    }
}

impl Index<Range<MemItem>> for Computer {
    type Output = [MemItem];

    fn index(&self, r: Range<MemItem>) -> &Self::Output {
        &self.memory[r.start as usize.. r.end as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() -> Result<()>{
        let mut computer: Computer = "1,9,10,3,2,3,11,0,99,30,40,50".parse()?;
        computer.run();
        assert_eq!(computer[0], 3500);
        Ok(())
    }
}
