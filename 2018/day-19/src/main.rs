use anyhow::bail;
use lazy_static::lazy_static;

static INPUT: &str = include_str!("input.txt");

type Register = [i64; 6];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Input {
    Immediate,
    Register,
}

impl Input {
    fn eval(&self, opi: i64, register: &Register) -> i64 {
        match self {
            Immediate => opi,
            Input::Register => register[opi as usize],
        }
    }
}

mod functions {
    pub fn add(a: i64, b: i64) -> i64 {
        a + b
    }
    pub fn mul(a: i64, b: i64) -> i64 {
        a * b
    }
    pub fn ban(a: i64, b: i64) -> i64 {
        a & b
    }
    pub fn bor(a: i64, b: i64) -> i64 {
        a | b
    }
    pub fn set(a: i64, _b: i64) -> i64 {
        a
    }
    pub fn gt(a: i64, b: i64) -> i64 {
        (a > b).into()
    }
    pub fn eq(a: i64, b: i64) -> i64 {
        (a == b).into()
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct Operation {
    name: &'static str,
    a: Input,
    b: Input,
    f: fn(i64, i64) -> i64,
}

impl Operation {
    const fn new(name: &'static str, a: Input, b: Input, f: fn(i64, i64) -> i64) -> Self {
        Self { name, a, b, f }
    }
}

use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use Input::*;
static OPERATIONS: [Operation; 16] = [
    Operation::new("addr", Register, Register, functions::add),
    Operation::new("addi", Register, Immediate, functions::add),
    Operation::new("mulr", Register, Register, functions::mul),
    Operation::new("muli", Register, Immediate, functions::mul),
    Operation::new("banr", Register, Register, functions::ban),
    Operation::new("bani", Register, Immediate, functions::ban),
    Operation::new("borr", Register, Register, functions::bor),
    Operation::new("bori", Register, Immediate, functions::bor),
    Operation::new("setr", Register, Immediate, functions::set),
    Operation::new("seti", Immediate, Immediate, functions::set),
    Operation::new("gtir", Immediate, Register, functions::gt),
    Operation::new("gtri", Register, Immediate, functions::gt),
    Operation::new("gtrr", Register, Register, functions::gt),
    Operation::new("eqir", Immediate, Register, functions::eq),
    Operation::new("eqri", Register, Immediate, functions::eq),
    Operation::new("eqrr", Register, Register, functions::eq),
];

lazy_static! {
    static ref MNEMONICS: BTreeMap<&'static str, usize> =
        BTreeMap::from_iter(OPERATIONS.iter().enumerate().map(|(idx, o)| (o.name, idx)));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    opcode: usize,
    a: i64,
    b: i64,
    c: usize,
}

impl Instruction {
    fn eval(&self, register: &Register) -> Register {
        let oper = &OPERATIONS[self.opcode];
        let mut res = *register;
        let a = oper.a.eval(self.a, register);
        let b = oper.b.eval(self.b, register);
        let c = (oper.f)(a, b);
        res[self.c] = c;
        res
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            OPERATIONS[self.opcode].name, self.a, self.b, self.c
        )
    }
}

struct Program {
    ip: usize,
    instructions: Vec<Instruction>,
    patch: bool,
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ip: usize = 0;
        let mut instructions = Vec::new();
        for line in s.lines() {
            if let Some(i) = line.strip_prefix("#ip ") {
                ip = i.parse()?;
            } else {
                let instr: Vec<_> = line.split_whitespace().collect();
                let Some(&opcode) = MNEMONICS.get(instr[0]) else {bail!("Unknown mnemonic {}", instr[0])};
                instructions.push(Instruction {
                    opcode,
                    a: instr[1].parse()?,
                    b: instr[2].parse()?,
                    c: instr[3].parse()?,
                });
            }
        }
        Ok(Self {
            ip,
            instructions,
            patch: false,
        })
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#ip {}", self.ip)?;
        for i in &self.instructions {
            writeln!(f, "{i}")?;
        }
        Ok(())
    }
}

impl Program {
    fn run(&self, register: &mut Register) {
        let mut ip = 0;

        while ip < self.instructions.len() {
            let pre_0 = register[0];
            let pre = register[self.ip];

            if self.patch && ip == 3 {
                let r1 = register[1];
                let r3 = register[3];
                let r4 = register[4];
                let d = r4 / r1;
                if r1 * d == r4 && d >= r3 {
                    register[0] += register[1];
                }

                register[5] = 1;
                register[3] = register[4];
                register[2] = 11;
            } else {
                *register = self.instructions[ip].eval(register);
            }
            //print!("ip={ip} {:?} {} ", register, self.instructions[ip]);

            //println!("{:?}", register);

            let post_0 = register[0];
            if pre_0 != post_0 {
                println!("ip: {ip}, registers: {register:?}");
            }

            let post = register[self.ip];
            if pre != post {
                ip = post as usize;
            }
            ip += 1;
            register[self.ip] = ip as i64;
        }
    }

    fn patched(mut self) -> Self {
        self.patch = true;
        self
    }
}

fn part_01(input: &str) -> i64 {
    let p = Program::from_str(input).unwrap();
    let mut register = [0; 6];
    p.run(&mut register);
    register[0]
}

fn part_02(input: &str) -> i64 {
    let p = Program::from_str(input).unwrap().patched();
    let mut register = [0; 6];
    register[0] = 1;
    p.run(&mut register);
    register[0]
}

fn main() {
    println!("part 1: {}", part_01(INPUT));
    println!("part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod tests {

    use test_case::test_case;

    use crate::{part_01, part_02, INPUT};

    static TEST_INPUT: &str = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test_case(TEST_INPUT, 7)]
    #[test_case(INPUT, 1248)]
    fn test_01(input: &str, reg0: i64) {
        assert_eq!(reg0, part_01(input));
    }

    #[test_case(INPUT, 14952912)]
    fn test_02(input: &str, reg0: i64) {
        assert_eq!(reg0, part_02(input));
    }
}
