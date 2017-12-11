// TODO: check and learn from https://www.redblobgames.com/grids/hexagons/
use std::str::FromStr;
#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

#[derive(Debug)]
struct ParseDirectionError(String);

impl Direction {
    fn parse_sequence(s: &str) -> Vec<Direction> {
        s.split(",").map(|d| d.parse::<Direction>().unwrap()).collect()
    }
}

impl FromStr for Direction {
    type Err = ParseDirectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        match s{
            "n" => Ok(N),
            "s" => Ok(S),
            "ne" => Ok(NE),
            "nw" => Ok(NW),
            "se" => Ok(SE),
            "sw" => Ok(SW),
            _ => Err(ParseDirectionError(s.to_owned()))
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

impl Position {
    fn move_by(&self, d: Direction) -> Self {
        use Direction::*;
        let (dx, dy) = match d {
            N => (0, 2),
            S => (0, -2),
            NE => (1, 1),
            NW => (-1, 1),
            SE => (1, -1),
            SW => (-1, -1),
        };
        Position(self.0 + dx, self.1 + dy)
    }

    fn move_by_all(&self, ds: &Vec<Direction>) -> Self {
        let mut curr = *self;
        for d in ds {
            curr = curr.move_by(*d);
        }
        curr
    }

    fn move_span(&self, ds: &Vec<Direction>) -> Vec<Position> {
        let mut cur = *self;

        let mut r = Vec::with_capacity(ds.len());
        for d in ds {
            cur = cur.move_by(*d);
            r.push(cur);
        }
        r
    }

    fn distance(&self) -> usize {
        let mut curr = *self;
        for d in 0.. {
            if curr.0 == 0 && curr.1 == 0 {
                return d;
            }

            let (dx, dy) = (curr.0.signum(), curr.1.signum());

            use Direction::*;
            let dir = match (dx, dy) {
                (0, 1) => S,
                (0, -1) => N,
                (1, 1) => SW,
                (1, _) => NW,
                (-1, 1) => SE,
                (-1, _) => NE,
                _ => unreachable!(),
            };

            curr = curr.move_by(dir);
        }
        unreachable!()
    }
}

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
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

    for line in buf.lines() {
        let ds = Direction::parse_sequence(&line.unwrap());
        println!("{}", Position::default().move_by_all(&ds).distance());

        println!("{:?}", Position::default().move_span(&ds).into_iter().map(|p| p.distance()).max());
    }
}

#[test]
fn test() {
    assert_eq!(Position::default().move_by_all(&Direction::parse_sequence("ne,ne,ne")).distance(), 3);
    assert_eq!(Position::default().move_by_all(&Direction::parse_sequence("ne,ne,sw,sw")).distance(), 0);
    assert_eq!(Position::default().move_by_all(&Direction::parse_sequence("ne,ne,s,s")).distance(), 2);
    assert_eq!(Position::default().move_by_all(&Direction::parse_sequence("se,sw,se,sw,sw")).distance(), 3);
}
