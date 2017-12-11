use std::char;
use std::io::{self, Read};
use std::marker::Sized;
use std::vec::Vec;

trait Keypad {
    fn up(&mut self);
    fn down(&mut self);
    fn left(&mut self);
    fn right(&mut self);
    fn current(&self) -> char;
    fn new() -> Self;
    fn get_code(instructions: &str) -> String 
        where Self: Sized {
        let mut code = Vec::new();
        let mut keypad = Self::new();

        for c in instructions.chars() {
            match c {
                'U' => keypad.up(),
                'D' => keypad.down(),
                'L' => keypad.left(),
                'R' => keypad.right(),
                '\n' => {
                    code.push(keypad.current());
                },
                _ => (),
            }
        }
        code.into_iter().collect()
    }
}

struct BasicKeypad {
    x: u32,
    y: u32,
}

impl Keypad for BasicKeypad {
    fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }
    fn down(&mut self) {
        if self.y < 2 {
            self.y += 1;
        }
    }
    fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    fn right(&mut self) {
        if self.x < 2 {
            self.x += 1;
        }
    }
    fn current(&self) -> char {
        char::from_digit(self.x + self.y * 3 + 1, 10).unwrap()
    }
    fn new() -> Self {
        BasicKeypad { x: 1, y: 1}
    }
}

struct FancyKeypad {
    x: i32,
    y: i32,
}

fn min(c: i32) -> i32 {
    (c - 2).abs()
}

fn max(c: i32) -> i32 {
    4 - min(c)
}

impl Keypad for FancyKeypad {
    fn up(&mut self) {
        if self.y > min(self.x) {
            self.y -= 1;
        }
    }
    fn down(&mut self) {
        if self.y < max(self.x) {
            self.y += 1;
        }
    }
    fn left(&mut self) {
        if self.x > min(self.y) {
            self.x -= 1;
        }
    }
    fn right(&mut self) {
        if self.x < max(self.y) {
            self.x += 1;
        }
    }
    fn current(&self) -> char {
        match (self.x, self.y) {
            (2, 0) => '1',
            (1, 1) => '2',
            (2, 1) => '3',
            (3, 1) => '4',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => 'A',
            (2, 3) => 'B',
            (3, 3) => 'C',
            (2, 4) => 'D',
            _ => unreachable!()
        }
    }
    fn new() -> Self {
        FancyKeypad { x: 0, y: 2}
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handler = stdin.lock();
    handler.read_to_string(&mut buffer).unwrap();
    println!("Basic code: {}", BasicKeypad::get_code(&buffer));
    println!("Fancy code: {}", FancyKeypad::get_code(&buffer));
}
