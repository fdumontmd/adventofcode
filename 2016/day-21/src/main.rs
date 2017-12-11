extern crate regex;
extern crate permutohedron;

use std::io::{self,Read};
use std::str::FromStr;

use permutohedron::Heap;
use regex::Regex;

const INPUT: &'static str = "abcdefgh";
const TARGET: &'static str = "fbgdceah";

#[derive(Copy, Clone)]
enum Location {
    Letter(u8),
    Index(usize),
}

impl Location {
    fn to_index(&self, data: &[u8]) -> usize {
        match *self {
            Location::Letter(c) => data.iter().position(|b| *b == c).unwrap(),
            Location::Index(pos) => pos,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

enum Command {
    Swap(Location, Location),
    Reverse(Location, Location),
    Rotate(Direction, usize),
    RotateBasedOn(Location),
    Move(Location, Location),
}

fn rotate(d: Direction, c: usize, data: &mut [u8]) {
    let len = data.len();
    for _ in 0..c {
        match d {
            Direction::Left => for pos in 0..len-1 {
                    data.swap(pos, pos+1);
                },
            Direction::Right => for pos in 1..len {
                data.swap(len-pos, len-pos-1);
            },
        }
    }
}

impl Command {
    fn apply(&self, data: &mut [u8]) {
        match self {
            &Command::Swap(l1, l2) => {
                let l1 = l1.to_index(data);
                let l2 = l2.to_index(data);
                data.swap(l1, l2);
            }
            &Command::Reverse(l1, l2) => {
                let l1 = l1.to_index(data);
                let l2 = l2.to_index(data);
                data[l1..l2+1].reverse();
            }
            &Command::Rotate(d, c) => rotate(d, c, data),
            &Command::RotateBasedOn(l) => {
                let pos = 1 + l.to_index(data);
                let pos = if pos > 4 {
                    pos + 1
                } else {
                    pos
                };
                Command::Rotate(Direction::Right, pos).apply(data);
            }
            &Command::Move(l1, l2) => {
                let l1 = l1.to_index(data);
                let l2 = l2.to_index(data);
                if l1 > l2 {
                    rotate(Direction::Right, 1, &mut data[l2..l1+1]);
                } else if l1 < l2 {
                    rotate(Direction::Left, 1, &mut data[l1..l2+1]);
                }
            }
        }
    }
}

fn find_original(commands: &Vec<Command>, target: &str) -> String {
    let mut input = Vec::from(INPUT.as_bytes());
    let heap = Heap::new(&mut input);
    
    for c in heap {
        let mut input = c.clone();
        for c in commands {
            c.apply(input.as_mut_slice());
        }

        if input == Vec::from(target.as_bytes()) {
            return String::from_utf8(c).unwrap();
        }
    }
    unreachable!()
}

fn main() {
    let mut password = Vec::from(INPUT);
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let swap = Regex::new(r"swap (\w+) (\S+) with (\w+) (\S+)").unwrap();
    let rotate = Regex::new(r"rotate (\w+) (\d+) steps?").unwrap();
    let rotate_based_on = Regex::new(r"rotate based on position of letter (.)").unwrap();
    let reverse = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let move_c = Regex::new(r"move position (\d+) to position (\d+)").unwrap();

    handle.read_to_string(&mut buffer).unwrap();

    let mut commands = Vec::new();

    for line in buffer.lines() {
        if let Some(ref caps) = swap.captures(&line) {
            let l1 = if "position" == caps.at(1).unwrap() {
                let pos = usize::from_str(caps.at(2).unwrap()).unwrap();
                Location::Index(pos)
            } else if "letter" == caps.at(1).unwrap() {
                Location::Letter(caps.at(2).unwrap().as_bytes()[0])
            } else {
                println!("Cannot parse {}", line);
                panic!()
            };
            let l2 = if "position" == caps.at(3).unwrap() {
                let pos = usize::from_str(caps.at(4).unwrap()).unwrap();
                Location::Index(pos)
            } else if "letter" == caps.at(3).unwrap() {
                Location::Letter(caps.at(4).unwrap().as_bytes()[0])
            } else {
                println!("Cannot parse {}", line);
                panic!()
            };
            commands.push(Command::Swap(l1, l2));
        } else if let Some(ref caps) = rotate.captures(&line) {
            let d = if "left" == caps.at(1).unwrap() {
                Direction::Left
            } else if "right" == caps.at(1).unwrap() {
                Direction::Right
            } else {
                println!("Cannot parse {}", line);
                panic!()
            };
            let steps = usize::from_str(caps.at(2).unwrap()).unwrap();
            commands.push(Command::Rotate(d, steps));
        } else if let Some(ref caps) = rotate_based_on.captures(&line) {
            let l = caps.at(1).unwrap().as_bytes()[0];
            commands.push(Command::RotateBasedOn(Location::Letter(l)));
        } else if let Some(ref caps) = reverse.captures(&line) {
            let pos1 = usize::from_str(caps.at(1).unwrap()).unwrap();
            let pos2 = usize::from_str(caps.at(2).unwrap()).unwrap();
            commands.push(Command::Reverse(Location::Index(pos1), Location::Index(pos2)))
        } else if let Some(ref caps) = move_c.captures(&line) {
            let pos1 = usize::from_str(caps.at(1).unwrap()).unwrap();
            let pos2 = usize::from_str(caps.at(2).unwrap()).unwrap();
            commands.push(Command::Move(Location::Index(pos1), Location::Index(pos2)));
        } else {
            println!("Cannot parse {}", line);
            panic!();
        }
    }

    for c in &commands {
        c.apply(password.as_mut_slice()); 
    }

    println!("Resulting password: {}", String::from_utf8_lossy(password.as_slice()));

    println!("Unscrambled password: {}", find_original(&commands, TARGET));
}

#[test]
fn test_rotate() {
    let mut input = Vec::from("abcde");
    Command::Rotate(Direction::Left, 2).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("cdeab"));
    Command::Rotate(Direction::Right, 2).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("abcde"));
}

#[test]
fn test() {
    use Command::*;
    use Location::*;
    use Direction::*;

    let mut input = Vec::from("abcde");
    Swap(Index(0), Index(4)).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("ebcda"));
    Swap(Letter('d' as u8), Letter('b' as u8)).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("edcba"));
    Reverse(Index(0), Index(4)).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("abcde"));
    Rotate(Left, 1).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("bcdea"));
    Move(Index(1), Index(4)).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("bdeac"));
    Move(Index(3), Index(0)).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("abdec"));
    RotateBasedOn(Letter('b' as u8)).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("ecabd"));
    RotateBasedOn(Letter('d' as u8)).apply(input.as_mut_slice());
    assert_eq!(input, Vec::from("decab"));
}
