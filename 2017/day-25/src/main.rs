use std::collections::VecDeque;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::{
        anychar,
        complete::{newline, space0},
    },
    combinator::{map, value},
    multi::many1,
    sequence::{pair, preceded, terminated},
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, tag("left")),
        value(Direction::Right, tag("right")),
    ))
    .parse(input)
}

// states are identified by consecutive letters, so we
// just store them in a slice and identify them by their
// index, defined as their letter - 'A'
#[derive(Debug, Copy, Clone)]
struct Action {
    write: bool,
    move_to: Direction,
    next_state: usize,
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    let write = preceded(
        pair(space0, tag("- Write the value ")),
        terminated(
            alt((value(false, tag("0")), value(true, tag("1")))),
            tag("."),
        ),
    );
    let move_to = preceded(
        pair(space0, tag("- Move one slot to the ")),
        terminated(parse_direction, tag(".")),
    );
    let next_state = preceded(tag("- Continue with state "), terminated(anychar, tag(".")));

    map(
        (
            preceded(space0, terminated(write, newline)),
            preceded(space0, terminated(move_to, newline)),
            preceded(space0, terminated(next_state, newline)),
        ),
        |(w, d, ns)| {
            let ns = (ns as u8 - b'A') as usize;
            Action {
                write: w,
                move_to: d,
                next_state: ns,
            }
        },
    )
    .parse(input)
}

#[derive(Debug, Copy, Clone)]
struct State {
    _id: char,
    if_true: Action,
    if_false: Action,
}

fn parse_state(input: &str) -> IResult<&str, State> {
    map(
        (
            terminated(preceded(tag("In state "), anychar), pair(tag(":"), newline)),
            preceded(
                pair(tag("  If the current value is 0:"), newline),
                parse_action,
            ),
            preceded(
                pair(tag("  If the current value is 1:"), newline),
                parse_action,
            ),
        ),
        |(s, iff, ift)| State {
            _id: s,
            if_true: ift,
            if_false: iff,
        },
    )
    .parse(input)
}

struct TuringMachine {
    state: usize,
    max_steps: usize,
    states: Vec<State>,
    tape: VecDeque<bool>,
    tape_idx: usize,
}

fn parse_turing_machine(input: &str) -> IResult<&str, TuringMachine> {
    map(
        (
            terminated(
                preceded(tag("Begin in state "), anychar),
                pair(tag("."), newline),
            ),
            terminated(
                preceded(
                    tag("Perform a diagnostic checksum after "),
                    nom::character::complete::usize,
                ),
                pair(tag(" steps."), newline),
            ),
            many1(preceded(newline, parse_state)),
        ),
        |(state, max_steps, states)| {
            let state = (state as u8 - b'A') as usize;
            let mut tm = TuringMachine {
                state,
                max_steps,
                states,
                tape: VecDeque::new(),
                tape_idx: 0,
            };
            tm.tape.push_front(false);
            tm
        },
    )
    .parse(input)
}

impl TuringMachine {
    fn move_left(&mut self) {
        if self.tape_idx == 0 {
            self.tape.push_front(false);
        } else {
            self.tape_idx -= 1;
        }
    }

    fn move_right(&mut self) {
        self.tape_idx += 1;
        if self.tape_idx == self.tape.len() {
            self.tape.push_back(false);
        }
    }

    fn step(&mut self) {
        let action = if self.tape[self.tape_idx] {
            self.states[self.state].if_true
        } else {
            self.states[self.state].if_false
        };

        self.tape[self.tape_idx] = action.write;

        match action.move_to {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
        self.state = action.next_state;
    }

    fn execute(&mut self) -> usize {
        for _ in 0..self.max_steps {
            self.step()
        }
        self.checksum()
    }

    fn checksum(&self) -> usize {
        self.tape.iter().filter(|b| **b).count()
    }
}

fn parse_all(input: &str) -> Result<TuringMachine, String> {
    match parse_turing_machine(input) {
        Ok((rem, tm)) => {
            if rem.is_empty() {
                Ok(tm)
            } else {
                Err(format!("incomplete parsing, remaining: {rem}"))
            }
        }
        Err(e) => Err(format!("parsing error: {e}")),
    }
}

fn puzzle(input: &str) -> usize {
    let mut tm = parse_all(input).unwrap();
    tm.execute()
}

fn main() {
    println!("part 1: {}", puzzle(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

    #[test_case("right", Direction::Right, "")]
    #[test_case("left", Direction::Left, "")]
    fn test_parse_direction(input: &str, dir: Direction, output: &str) {
        assert_eq!(Ok((output, dir)), parse_direction(input));
    }

    #[test_case(TEST_INPUT, 3)]
    #[test_case(INPUT, 2832)]
    fn test_puzzle(input: &str, checksum: usize) {
        assert_eq!(checksum, puzzle(input));
    }
}
