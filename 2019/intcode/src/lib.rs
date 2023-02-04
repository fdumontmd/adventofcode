use anyhow::{Context, Error, Result};
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Write;
use std::ops::{Index, Range};
use std::str::FromStr;

pub type MemItem = i64;

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn new(mode: MemItem) -> Self {
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq)]
enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JIT,
    JIF,
    TLT,
    TEq,
    SetBase,
    Stop,
}

impl OpCode {
    fn new(opcode: MemItem) -> Self {
        use OpCode::*;
        match opcode % 100 {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
            5 => JIT,
            6 => JIF,
            7 => TLT,
            8 => TEq,
            9 => SetBase,
            99 => Stop,
            _ => panic!("unknown opcode {}", opcode),
        }
    }

    fn parameter_count(&self) -> usize {
        use OpCode::*;
        match *self {
            Add | Mul | TLT | TEq => 3,
            JIT | JIF => 2,
            Input | Output | SetBase => 1,
            Stop => 0,
        }
    }
}

struct Instruction<'a> {
    computer: &'a mut Computer,
    opcode: OpCode,
    parameters: Vec<ParameterMode>,
}

impl<'a> Instruction<'a> {
    fn new(computer: &'a mut Computer) -> Self {
        let instr = computer.get_instruction();
        let opcode = OpCode::new(instr);

        let mut modes = instr / 100;
        let mut parameters = Vec::new();

        for _ in 0..opcode.parameter_count() {
            parameters.push(ParameterMode::new(modes % 10));
            modes /= 10;
        }

        Self {
            computer,
            opcode,
            parameters,
        }
    }

    fn get(&mut self, param: usize) -> MemItem {
        match self.parameters[param - 1] {
            ParameterMode::Position => {
                let location = self.computer.get_parameter(param) as usize;
                self.computer.get_at(location)
            }
            ParameterMode::Immediate => self.computer.get_parameter(param),
            ParameterMode::Relative => {
                let relative = self.computer.get_parameter(param) as isize;
                let location: usize = (relative + self.computer.base).try_into().unwrap();
                self.computer.get_at(location)
            }
        }
    }

    fn set(&mut self, param: usize, value: MemItem) {
        let location = match self.parameters[param - 1] {
            ParameterMode::Position => self.computer.get_parameter(param) as usize,
            ParameterMode::Relative => {
                let relative = self.computer.get_parameter(param) as isize;
                (relative + self.computer.base).try_into().unwrap()
            }
            ParameterMode::Immediate => {
                panic!(
                    "cannot write to immediate parameter: ic: {}, instr: {}",
                    self.computer.ic,
                    self.computer.get_instruction()
                )
            }
        };
        self.computer.set_at(location, value);
    }

    fn increase_ic(&mut self) {
        self.computer.ic += self.opcode.parameter_count() + 1;
    }

