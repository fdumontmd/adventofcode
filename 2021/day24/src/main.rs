use std::collections::{BTreeSet, VecDeque};

static INPUT: &str = include_str!("input.txt");

enum Value {
    Immediate(i64),
    Register(usize),
}

enum Instruction {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
}

struct Program {
    instructions: Vec<Instruction>,
    register_names: Vec<String>,
}

impl Program {
    fn from_input(input: &str) -> Self {
        let mut register_names = BTreeSet::new();
        for line in input.lines() {
            let parts = Vec::from_iter(line.split_whitespace());
            register_names.insert(parts[1].to_owned());
        }
        let register_names = Vec::from_iter(register_names);

        let mut instructions = vec![];

        for line in input.lines() {
            let parts = Vec::from_iter(line.split_whitespace());

            let Some(register) = register_names.iter().position(|r| r == parts[1]) else { panic!("cannot find register {} in instruction {}", parts[1], line)};

            let instr = if parts[0] == "inp" {
                Instruction::Inp(register)
            } else {
                let value = if let Ok(i) = parts[2].parse::<i64>() {
                    Value::Immediate(i)
                } else {
                    let Some(register) = register_names.iter().position(|r| r == parts[2]) else { panic!("cannot find register {} in instruction {}", parts[2], line) };
                    Value::Register(register)
                };
                match parts[0] {
                    "add" => Instruction::Add(register, value),
                    "mul" => Instruction::Mul(register, value),
                    "div" => Instruction::Div(register, value),
                    "mod" => Instruction::Mod(register, value),
                    "eql" => Instruction::Eql(register, value),
                    _ => panic!("cannot parse instruction {line}"),
                }
            };
            instructions.push(instr);
        }

        Self {
            register_names,
            instructions,
        }
    }
}

struct ALU<'a> {
    registers: Vec<i64>,
    inp_limit: usize,
    input: VecDeque<i64>,
    program: &'a Program,
}

impl<'a> ALU<'a> {
    fn new(program: &'a Program) -> Self {
        ALU {
            registers: vec![0; program.register_names.len()],
            inp_limit: usize::MAX,
            input: VecDeque::new(),
            program,
        }
    }

    fn register_value(&self, register_name: &str) -> i64 {
        let register = self
            .program
            .register_names
            .iter()
            .position(|r| r == register_name)
            .unwrap();
        self.registers[register]
    }

    fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    fn eval(&self, value: &Value) -> i64 {
        match value {
            Value::Immediate(i) => *i,
            Value::Register(r) => self.registers[*r],
        }
    }

    fn step(&mut self, ip: usize) -> bool {
        match self.program.instructions[ip] {
            Instruction::Inp(ref r) => {
                if self.inp_limit == 0 {
                    return false;
                }

                self.registers[*r] = self.input.pop_front().unwrap();
                self.inp_limit -= 1;
            }
            Instruction::Add(ref r, ref v) => self.registers[*r] += self.eval(v),
            Instruction::Mul(ref r, ref v) => self.registers[*r] *= self.eval(v),
            Instruction::Div(ref r, ref v) => self.registers[*r] /= self.eval(v),
            Instruction::Mod(ref r, ref v) => self.registers[*r] %= self.eval(v),
            Instruction::Eql(ref r, ref v) => {
                self.registers[*r] = (self.registers[*r] == self.eval(v)) as i64
            }
        }
        true
    }

    fn run(&mut self) -> bool {
        self.inp_limit = self.input.len();
        for ip in 0..self.program.instructions.len() {
            if !self.step(ip) {
                return false;
            }
        }
        true
    }
}

// ok, there's a relation between digits based on how
// z is used (a stack, using multiples of 26 to push values)
// each set of instructions after inp follow a pattern:
// x is set to value from top of the stack (z % 26)
// if div z 26, means pop stack, otherwise keep
// if add x positive value, means we don't can't match x against next digit
// if add x negative value, we're expected to match x against next digits
// otherwise z != 0 as we don't pop enough
// then, if x is not same as w, we push new digit + value into z
// so based on those values that are pushed into z, and using the fact
// that add x -xxx means we're expected to match, we can build relations
// between numbers
//
// for my input:
// d14 = d1 - 1
// d13 = d2 + 1
// d12 = d11 + 2
// d10 = d9
// d8 = d3 + 5
// d7 = d6 - 4
// d5 = 1
// d4 = 9

fn part_1(input: &str) -> i64 {
    let program = Program::from_input(input);

    const V: i64 = 98491959997994;

    let mut alu = ALU::new(&program);
    alu.add_input(9);
    alu.add_input(8);
    alu.add_input(4);
    alu.add_input(9);
    alu.add_input(1);
    alu.add_input(9);
    alu.add_input(5);
    alu.add_input(9);
    alu.add_input(9);
    alu.add_input(9);
    alu.add_input(7);
    alu.add_input(9);
    alu.add_input(9);
    alu.add_input(4);

    alu.run();
    let x = alu.register_value("x");
    let y = alu.register_value("y");
    let z = alu.register_value("z");
    println!("x = {}, y = {}, z = {}", x, y, z);
    assert_eq!(z, 0);
    V
}

fn part_2(input: &str) -> i64 {
    let program = Program::from_input(input);

    const V: i64 = 61191516111321;

    let mut alu = ALU::new(&program);
    alu.add_input(6);
    alu.add_input(1);
    alu.add_input(1);
    alu.add_input(9);
    alu.add_input(1);
    alu.add_input(5);
    alu.add_input(1);
    alu.add_input(6);
    alu.add_input(1);
    alu.add_input(1);
    alu.add_input(1);
    alu.add_input(3);
    alu.add_input(2);
    alu.add_input(1);

    alu.run();
    let x = alu.register_value("x");
    let y = alu.register_value("y");
    let z = alu.register_value("z");
    println!("x = {}, y = {}, z = {}", x, y, z);
    assert_eq!(z, 0);
    V
}

// ok, original idea is a bust
// need to work backward
// assume z is 0, then workout what values
// the other variables must have to keep that
// also, start with 0 everywhere
// ?? search
fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}
