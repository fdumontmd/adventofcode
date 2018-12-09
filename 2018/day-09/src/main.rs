use aoc_utils::get_input;

use indicatif::ProgressBar;
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
    //let progress = ProgressBar::new(p.last_marble as u64);
    let mut scores = vec![0; p.players];
    let mut circle = vec![0];
    let mut current_player = p.players - 1;
    let mut current_marble = 0;

    for marble in 1..=p.last_marble {
        if marble % 10000 == 0 {
            print!("#");
            use std::io::Write;
            std::io::stdout().flush();
        }
        //progress.tick();
        current_player = (current_player + 1) % p.players;
        if marble % 23 == 0 {
            scores[current_player] += marble;
            current_marble = (current_marble + 7 * circle.len()) - 7;
            current_marble %= circle.len();
            // we have a hole; try to use it
            scores[current_player] += circle[current_marble];
            // put next marble
            circle[current_marble] = marble + 1;
            // need to swap right twice
            let next = (current_marble + 1) % circle.len();
            circle.swap(current_marble, next);
            let next_next = (next + 1) % circle.len();
            circle.swap(next, next_next);
            current_marble = next_next;
            //progress.tick();
        } else {
            if marble > 1 && (marble - 1) % 23 == 0 {
                // already done
                continue;
            }
            let next = (current_marble + 1) % circle.len();
            if next == circle.len() - 1 {
                circle.push(marble);
            } else {
                circle.insert(next + 1, marble);
            }
            current_marble = next + 1;
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