    fn execute(&mut self) {
        use OpCode::*;
        match self.opcode {
            Add => {
                let value = self.get(1) + self.get(2);
                self.set(3, value);
                self.increase_ic();
            }
            Mul => {
                let value = self.get(1) * self.get(2);
                self.set(3, value);
                self.increase_ic();
            }
            Input => {
                let value = self.computer.get_input();
                self.set(1, value);
                self.increase_ic();
            }
            Output => {
                let value = self.get(1);
                self.computer.emit_output(value);
                self.increase_ic();
            }
            JIT => {
                if self.get(1) != 0 {
                    self.computer.ic = self.get(2) as usize;
                } else {
                    self.increase_ic();
                }
            }
            JIF => {
                if self.get(1) == 0 {
                    self.computer.ic = self.get(2) as usize;
                } else {
                    self.increase_ic();
                }
            }
            TLT => {
                if self.get(1) < self.get(2) {
                    self.set(3, 1);
                } else {
                    self.set(3, 0);
                }

                self.increase_ic();
            }
            TEq => {
                if self.get(1) == self.get(2) {
                    self.set(3, 1);
                } else {
                    self.set(3, 0);
                }

                self.increase_ic();
            }
            SetBase => {
                self.computer.base += self.get(1) as isize;

                self.increase_ic();
            }

            Stop => {}
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Computer {
    ic: usize,
    base: isize,
    memory: Vec<MemItem>,
    input: VecDeque<MemItem>,
    output: Vec<MemItem>,
}

impl Computer {
    pub fn new(memory: Vec<MemItem>) -> Self {
        Computer {
            memory,
            ic: 0,
            base: 0,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }

    pub fn get_at(&mut self, location: usize) -> MemItem {
        if location >= self.memory.len() {
            self.memory.resize(location + 1, 0);
        }
        self.memory[location]
    }

    pub fn set_at(&mut self, location: usize, value: MemItem) {
        if location >= self.memory.len() {
            self.memory.resize(location + 1, 0);
        }
        self.memory[location] = value;
    }

    fn get_instruction(&self) -> MemItem {
        self.memory[self.ic]
    }

    pub fn add_input(&mut self, input: MemItem) {
        self.input.push_back(input);
    }

    pub fn is_stopped(&self) -> bool {
        self.ic >= self.memory.len() || self.get_instruction() == 99
    }

    fn get_parameter(&self, offset: usize) -> MemItem {
        self.memory[self.ic + offset]
    }

    pub fn step(&mut self) {
        Instruction::new(self).execute();
    }

    fn get_input(&mut self) -> MemItem {
        self.input.pop_front().unwrap()
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

    pub fn waiting_for_input(&self) -> bool {
        OpCode::new(self.get_instruction()) == OpCode::Input && self.input.is_empty()
    }

    pub fn wait_until_output(&mut self) -> Option<MemItem> {
        while !self.is_stopped() && self.output.is_empty() && !self.waiting_for_input() {
            self.step();
        }
        self.output.pop()
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
            .split(',')
            .map(|code| {
                code.trim()
                    .parse::<MemItem>()
                    .with_context(|| format!("cannot parse {code} as code"))
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

pub struct Ascii {
    computer: Computer,
}
impl FromStr for Ascii {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Ascii {
            computer: s.parse()?,
        })
    }
}

impl Ascii {
    pub fn execute(&mut self, cmd: &str) {
        println!("{cmd}");
        let cmd = cmd.trim();
        for b in cmd.trim().bytes() {
            self.computer.add_input(b as i64);
        }
        self.computer.add_input(10);
    }

    pub fn show_output(&mut self) -> String {
        let mut line = String::new();
        while let Some(o) = self.computer.wait_until_output() {
            if let Ok(b) = u8::try_from(o) {
                write!(&mut line, "{}", b as char).expect("write, dammit");
            } else {
                self.computer.emit_output(o);
                break;
            }
        }
        line
    }

    pub fn non_ascii_output(&mut self) -> Option<i64> {
        self.computer.wait_until_output()
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
        computer.add_input(1);
        computer.run();
        assert_eq!(computer.output, vec![0]);
        let mut computer: Computer = "3,9,8,9,10,9,4,9,99,-1,8".parse()?;
        computer.add_input(8);
        computer.run();
        assert_eq!(computer.output, vec![1]);
        Ok(())
    }

    #[test]
    fn test_less_than_8() -> Result<()> {
        let mut computer: Computer = "3,9,7,9,10,9,4,9,99,-1,8".parse()?;
        computer.add_input(1);
        computer.run();
        assert_eq!(computer.output, vec![1]);
        let mut computer: Computer = "3,9,7,9,10,9,4,9,99,-1,8".parse()?;
        computer.add_input(9);
        computer.run();
        assert_eq!(computer.output, vec![0]);
        Ok(())
    }

    #[test]
    fn test_input_eq_8_imm() -> Result<()> {
        let mut computer: Computer = "3,3,1108,-1,8,3,4,3,99".parse()?;
        computer.add_input(1);
        computer.run();
        assert_eq!(computer.output, vec![0]);
        let mut computer: Computer = "3,3,1108,-1,8,3,4,3,99".parse()?;
        computer.add_input(8);
        computer.run();
        assert_eq!(computer.output, vec![1]);
        Ok(())
    }

    #[test]
    fn test_less_than_8_imm() -> Result<()> {
        let mut computer: Computer = "3,3,1107,-1,8,3,4,3,99".parse()?;
        computer.add_input(1);
        computer.run();
        assert_eq!(computer.output, vec![1]);
        let mut computer: Computer = "3,3,1107,-1,8,3,4,3,99".parse()?;
        computer.add_input(9);
        computer.run();
        assert_eq!(computer.output, vec![0]);
        Ok(())
    }

    #[test]
    fn test_copy() -> Result<()> {
        let mut computer: Computer =
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".parse()?;
        computer.run();
        assert_eq!(
            computer.output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
        Ok(())
    }

    #[test]
    fn test_rebase_output() -> Result<()> {
        let mut computer: Computer = "1102,34915192,34915192,7,4,7,99,0".parse()?;
        computer.run();
        assert_eq!(computer.get_output(), vec![1219070632396864]);
        Ok(())
    }

    #[test]
    fn test_big_number() -> Result<()> {
        let mut computer: Computer = "104,1125899906842624,99".parse()?;
        computer.run();
        assert_eq!(computer.get_output(), vec![1125899906842624]);
        Ok(())
    }
}
