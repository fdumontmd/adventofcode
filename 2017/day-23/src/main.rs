use std::fmt::Display;

const INPUT: &str = include_str!("input.txt");

#[derive(Default, Debug)]
struct Registers([isize; 8]);

impl Registers {
    fn read_register(&self, r: u8) -> isize {
        self.0[r as usize]
    }

    fn store_register(&mut self, r: u8, v: isize) {
        self.0[r as usize] = v;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
enum Value {
    Register(u8),
    Immediate(isize),
}

impl Value {
    fn get_value(&self, registers: &Registers) -> isize {
        match self {
            Value::Register(r) => registers.read_register(*r),
            Value::Immediate(i) => *i,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Register(r) => write!(f, "{}", (r + b'a') as char),
            Value::Immediate(i) => write!(f, "{i}"),
        }
    }
}

enum Instructions {
    Set(u8, Value),
    Sub(u8, Value),
    Mul(u8, Value),
    Jnz(Value, Value),
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Set(r, value) => write!(f, "set {} {value}", (r + b'a') as char),
            Instructions::Sub(r, value) => write!(f, "sub {} {value}", (r + b'a') as char),
            Instructions::Mul(r, value) => write!(f, "mul {} {value}", (r + b'a') as char),
            Instructions::Jnz(value, value1) => write!(f, "jnz {value} {value1}"),
        }
    }
}

fn parse_register(input: &str) -> u8 {
    input.chars().next().unwrap() as u8 - b'a'
}

fn parse_value(input: &str) -> Value {
    match input.parse() {
        Ok(i) => Value::Immediate(i),
        Err(_) => Value::Register(parse_register(input)),
    }
}

impl Instructions {
    fn from_str(input: &str) -> Self {
        let parts: Vec<_> = input.split_whitespace().collect();
        match parts[0] {
            "set" => Instructions::Set(parse_register(parts[1]), parse_value(parts[2])),
            "sub" => Instructions::Sub(parse_register(parts[1]), parse_value(parts[2])),
            "mul" => Instructions::Mul(parse_register(parts[1]), parse_value(parts[2])),
            "jnz" => Instructions::Jnz(parse_value(parts[1]), parse_value(parts[2])),
            _ => panic!("cannot parse instruction {input}"),
        }
    }

    fn execute(&self, registers: &mut Registers) -> isize {
        match self {
            Instructions::Set(r, value) => {
                registers.store_register(*r, value.get_value(registers));
                1
            }
            Instructions::Sub(r, value) => {
                let v = registers.read_register(*r) - value.get_value(registers);
                registers.store_register(*r, v);
                1
            }
            Instructions::Mul(r, value) => {
                let v = registers.read_register(*r) * value.get_value(registers);
                registers.store_register(*r, v);
                1
            }
            Instructions::Jnz(value, value1) => {
                let v = value.get_value(registers);
                if v != 0 {
                    value1.get_value(registers)
                } else {
                    1
                }
            }
        }
    }
}

struct Program(Vec<Instructions>);

impl Program {
    fn from_str(input: &str) -> Self {
        Self(input.lines().map(Instructions::from_str).collect())
    }
}

struct Part1Computer<'a> {
    ip: usize,
    registers: Registers,
    program: &'a Program,
    mul_count: usize,
}

impl<'a> Part1Computer<'a> {
    fn new(program: &'a Program) -> Self {
        Self {
            ip: 0,
            registers: Registers::default(),
            program,
            mul_count: 0,
        }
    }

    fn step(&mut self) -> bool {
        if self.ip >= self.program.0.len() {
            false
        } else {
            if matches!(self.program.0[self.ip], Instructions::Mul(_, _)) {
                self.mul_count += 1;
            }
            self.ip = self
                .ip
                .checked_add_signed(self.program.0[self.ip].execute(&mut self.registers))
                .unwrap_or(self.program.0.len());

            true
        }
    }
}

fn part1(input: &str) -> usize {
    let program = Program::from_str(input);
    let mut computer = Part1Computer::new(&program);

    while computer.step() {}

    computer.mul_count
}

fn is_prime(n: u64) -> bool {
    for d in 2..=n.isqrt() {
        if n % d == 0 {
            return false;
        }
    }
    true
}

fn part2() -> isize {
    // code analysis of the input shows it computes the number of non-prime numbers between 105700
    // and 105700 + 17000 but checking only by increment of 17
    // can't really "execute" anything on that computer that would compute the same value
    // so just (re)write in rust

    let mut count = 0;
    for step in 0..=1000 {
        let n = 105700 + step * 17;
        if !is_prime(n) {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3025, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(915, part2());
    }
}
