use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;
use std::io::BufRead;
use std::str::FromStr;
use aoc_utils::get_input;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn cover(&self, cloth: &mut HashMap<(usize, usize), usize>) {
        for x in self.left .. self.left + self.width {
            for y in self.top .. self.top + self.height {
                *cloth.entry((x, y)).or_default() += 1;
            }
        }
    }

    fn contains(&self, point: (usize, usize)) -> bool {
        self.left <= point.0 && point.0 <= (self.left + self.width)
            && self.top <= point.1 && point.1 <= (self.top + self.height)
    }

    fn overlap(&self, other: &Claim) -> bool {
        for x in other.left..other.left + other.width {
            for y in other.top..other.top + other.height {
                if self.contains((x, y)) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
struct ParseClaimError{
    message: String,
    cause: Option<ParseIntError>
}

impl fmt::Display for ParseClaimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseClaimError {
    fn description(&self) -> &str {
        "ParseClaimError"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.cause.as_ref().map(|e| e as &dyn Error)
    }
}

lazy_static! {
    static ref CLAIM_RE: Regex = Regex::new(r"#(\S+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

impl From<ParseIntError> for ParseClaimError {
    fn from(error: ParseIntError) -> Self {
        ParseClaimError {
            message: "error parsing field".into(),
            cause: Some(error),
        }
    }
}

impl FromStr for Claim {
    type Err = ParseClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for cap in CLAIM_RE.captures_iter(s) {
            return Ok(Claim {
                    id: cap[1].parse()?,
                    left: cap[2].parse()?,
                    top: cap[3].parse()?,
                    width: cap[4].parse()?,
                    height: cap[5].parse()?,
                });
        }
        Err(ParseClaimError{ message: s.into(), cause: None})
    }
}

fn input() -> Result<Vec<Claim>, Box<dyn Error>> {
    let mut v = Vec::new();
    for line in get_input().lines() {
        let line = line?;
        v.push(line.parse()?);
    }
    Ok(v)
}

fn part_one(v: &Vec<Claim>) -> usize {
    let mut cloth: HashMap<(usize, usize), usize> = HashMap::new();

    for claim in v {
        claim.cover(&mut cloth);
    }

    cloth.values().filter(|v| **v > 1).count()
}

fn part_two(v: &Vec<Claim>) -> Option<usize> {
    'main_loop:
    for c1 in v {
        for c2 in v {
            if c1.id != c2.id && c1.overlap(&c2) {
                continue 'main_loop;
            }
        }

        return Some(c1.id);
    }
    None
}

fn main() -> Result<(), Box<dyn Error>>{
    let v = input()?;
    println!("multiple claim count: {}", part_one(&v));
    println!("non overlapping claim: {:?}", part_two(&v).unwrap());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<Claim> {
        let mut v = Vec::new();
        v.push("#1 @ 1,3: 4x4".parse().unwrap());
        v.push("#2 @ 3,1: 4x4".parse().unwrap());
        v.push("#3 @ 5,5: 2x2".parse().unwrap());
        v
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&input()), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&input()), Some(3));
    }
}
