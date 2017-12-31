extern crate aoc_utils;

use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl Instruction {
    fn apply(&self, input: &mut Vec<u8>) {
        use Instruction::*;
        match self {
            &Spin(mid) => {
                let mut v = Vec::with_capacity(input.len());
                {
                    // scope the immutable borrow
                    let (last, first) = input.split_at(input.len() - mid);
                    v.extend_from_slice(first);
                    v.extend_from_slice(last);
                }
                input.clone_from(&v);
            }
            &Exchange(pos1, pos2) => input.swap(pos1, pos2),
            &Partner(p1, p2) => {
                let pos1 = input.iter().position(|p| *p == p1).unwrap();
                let pos2 = input.iter().position(|p| *p == p2).unwrap();
                input.swap(pos1, pos2);
            }
        }
    }
}

struct InstructionParsingError(String);

impl FromStr for Instruction {
    type Err = InstructionParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(InstructionParsingError("empty input".to_owned()));
        }
        let (i, params) = s.split_at(1);
        match i {
            "s" => {
                if let Ok(pos) = params.parse() {
                    return Ok(Instruction::Spin(pos))
                }
            }
            "x" => {
                let params: Vec<&str> = params.split('/').collect();
                if params.len() == 2 {
                    if let Ok(pos1) = params[0].parse() {
                        if let Ok(pos2) = params[1].parse() {
                            return Ok(Instruction::Exchange(pos1, pos2));
                        }
                    }
                }
            }
            "p" => {
                let params: Vec<&str> = params.split('/').collect();
                if params.len() == 2 {
                    if params[0].len() == 1 && params[1].len() == 1 {
                        return Ok(Instruction::Partner(params[0].as_bytes()[0],
                                                       params[1].as_bytes()[0]));
                    }
                }
            }
            _ => {}
        };
        Err(InstructionParsingError(s.to_owned()))
    }
}

fn main() {
    let mut instructions = Vec::new();
    let mut steps = Vec::new();
    use std::io::BufRead;
    for line in aoc_utils::get_input().lines() {
        let line = line.unwrap();
        for i in line.split(',') {
            if let Ok(i) = i.parse::<Instruction>() {
                instructions.push(i);
            } else {
                panic!("Cannot parse {}", i);
            }
        }
    }
    let mut input: Vec<u8> = b"abcdefghijklmnop".iter().cloned().collect();
    steps.push(input.clone());

    loop {
        for i in &instructions {
            i.apply(&mut input);
        }
        if input == steps[0] {
            break;
        }
        steps.push(input.clone());
    };

    let final_step = 1_000_0000_000 % steps.len();

    println!("Steps length: {}", steps.len());
    println!("First step: {}", String::from_utf8_lossy(&steps[0]));
    println!("Last step: {}", String::from_utf8_lossy(&steps[steps.len() - 1]));

    println!("After first dance: {}", String::from_utf8_lossy(&steps[1]));
    println!("After final dance: {}", String::from_utf8_lossy(&steps[final_step]));
}

#[test]
fn test_basic() {
    let mut input = b"abcde".iter().cloned().collect();

    Instruction::Spin(1).apply(&mut input);
    assert_eq!(input, b"eabcd");
    Instruction::Exchange(3, 4).apply(&mut input);
    assert_eq!(input, b"eabdc");
    Instruction::Partner(b'e', b'b').apply(&mut input);
    assert_eq!(input, b"baedc");
}
