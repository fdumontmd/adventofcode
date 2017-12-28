use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
enum Register {
    A,
    B,
}

impl Register {
    fn read_registry(line: &str) -> Register {
        match &line[4..5] {
            "a" => Register::A,
            "b" => Register::B,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Registers(u64, u64);

impl<'a> Index<&'a Register> for Registers {
    type Output = u64;

    fn index(&self, register: &'a Register) -> &Self::Output {
        match register {
            &Register::A => &self.0,
            &Register::B => &self.1,
        }
    }
}

impl<'a> IndexMut<&'a Register> for Registers {
    fn index_mut(&mut self, register: &'a Register) -> &mut Self::Output {
        match register {
            &Register::A => &mut self.0,
            &Register::B => &mut self.1,
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Registers(0, 0)
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

#[derive(Clone, Debug)]
struct Computer {
    instructions: Vec<Instruction>,
    registers: Registers,
}

impl Computer {
    fn new() -> Self {
        Computer {
            instructions: Vec::new(),
            registers: Registers::default(),
        }
    }
    fn add(&mut self, i: Instruction) -> &mut Self {
        self.instructions.push(i);
        self
    }

    fn run(&mut self) {
        let mut ip = 0;

        loop {
            if ip >= self.instructions.len() {
                break;
            }
            //println!("IP = {} ; REG: {:?} ; EXEC = {:?}", ip, self.registers, self.instructions[ip]);
            match &self.instructions[ip] {
                &Instruction::Hlf(ref r) => {
                    self.registers[r] /= 2;
                }
                &Instruction::Tpl(ref r) => {
                    self.registers[r] *= 3;
                }
                &Instruction::Inc(ref r) => {
                    self.registers[r] += 1;
                }
                &Instruction::Jmp(o) => {
                    ip = (ip as isize + o) as usize;
                    continue;
                }
                &Instruction::Jie(ref r, o) => {
                    if self.registers[r] % 2 == 0 {
                        ip = (ip as isize + o) as usize;
                        continue;
                    }
                }
                &Instruction::Jio(ref r, o) => {
                    if self.registers[r] == 1 {
                        ip = (ip as isize + o) as usize;
                        continue;
                    }
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

    let mut cpu = Computer::new();

    for line in buf.lines() {
        let line = line.unwrap();

        cpu.add(match &line[..3] {
            "inc" => {
                Instruction::Inc(Register::read_registry(&line))
            }
            "hlf" => {
                Instruction::Hlf(Register::read_registry(&line))
            }
            "tpl" => {
                Instruction::Tpl(Register::read_registry(&line))
            }
            "jmp" => {
                Instruction::Jmp(line[4..].parse().unwrap())
            }
            "jie" => {
                Instruction::Jie(Register::read_registry(&line), line[7..].parse().unwrap())
            }
            "jio" => {
                Instruction::Jio(Register::read_registry(&line), line[7..].parse().unwrap())
            }
            _ => unreachable!(),
        });
    }

    //println!("{:?}", cpu);

    let tmp = cpu.clone();

    cpu.run();

    println!("A = {}, B = {}", cpu.registers[&Register::A], cpu.registers[&Register::B]);

    let mut cpu = tmp;

    cpu.registers[&Register::A] = 1;

    cpu.run();

    println!("A = {}, B = {}", cpu.registers[&Register::A], cpu.registers[&Register::B]);

}

#[test]
fn test() {
    let mut cpu = Computer::new();
    cpu
        .add(Instruction::Inc(Register::A))
        .add(Instruction::Jio(Register::A, 2))
        .add(Instruction::Tpl(Register::A))
        .add(Instruction::Inc(Register::A));

    cpu.run();

    assert_eq!(cpu.registers[&Register::A], 2);
    assert_eq!(cpu.registers[&Register::B], 0);
}
