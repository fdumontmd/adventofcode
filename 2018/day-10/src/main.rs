use aoc_utils::*;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref STAR_RE: Regex =
        Regex::new(
            r"^position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>$"
        ).unwrap();
}

// working hypothesis: at some point the distance between minimum y and
// maximum y will be 7; print the strip of stars then

#[derive(Copy, Clone)]
struct Star {
    position: (i64, i64),
    velocity: (i64, i64),
}

#[derive(Debug)]
struct StarParsingError(String);

use std::fmt;

impl fmt::Display for StarParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use std::error::Error;
impl Error for StarParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

use std::str::FromStr;

impl FromStr for Star {
    type Err = StarParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for cap in STAR_RE.captures_iter(s) {
            let position: (i64, i64) =
                (cap[1].parse().or_else(|_| Err(StarParsingError(s.into())))?,
                 cap[2].parse().or_else(|_| Err(StarParsingError(s.into())))?);
            let velocity: (i64, i64) =
                (cap[3].parse().or_else(|_| Err(StarParsingError(s.into())))?,
                 cap[4].parse().or_else(|_| Err(StarParsingError(s.into())))?);
            return Ok(Star{
                position,
                velocity
            });
        }
        Err(StarParsingError(s.into()))
    }
}

struct Constellation(Vec<Star>);

impl Constellation {
    fn new(stars: Vec<Star>) -> Self {
        Constellation(stars)
    }

    fn step(&self) -> Self {
        Constellation::new(
            self.0.iter().cloned()
                .map(|mut star| {
                    star.position.0 += star.velocity.0;
                    star.position.1 += star.velocity.1;
                    star
                }).collect()
        )
    }

    fn x_min(&self) -> i64 {
        self.0.iter().map(|s| s.position.0).min().unwrap()
    }

    fn x_max(&self) -> i64 {
        self.0.iter().map(|s| s.position.0).max().unwrap()
    }

    fn y_min(&self) -> i64 {
        self.0.iter().map(|s| s.position.1).min().unwrap()
    }

    fn y_max(&self) -> i64 {
        self.0.iter().map(|s| s.position.1).max().unwrap()
    }

    fn x_span(&self) -> usize {
        let x_min = self.x_min();
        let x_max = self.x_max();
        (x_max - x_min + 1) as usize
    }

    fn y_span(&self) -> usize {
        let y_min = self.y_min();
        let y_max = self.y_max();
        (y_max - y_min + 1) as usize
    }

    fn print(&self) {
        let x_min = self.x_min();
        let y_min = self.y_min();

        let x_span = self.x_span();
        let y_span = self.y_span();

        let mut lines = vec![vec![b' '; x_span]; y_span];

        for star in &self.0 {
            let x = (star.position.0 - x_min) as usize;
            let y = (star.position.1 - y_min) as usize;

            lines[y][x] = b'#';
        }

        for line in lines {
            println!("{}", String::from_utf8_lossy(&line));
        }
    }
}

fn input() -> Result<Constellation, Box<dyn Error>> {
    let mut v = Vec::new();
    for line in get_input().lines() {
        let line = line?;
        v.push(line.parse()?);
    }
    Ok(Constellation::new(v))
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let mut seconds = 0;
    let mut c = input()?;
    while c.y_span() > 8 {
        let nc = c.step();
        if nc.y_span() > c.y_span() {
            break;
        }
        seconds += 1;
        c = nc;
    }
    println!("minimum x-span: {}, y-span: {} after {} seconds", c.x_span(), c.y_span(), seconds);

    c.print();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part_one()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn test_parser() {
        INPUT.lines().map(|l| l.parse::<Star>().unwrap()).collect::<Vec<_>>();
    }
}
