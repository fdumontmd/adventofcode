use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Value {
    Register(String),
    Number(i32),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Cpy,
    Jnz,
    Inc,
    Dec,
    Tgl,
}

#[derive(Clone, Debug)]
enum Instruction {
    Unary(Op, Value),
    Binary(Op, Value, Value),
}

struct Computer {
    instructions: Vec<Instruction>,
    registers: HashMap<String, i32>,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Computer {
            instructions,
            registers: HashMap::new(),
        }
    }

    fn get(&self, register: &str) -> i32 {
        *self.registers.get(register).or(Some(&0)).unwrap()
    }

    fn set(&mut self, register: String, value: i32) {
        self.registers.insert(register, value);
    }

    fn get_value(&self, v: &Value) -> i32 {
        match *v {
            Value::Number(i) => i,
            Value::Register(ref s) => {
                match self.registers.get(s) {
                    Some(i) => *i,
                    _ => 0
                }
            }
        }
    }

    fn run(&mut self) {
        let mut ip = 0;

        loop {
            if ip >= self.instructions.len() {
                break;
            }

            let mut ti = None;

            use Instruction::*;
            match &self.instructions[ip] {
                &Binary(Op::Cpy, ref v, ref t) => {
                    match t {
                        &Value::Register(ref s) => {
                            let v = self.get_value(&v);
                            self.registers.insert(s.clone(), v);
                        }
                        _ => {}
                    }
                }
                &Unary(Op::Inc, ref v) => {
                    match v {
                        &Value::Register(ref s) => {
                            *self.registers.entry(s.clone()).or_insert(0) += 1;
                        }
                        _ => {}
                    }
                }
                &Unary(Op::Dec, ref v) => {
                    match v {
                        &Value::Register(ref s) => {
                            *self.registers.entry(s.to_owned()).or_insert(0) -= 1;
                        }
                        _ => {}
                    }
                }
                &Binary(Op::Jnz, ref v, ref o) => {
                    let offset = self.get_value(o);
                    let v = self.get_value(v);
                    if v != 0 {
                        let mut iip = ip as i32;
                        if iip + offset < 0 {
                            break;
                        }
                        iip += offset;
                        assert!(iip > 0);
                        ip = iip as usize;
                        continue;
                    }
                }
                &Unary(Op::Tgl, ref v) => {
                    ti = Some(ip as i32 + self.get_value(v));
                }
                _ => unreachable!(),
            }
            if let Some(ti) = ti {
                if ti >= 0 && ti < self.instructions.len() as i32 {
                    let ti = ti as usize;
                    match &mut self.instructions[ti] {
                        &mut Binary(ref mut op, _, _) => {
                            if *op == Op::Cpy {
                                *op = Op::Jnz;
                            } else {
                                *op = Op::Cpy;
                            }
                        }
                        &mut Unary(ref mut op, _) => {
                            if *op == Op::Inc {
                                *op = Op::Dec;
                            } else {
                                *op = Op::Inc;
                            }
                        }
                    };
                }
            }
            ip += 1;
        }
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::env::args;

    assert!(args().len() > 1);
    let path = args().nth(1).unwrap();
    let input = File::open(&path).unwrap();
    let buf = BufReader::new(input);

    fn parse_arg(s: &str) -> Value {
        match s.parse::<i32>() {
            Ok(i) => Value::Number(i),
            Err(_) => Value::Register(s.to_owned())
        }
    }

    let mut instructions = Vec::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.split_whitespace().collect();
        use Instruction::*;
        let i = match v[0] {
            "cpy" => {
                Binary(Op::Cpy, parse_arg(v[1]), parse_arg(v[2]))
            }
            "jnz" => {
                Binary(Op::Jnz, parse_arg(v[1]), parse_arg(v[2]))
            }
            "inc" => {
                Unary(Op::Inc, parse_arg(v[1]))
            }
            "dec" => {
                Unary(Op::Dec, parse_arg(v[1]))
            }
            "tgl" => {
                Unary(Op::Tgl, parse_arg(v[1]))
            }
            _ => unreachable!(),
        };

        instructions.push(i);
    }

    let mut cpu = Computer::new(instructions.clone());
    cpu.set("a".to_owned(), 7);
    cpu.run();
    println!("For input = 7; a == {}", cpu.get("a"));

    let mut cpu = Computer::new(instructions.clone());
    cpu.set("a".to_owned(), 12);
    cpu.run();
    println!("For input = 12, a == {}", cpu.get("a"));
}

#[test]
fn test() {
    let mut instructions = Vec::new();
    instructions.push(Instruction::Binary(Op::Cpy, Value::Number(2), Value::Register("a".to_owned())));
    instructions.push(Instruction::Unary(Op::Tgl, Value::Register("a".to_owned())));
    instructions.push(Instruction::Unary(Op::Tgl, Value::Register("a".to_owned())));
    instructions.push(Instruction::Unary(Op::Tgl, Value::Register("a".to_owned())));
    instructions.push(Instruction::Binary(Op::Cpy, Value::Number(1), Value::Register("a".to_owned())));
    instructions.push(Instruction::Unary(Op::Dec, Value::Register("a".to_owned())));
    instructions.push(Instruction::Unary(Op::Dec, Value::Register("a".to_owned())));

    let mut cpu = Computer::new(instructions);

    cpu.run();
    assert_eq!(cpu.get("a".to_owned()), 3);
}
