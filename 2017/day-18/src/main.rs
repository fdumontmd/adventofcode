use std::{collections::VecDeque, fmt::Display};

const INPUT: &str = include_str!("input.txt");

enum Value {
    Reg(u8),
    Imm(i64),
}

impl Value {
    fn evaluate(&self, computer: &Computer) -> i64 {
        match self {
            Value::Reg(r) => computer.get_register_value(*r),
            Value::Imm(i) => *i,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Reg(r) => write!(f, "{}", (r + b'a') as char),
            Value::Imm(i) => write!(f, "{i}"),
        }
    }
}

enum Instructions {
    Set(u8, Value),
    Add(u8, Value),
    Mul(u8, Value),
    Mod(u8, Value),
    Snd(Value),
    Rcv(u8),
    Jgz(Value, Value),
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Set(r, value) => write!(f, "set {} {value}", Value::Reg(*r)),
            Instructions::Add(r, value) => write!(f, "add {} {value}", Value::Reg(*r)),
            Instructions::Mul(r, value) => write!(f, "mul {} {value}", Value::Reg(*r)),
            Instructions::Mod(r, value) => write!(f, "mod {} {value}", Value::Reg(*r)),
            Instructions::Snd(v) => write!(f, "snd {v}"),
            Instructions::Rcv(r) => write!(f, "rcv {}", Value::Reg(*r)),
            Instructions::Jgz(value, value1) => write!(f, "jgz {value} {value1}"),
        }
    }
}

fn parse_register(input: &str) -> u8 {
    input.as_bytes()[0] - b'a'
}

fn parse_value(input: &str) -> Value {
    match input.parse() {
        Ok(i) => Value::Imm(i),
        Err(_) => Value::Reg(parse_register(input)),
    }
}

impl Instructions {
    fn parse(input: &str) -> Self {
        let parts: Vec<_> = input.split_whitespace().collect();
        match parts[0] {
            "set" => Instructions::Set(parse_register(parts[1]), parse_value(parts[2])),
            "add" => Instructions::Add(parse_register(parts[1]), parse_value(parts[2])),
            "mul" => Instructions::Mul(parse_register(parts[1]), parse_value(parts[2])),
            "mod" => Instructions::Mod(parse_register(parts[1]), parse_value(parts[2])),
            "snd" => Instructions::Snd(parse_value(parts[1])),
            "rcv" => Instructions::Rcv(parse_register(parts[1])),
            "jgz" => Instructions::Jgz(parse_value(parts[1]), parse_value(parts[2])),
            _ => panic!("cannot parse {input}"),
        }
    }

    fn execute(&self, c: &mut Computer, io: &mut impl IO) -> i64 {
        match self {
            Instructions::Set(r, value) => {
                c.store_register_value(*r, value.evaluate(c));
                1
            }
            Instructions::Add(r, value) => {
                c.store_register_value(*r, c.get_register_value(*r) + value.evaluate(c));
                1
            }
            Instructions::Mul(r, value) => {
                c.store_register_value(*r, c.get_register_value(*r) * value.evaluate(c));
                1
            }

            Instructions::Mod(r, value) => {
                c.store_register_value(*r, c.get_register_value(*r) % value.evaluate(c));
                1
            }

            Instructions::Snd(v) => {
                io.send(v.evaluate(c));
                1
            }
            Instructions::Rcv(r) => match io.rcv(c.get_register_value(*r)) {
                Some(v) => {
                    c.store_register_value(*r, v);
                    1
                }
                None => 0,
            },
            Instructions::Jgz(value, value1) => {
                let t = value.evaluate(c);
                if t > 0 { value1.evaluate(c) } else { 1 }
            }
        }
    }
}

struct Program(Vec<Instructions>);

impl Program {
    fn parse(input: &str) -> Self {
        Program(input.lines().map(Instructions::parse).collect())
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            writeln!(f, "{i}")?;
        }
        Ok(())
    }
}

trait IO {
    fn send(&mut self, v: i64);
    fn rcv(&mut self, v: i64) -> Option<i64>;
}

