use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone, Debug)]
enum Value {
    Immediate(i64),
    Register(Register)
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Copy(Value, Register),
    Inc(Register),
    Dec(Register),
    JNZ(Value, Value),
    OUT(Register),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct CPU {
    ip: usize,
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    out: Option<i64>,
}

impl CPU {
    fn new() -> Self {
        CPU {ip: 0, a: 0, b: 0, c: 0, d: 0, out: None}
    }

    fn get_value(&self, value: &Value) -> i64 {
        match *value {
            Value::Immediate(i) => i,
            Value::Register(r) => {
                match r {
                    Register::A => self.a,
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                }
            }
        }
    }

    fn update_register(self, register: Register, value: i64) -> Self {
        match register {
            Register::A => CPU { a: value, ..self },
            Register::B => CPU { b: value, ..self },
            Register::C => CPU { c: value, ..self },
            Register::D => CPU { d: value, ..self },
        }
    }

    fn next_ip(self) -> Self {
        CPU { ip: self.ip + 1, out: None, .. self}
    }

    fn next_ip_with(self, out: i64) -> Self {
        CPU { ip: self.ip + 1, out: Some(out), .. self }
    }

    fn execute(self, program: &Program) -> Self {
        if self.ip < program.len() {
            use Instruction::*;
            match &program[self.ip] {
                &Copy(ref v, r) => {
                    self.update_register(r, self.get_value(v)).next_ip()
                }
                &Inc(r) => {
                    self.update_register(r,
                                         self.get_value(&Value::Register(r)) + 1)
                        .next_ip()
                }
                &Dec(r) => {
                    self.update_register(r,
                                         self.get_value(&Value::Register(r)) - 1)
                        .next_ip()
                }
                &JNZ(ref v1, v) => {
                    if self.get_value(v1) != 0 {
                        let shift = self.get_value(&v);
                        let new_ip = ( (self.ip as i64) + shift ) as usize;
                        CPU { ip: new_ip, out: None, ..self }
                    } else {
                        self.next_ip()
                    }
                }
                &OUT(r) => {
                    let out = self.get_value(&Value::Register(r));
                    self.next_ip_with(out)
                }
            }
        } else {
            self
        }
    }
}

type Program = Vec<Instruction>;

#[derive(Clone, Debug)]
struct Computer<'p> {
    cpu: CPU,
    program: &'p Program,
    previous_states: HashMap<CPU, usize>,
    emitted: Vec<i64>,
}

enum Outcome {
    Loop(usize),
    Halted,
    Continue,
}

impl<'p> Computer<'p> {
    fn new(program: &'p Program) -> Self {
        Computer {
            cpu: CPU::new(),
            program,
            previous_states: HashMap::new(),
            emitted: Vec::new(),
        }
    }

    fn halted(&self) -> bool {
        self.cpu.ip >= self.program.len()
    }

    fn run(&mut self) {
        loop {
            if self.halted() {
                break;
            }
            self.cpu = self.cpu.execute(&self.program);
        }
    }

    fn step(&mut self) -> Outcome {
        if self.halted() {
            Outcome::Halted
        } else {
            let new_state = self.cpu.execute(&self.program);
            if self.previous_states.contains_key(&new_state) {
                Outcome::Loop(self.emitted.len())
            } else {
                if new_state.out.is_some() {
                    let out = new_state.out.unwrap();
                    if out == 0 || out == 1 {
                        self.emitted.push(new_state.out.unwrap());
                    } else {
                        return Outcome::Halted;
                    }
                }
                self.previous_states.insert(new_state.clone(), self.emitted.len());
                self.cpu = new_state;
                Outcome::Continue
            }
        }
    }

    fn signal_with(program: &Program, a: i64) -> Option<(usize, Vec<i64>)> {
        let mut computer = Computer::new(program);
        computer.cpu.a = a;

        loop {
            match computer.step() {
                Outcome::Halted => {
                    return None;
                }
                Outcome::Loop(u) => {
                    return Some((u, computer.emitted));
                }
                Outcome::Continue => {}
            }
        }
    }
}

fn parse_value(value: &str) -> Value {
    match i64::from_str(value) {
        Ok(i) => Value::Immediate(i),
        Err(_) => Value::Register(parse_register(value))
    }
}

fn parse_register(register: &str) -> Register {
    if "a" == register {
        Register::A
    } else if "b" == register {
        Register::B
    } else if "c" == register {
        Register::C
    } else if "d" == register {
        Register::D
    } else {
        println!("Unkown register [{}]", register);
        unreachable!()
    }
}

fn parse_instruction(instr: &str) -> Instruction {
    let mut iter = instr.split_whitespace();
    let code = iter.next().unwrap();
    if code == "cpy" {
        Instruction::Copy(parse_value(iter.next().unwrap()),
                          parse_register(iter.next().unwrap()))
    } else if code == "inc" {
        Instruction::Inc(parse_register(iter.next().unwrap()))
    } else if code == "dec" {
        Instruction::Dec(parse_register(iter.next().unwrap()))
    } else if code == "jnz" {
        Instruction::JNZ(parse_value(iter.next().unwrap()),
                        parse_value(iter.next().unwrap()))
    } else if code == "out" {
        Instruction::OUT(parse_register(iter.next().unwrap()))
    } else {
        println!("Unknown OP code [{}]", code);
        unreachable!()
    }
}


fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut program = Vec::new();

    for line in buffer.lines() {
        program.push(parse_instruction(line.trim()));
    }

    for a in 0.. {
        if let Some((u, emitted)) = Computer::signal_with(&program, a) {
            if emitted.iter().cloned().zip([0, 1].iter().cloned().cycle()).all(|(a, b)| a == b) {
                println!("{}  -> {:?} looping from {}", a, emitted, u);

                if (emitted.len() - u ) % 2 == 0 {
                    break;
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut program = Vec::new();
    program.push(Instruction::Copy(Value::Immediate(41), Register::A));
    program.push(Instruction::Inc(Register::A));
    program.push(Instruction::Inc(Register::A));
    program.push(Instruction::Dec(Register::A));
    program.push(Instruction::JNZ(Value::Register(Register::A), Value::Immediate(2)));
    program.push(Instruction::Dec(Register::A));

    let mut computer = Computer::new(&program);
    computer.run();

    assert_eq!(computer.cpu.a, 42);
}
