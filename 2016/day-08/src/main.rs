use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

static LINE_ON: [bool; WIDTH] = [true; WIDTH]; 

struct Panel {
    panel: [[bool; WIDTH]; HEIGHT],
}

impl Panel {
    fn new() -> Self {
        Panel{ panel: [[false; 50]; 6] }
    }

    fn rect(&mut self, x: usize, y: usize) {
        for r in 0..y {
            self.panel[r][..x].clone_from_slice(&LINE_ON[..x]);
        }
    }

    fn rotate_row(&mut self, row: usize, shift: usize) {
        let mut row_data = [false; WIDTH];
        row_data.clone_from_slice(&self.panel[row]);
        self.panel[row][shift..].clone_from_slice(&row_data[..WIDTH-shift]);
        self.panel[row][..shift].clone_from_slice(&row_data[WIDTH-shift..]);
    }

    fn rotate_column(&mut self, col: usize, shift: usize) {
        let mut column_data = [false; HEIGHT];
        for row in 0..HEIGHT {
            column_data[row] = self.panel[row][col];
        }
        for row in 0..HEIGHT {
            self.panel[(row + shift) % HEIGHT][col] = column_data[row];
        }
    }

    fn count(&self) -> usize {
        let mut total = 0;
        for r in self.panel.iter() {
            for c in r.iter() {
                if *c {
                    total += 1;
                }
            }
        }
        total
    }

    fn execute_command(&mut self, command: &str) {
        if command.starts_with("rect ") {
            let size = &command[5..];
            let mut iter = size.split("x");
            let x = usize::from_str(iter.next().unwrap()).unwrap();
            let y = usize::from_str(iter.next().unwrap()).unwrap();
            self.rect(x, y);
        } else if command.starts_with("rotate column x=") {
            let params = &command[16..];
            let mut iter = params.split("by");
            let col = usize::from_str(iter.next().unwrap().trim()).unwrap();
            let shift = usize::from_str(iter.next().unwrap().trim()).unwrap();
            self.rotate_column(col, shift);
        } else if command.starts_with("rotate row y=") {
            let params = &command[13..];
            let mut iter = params.split("by");
            let row = usize::from_str(iter.next().unwrap().trim()).unwrap();
            let shift = usize::from_str(iter.next().unwrap().trim()).unwrap();
            self.rotate_row(row, shift);
        } else {
            unreachable!();
        }
    }
}

impl fmt::Display for Panel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.panel.iter() {
            for c in r.iter() {
                write!(f, "{}", if *c {'#'} else {' '})?;
            }

            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut panel = Panel::new();

    for command in buffer.lines() {
        panel.execute_command(command);
    }

    println!("Count of lights on: {}", panel.count());
    println!("{}", panel);
}