// don't store the program in the computer as it's fixed; the Computer is just for changing data
struct Computer {
    ip: usize,
    registers: [i64; 26],
}

impl Computer {
    fn new() -> Self {
        Computer {
            ip: 0,
            registers: [0; 26],
        }
    }

    fn get_register_value(&self, r: u8) -> i64 {
        self.registers[r as usize]
    }

    fn store_register_value(&mut self, r: u8, v: i64) {
        self.registers[r as usize] = v
    }

    fn step(&mut self, program: &Program, io: &mut impl IO) -> bool {
        if self.ip >= program.0.len() {
            false
        } else {
            let s = program.0[self.ip].execute(self, io);
            self.ip = self.ip.checked_add_signed(s as isize).unwrap();
            s != 0
        }
    }

    fn init_computer_id(&mut self, id: i64) {
        self.store_register_value(b'p' - b'a', id);
    }
}

struct Part1IO {
    sound: Option<i64>,
    read: bool,
}

impl Part1IO {
    fn new() -> Self {
        Self {
            sound: None,
            read: false,
        }
    }
}

impl IO for Part1IO {
    fn send(&mut self, v: i64) {
        self.sound = Some(v);
    }

    fn rcv(&mut self, v: i64) -> Option<i64> {
        if v == 0 {
            None
        } else {
            self.read = self.read || self.sound.is_some();
            self.sound
        }
    }
}

fn part1(program: &Program) -> i64 {
    let mut computer = Computer::new();
    let mut part1_io = Part1IO::new();

    while computer.step(program, &mut part1_io) {
        if part1_io.read {
            break;
        }
    }

    part1_io.sound.unwrap()
}

struct IOQueue {
    sends: usize,
    queue: VecDeque<i64>,
}

impl IOQueue {
    fn new() -> Self {
        Self {
            sends: 0,
            queue: VecDeque::new(),
        }
    }

    fn send(&mut self, i: i64) {
        self.sends += 1;
        self.queue.push_front(i)
    }
}

struct Part2IO<'a> {
    send: &'a mut IOQueue,
    receive: &'a mut IOQueue,
}

impl<'a> Part2IO<'a> {
    fn new(send: &'a mut IOQueue, receive: &'a mut IOQueue) -> Self {
        Self { send, receive }
    }
}

impl IO for Part2IO<'_> {
    fn send(&mut self, v: i64) {
        self.send.send(v)
    }

    fn rcv(&mut self, _: i64) -> Option<i64> {
        self.receive.queue.pop_back()
    }
}

fn part2(program: &Program) -> usize {
    let mut queue0 = IOQueue::new();
    let mut queue1 = IOQueue::new();

    let mut c0 = Computer::new();
    c0.init_computer_id(0);
    let mut c1 = Computer::new();
    c1.init_computer_id(1);

    loop {
        let mut progress = false;

        // computer 0 sends into queue1 and receives from queue0
        let mut io = Part2IO::new(&mut queue1, &mut queue0);
        while c0.step(program, &mut io) {
            progress = true;
        }

        // computer 1 sends into queue0 and receives from queue1
        let mut io = Part2IO::new(&mut queue0, &mut queue1);
        while c1.step(program, &mut io) {
            progress |= true;
        }

        if !progress {
            break;
        }
    }

    queue0.sends
}

fn main() {
    let program = Program::parse(INPUT);
    println!("part1: {}", part1(&program));
    println!("part2: {}", part2(&program));
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    const TEST_INPUT_1: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";
    #[test_case(TEST_INPUT_1, 4)]
    #[test_case(INPUT, 8600)]
    fn test_part1(input: &str, sound: i64) {
        let program = Program::parse(input);
        assert_eq!(sound, part1(&program));
    }

    const TEST_INPUT_2: &str = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    #[test_case(TEST_INPUT_2, 3)]
    #[test_case(INPUT, 7239)]
    fn test_part2(input: &str, sends: usize) {
        let program = Program::parse(input);
        assert_eq!(sends, part2(&program));
    }
}
