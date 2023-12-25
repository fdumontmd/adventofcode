use std::collections::HashMap;

use nom::{
    branch::alt,
    character::complete::{alphanumeric1, newline},
    combinator::map,
    multi::many1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use nom_locate::LocatedSpan;
use nom_supreme::{error::ErrorTree, final_parser::final_parser, tag::complete::tag};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from(input: &str) -> Self {
        match input {
            "L" => Direction::Left,
            _ => Direction::Right,
        }
    }
    pub fn follow(&self, (left, right): &(&'static str, &'static str)) -> &'static str {
        match self {
            Direction::Left => left,
            Direction::Right => right,
        }
    }
}

pub struct Puzzle {
    pub directions: Vec<Direction>,
    pub turns: HashMap<&'static str, (&'static str, &'static str)>,
}

pub type Span = LocatedSpan<&'static str>;

pub fn parse(input: &'static str) -> Result<Puzzle, AocError> {
    final_parser(parse_puzzle)(Span::new(input))
        .map_err(|e: ErrorTree<Span>| AocError::from_error_tree(input, e))
}

fn parse_puzzle(input: Span) -> IResult<Span, Puzzle, ErrorTree<Span>> {
    let (input, (directions, turns)) = tuple((
        terminated(
            many1(map(alt((tag("L"), tag("R"))), |l: Span| {
                Direction::from(l.fragment())
            })),
            many1(newline),
        ),
        many1(terminated(turns, newline)),
    ))(input)?;
    Ok((
        input,
        Puzzle {
            directions,
            turns: HashMap::from_iter(turns),
        },
    ))
}

fn turns(
    input: Span,
) -> IResult<Span, (&'static str, (&'static str, &'static str)), ErrorTree<Span>> {
    let (input, (from, (left, right))) = tuple((
        terminated(alphanumeric1, tag(" = ")),
        (separated_pair(
            preceded(tag("("), alphanumeric1),
            tag(", "),
            terminated(alphanumeric1, tag(")")),
        )),
    ))(input)?;
    Ok((
        input,
        (from.fragment(), (left.fragment(), right.fragment())),
    ))
}
