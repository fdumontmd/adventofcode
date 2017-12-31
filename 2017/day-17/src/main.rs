use std::ops::Index;
use std::fmt::{Debug, Error, Formatter};

struct CircularBuffer {
    current_position: usize,
    buffer: Vec<usize>,
}

impl CircularBuffer {
    fn new() -> Self {
        let mut buffer = Vec::new();
        buffer.push(0);
        CircularBuffer {
            current_position: 0,
            buffer,
        }
    }

    fn move_forward(&mut self, steps: usize) {
        self.current_position += steps;
        self.current_position = self.current_position % self.buffer.len();
    }

    fn insert_after_current(&mut self, val: usize) {
        if self.current_position == self.buffer.len() - 1 {
            self.buffer.push(val);
        } else {
            self.buffer.insert(self.current_position + 1, val);
        }
        self.current_position += 1;
    }
}

impl Index<usize> for CircularBuffer {
    type Output = usize;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.buffer[idx % self.buffer.len()]
    }
}

impl Debug for CircularBuffer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (idx, v) in self.buffer.iter().enumerate() {
            if idx == self.current_position {
                write!(f, "({}) ", v)?;
            } else {
                write!(f, "{} ", v)?;
            }
        }
        write!(f, "\n")
    }
}

struct FakeCircularBuffer {
    current_position: usize,
    len: usize,
    value_after_zero: Option<usize>,
}

impl FakeCircularBuffer {
    fn new() -> Self {
        FakeCircularBuffer {
            current_position: 0,
            len: 1,
            value_after_zero: None,
        }
    }
    fn move_forward(&mut self, steps: usize) {
        self.current_position += steps;
        self.current_position = self.current_position % self.len;
    }

    fn insert_after_current(&mut self, val: usize) {
        if self.current_position == 0 {
            self.value_after_zero = Some(val);
        }
        self.current_position += 1;
        self.len += 1;
        self.current_position = self.current_position % self.len;
    }
}

fn main() {
    let stride = 301;
    let mut cb = CircularBuffer::new();
    for idx in 1..2018 {
        cb.move_forward(stride);
        cb.insert_after_current(idx);
    }
    println!("Next value after 2017 insertions: {}", cb[cb.current_position + 1]);
    // a different approach is needed: iterate over the numbers, and compute the
    // current_position as before, but do not insert anything; just keep track of
    // any number inserted when current_position == 0
    let mut fcb = FakeCircularBuffer::new();
    for idx in 1..50_000_001 {
        fcb.move_forward(stride);
        fcb.insert_after_current(idx);
    }
    println!("Value after 0: {}", fcb.value_after_zero.unwrap());
}

#[test]
fn test_basic() {
    let stride = 3;
    let mut cb = CircularBuffer::new();
    for idx in 1..2018 {
        cb.move_forward(stride);
        cb.insert_after_current(idx);
    }
    assert_eq!(cb[cb.current_position + 1], 638);
}
