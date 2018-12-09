use aoc_utils::get_input;
use aoc_utils::ring::Ring;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PROBLEM: Regex =
        Regex::new(r"(\d+) players; last marble is worth (\d+) points")
        .unwrap();
}

struct Problem {
    players: usize,
    last_marble: usize,
}

#[derive(Debug)]
struct ProblemParsingError(String);

use std::fmt;

impl fmt::Display for ProblemParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use std::error::Error;
impl Error for ProblemParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

use std::str::FromStr;
impl FromStr for Problem {
    type Err = ProblemParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for cap in PROBLEM.captures_iter(s) {
            let players = cap[1].parse().or(Err(ProblemParsingError(s.into())))?;
            let last_marble = cap[2].parse().or(Err(ProblemParsingError(s.into())))?;

            return Ok(Problem {
                players,
                last_marble,
            });
        }
        Err(ProblemParsingError(s.into()))
    }
}

fn score(p: &Problem) -> usize {
    let mut scores = vec![0; p.players];
    let mut circle = Ring::new();
    circle.insert(0);
    let mut current_player = p.players - 1;

    for marble in 1..=p.last_marble {
        current_player = (current_player + 1) % p.players;
        if marble % 23 == 0 {
            scores[current_player] += marble;
            circle.move_left(7);
            scores[current_player] += circle.remove().unwrap();
        } else {
            circle.move_right(2);
            circle.insert(marble);
        }
    }

    scores.into_iter().max().unwrap()
}

fn input() -> Result<String, Box<dyn Error>> {
    use std::io::Read;
    let mut buf = String::new();
    get_input().read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut p: Problem = input()?.parse()?;
    println!("part one: {}", score(&p));
    p.last_marble *= 100;
    println!("part two: {}", score(&p));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(score(&("9 players; last marble is worth 25 points".parse().unwrap())), 32);
        assert_eq!(score(&("10 players; last marble is worth 1618 points".parse().unwrap())), 8317);
        assert_eq!(score(&("13 players; last marble is worth 7999 points".parse().unwrap())), 146373);
        assert_eq!(score(&("17 players; last marble is worth 1104 points".parse().unwrap())), 2764);
        assert_eq!(score(&("21 players; last marble is worth 6111 points".parse().unwrap())), 54718);
        assert_eq!(score(&("30 players; last marble is worth 5807 points".parse().unwrap())), 37305);
    }
}
