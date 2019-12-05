use anyhow::{Context, Error, Result};
use std::ops::{Index, Range};
use std::str::FromStr;

pub type MemItem = i64;

#[derive(Debug)]
pub struct Computer {
    memory: Vec<MemItem>,
    ic: MemItem,
    input: MemItem,
    output: Vec<MemItem>,
}

impl Computer {
    pub fn new(memory: Vec<MemItem>) -> Self {
        Computer {
            memory,
            ic: 0,
            input: 1,
            output: Vec::new(),
        }
    }

    pub fn set_input(&mut self, input: MemItem) {
        self.input = input;
    }

    pub fn is_stopped(&self) -> bool {
        self.ic as usize >= self.memory.len() || self[self.ic] == 99
    }

    fn get_immediate(&self, offset: usize) -> MemItem {
        self.memory[self.ic as usize + offset]
    }

    fn get_value(&self, offset: usize) -> MemItem {
        let modes = self[self.ic] / 100;

        let v = self.memory[self.ic as usize + offset];

        let mode = (modes / 10i64.pow(offset as u32 - 1)) % 10;

        if mode == 0 {
            self.memory[v as usize]
        } else {
            v
        }
    }

    pub fn step(&mut self) {
        match self[self.ic] % 100 {
            1 => {
                if self.ic / 10000 != 0 {
                    panic!("invalid address mode {:?}", self);
                }
                let target = self.get_immediate(3) as usize;
                self.memory[target] = self.get_value(1) + self.get_value(2);
                self.ic += 4;
            }
            2 => {
                if self.ic / 10000 != 0 {
                    panic!("invalid address mode {:?}", self);
                }
                let target = self.get_immediate(3) as usize;
                self.memory[target] = self.get_value(1) * self.get_value(2);
                self.ic += 4;
            }
            3 => {
                if self.ic / 100 != 0 {
                    panic!("invalid address mode {:?}", self);
                }
                let target = self.get_immediate(1) as usize;
                self.memory[target] = self.get_input();
                self.ic += 2;
            }
            4 => {
                self.emit_output(self.get_value(1));
                self.ic += 2;
            }
            5 => {
                if self.get_value(1) != 0 {
                    self.ic = self.get_value(2);
                } else {
                    self.ic += 3;
                }
            }
            6 => {
                if self.get_value(1) == 0 {
                    self.ic = self.get_value(2);
                } else {
                    self.ic += 3;
                }
            }
            7 => {
                if self.ic / 1000 != 0 {
                    panic!("invalid address mode {:?}", self);
                }
                let target = self.get_immediate(3) as usize;
                self.memory[target] = if self.get_value(1) < self.get_value(2) {
                    1
                } else {
                    0
                };
                self.ic += 4;
            }
            8 => {
                if self.ic / 1000 != 0 {
                    panic!("invalid address mode {:?}", self);
                }
                let target = self.get_immediate(3) as usize;
                self.memory[target] = if self.get_value(1) == self.get_value(2) {
                    1
                } else {
                    0
                };
                self.ic += 4;
            }
            99 => {}
            _ => {
                panic!("Unknown opcode {} at addess {}", self[self.ic], self.ic);
            }
        }
    }

    fn get_input(&mut self) -> MemItem {
        self.input
    }

    pub fn get_output(&self) -> Vec<MemItem> {
        self.output.clone()
    }

    fn emit_output(&mut self, v: MemItem) {
        self.output.push(v);
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
        let memory: Vec<MemItem> = s
            .split(",")
            .map(|code| {
                code.trim()
                    .parse::<MemItem>()
                    .with_context(|| format!("cannot parse {} as code", code))
            })
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
        &self.memory[r.start as usize..r.end as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() -> Result<()> {
        let mut computer: Computer = "1,9,10,3,2,3,11,0,99,30,40,50".parse()?;
        computer.run();
        assert_eq!(computer[0], 3500);
        Ok(())
    }

    #[test]
    fn with_mode() -> Result<()> {
        let mut computer: Computer = "1002,4,3,4,33".parse()?;
        computer.run();
        assert_eq!(computer[4], 99);
        Ok(())
    }

    #[test]
    fn with_neg() -> Result<()> {
        let mut computer: Computer = "1101,100,-1,4,0".parse()?;
        computer.run();
        assert_eq!(computer[4], 99);
        Ok(())
    }

    #[test]
    fn test_input_eq_8() -> Result<()> {
        let mut computer: Computer = "3,9,8,9,10,9,4,9,99,-1,8".parse()?;
        computer.run();
        assert_eq!(computer.output, vec![0]);
        let mut computer: Computer = "3,9,8,9,10,9,4,9,99,-1,8".parse()?;
        computer.set_input(8);
        computer.run();
        assert_eq!(computer.output, vec![1]);
        Ok(())
    }

    #[test]
    fn test_less_than_8() -> Result<()> {
        let mut computer: Computer = "3,9,7,9,10,9,4,9,99,-1,8".parse()?;
        computer.run();
        assert_eq!(computer.output, vec![1]);
        let mut computer: Computer = "3,9,7,9,10,9,4,9,99,-1,8".parse()?;
        computer.set_input(9);
        computer.run();
        assert_eq!(computer.output, vec![0]);
        Ok(())
    }

    #[test]
    fn test_input_eq_8_imm() -> Result<()> {
        let mut computer: Computer = "3,3,1108,-1,8,3,4,3,99".parse()?;
        computer.run();
        assert_eq!(computer.output, vec![0]);
        let mut computer: Computer = "3,3,1108,-1,8,3,4,3,99".parse()?;
        computer.set_input(8);
        computer.run();
        assert_eq!(computer.output, vec![1]);
        Ok(())
    }

    #[test]
    fn test_less_than_8_imm() -> Result<()> {
        let mut computer: Computer = "3,3,1107,-1,8,3,4,3,99".parse()?;
        computer.run();
        assert_eq!(computer.output, vec![1]);
        let mut computer: Computer = "3,3,1107,-1,8,3,4,3,99".parse()?;
        computer.set_input(9);
        computer.run();
        assert_eq!(computer.output, vec![0]);
        Ok(())
    }

}
