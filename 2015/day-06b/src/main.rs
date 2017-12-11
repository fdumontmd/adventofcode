use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Coordinate {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

#[derive(Debug)]
enum CoordinateParseError {
    FormatParseError,
    NumberParseError,
}

impl From<ParseIntError> for CoordinateParseError {
    fn from(err: ParseIntError) -> CoordinateParseError {
        CoordinateParseError::NumberParseError
    }
}

impl FromStr for Coordinate {
    type Err = CoordinateParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(' ').collect::<Vec<&str>>();
        if v.len() != 3 {
            return Err(CoordinateParseError::FormatParseError);
        }

        let top_left = v[0].split(',').collect::<Vec<&str>>();
        let bottom_right = v[2].split(',').collect::<Vec<&str>>();

        if top_left.len() != 2 || bottom_right.len() != 2 {

            return Err(CoordinateParseError::FormatParseError);
        }

        let x1 = try!(top_left[0].parse());
        let y1 = try!(top_left[1].parse());
        let x2 = try!(bottom_right[0].parse());
        let y2 = try!(bottom_right[1].parse());


        Ok(Coordinate::new(x1, y1, x2, y2))
    }
}

impl Coordinate {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Coordinate {
        Coordinate {
            top_left: (x1, y1),
            bottom_right: (x2, y2),
        }
    }
}

struct Command {
    action: Action,
    coord: Coordinate,
}

#[derive(Debug)]
enum CommandParseError {
    ParseError,
}

impl FromStr for Command {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pos = 0;
        let mut action = Action::TurnOn;
        if s.starts_with("turn on") {
            action = Action::TurnOn;
            pos = 8;
        } else if s.starts_with("turn off") {
            action = Action::TurnOff;
            pos = 9
        } else if s.starts_with("toggle") {
            action = Action::Toggle;
            pos = 7;
        } else {
            unreachable!();
        }

        let c = s[pos..].parse().unwrap();

        Ok(Command::new(action, c))
    }
}

impl Command {
    fn new(action: Action, coord: Coordinate) -> Command {
        Command {
            action: action,
            coord: coord,
        }
    }
}

struct Grid {
    lights: Vec<u32>,
}

impl Grid {
    fn new() -> Grid {
        Grid { lights: vec![0; 1000000] }
    }

    fn process(&mut self, c: Command) {
        let Command{action, coord} = c;

        let (col_from, row_from) = coord.top_left;
        let (col_to, row_to) = coord.bottom_right;

        for r in row_from..(row_to + 1) {
            for c in col_from..(col_to + 1) {
                use Action::*;
                let mut i = &mut self.lights[r * 1000 + c];
                match action {
                    TurnOn => *i += 1,
                    TurnOff => {
                        if *i > 0 {
                            *i -= 1;
                        } else {
                            *i = 0;
                        }
                    }
                    Toggle => *i += 2,
                }
            }
        }

    }

    fn count_on(&self) -> u64 {
        let mut total = 0;
        for i in &self.lights {
            total += *i as u64;
        }
        total
    }
}


fn main() {
    let mut g = Grid::new();

    for line in BufReader::new(stdin())
                    .lines()
                    .filter_map(|result| result.ok()) {
        if !line.trim().is_empty() {
            let c: Command = line.parse().unwrap();
            g.process(c);
        }
    }

    println!("Lights brightness: {}", g.count_on());
}
