use std::marker::PhantomData;

use nom::bytes::complete::tag;
use nom::character::complete as cc;
use nom::combinator::eof;
use nom::error::ParseError;
use nom::sequence::terminated;
use nom::{
    branch::alt,
    character::complete::{line_ending, space0, space1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use nom_locate::LocatedSpan;
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::final_parser;

use crate::custom_error::AocError;

// LocatedSpan is better because while nom-supreme has a useable Location, it
// does not appear to be utf8 compatible, and probably slow (does a search to
// find the actual location)
pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub subsets: Vec<Subsets>,
}

#[derive(Debug, Copy, Clone)]
pub struct Subsets {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

enum Color {
    Red,
    Green,
    Blue,
}

pub fn parse(input: &'static str) -> Result<Vec<Game>, AocError> {
    let games_res: Result<_, ErrorTree<Span>> =
        final_parser(Parser::<ErrorTree<Span>>::parse_all_games)(Span::new(input));
    games_res.map_err(|e| match e {
        nom_supreme::error::GenericErrorTree::Base { location, kind } => {
            let offset = location.location_offset().into();
            AocError::BadInput {
                src: input,
                bad_bits: miette::SourceSpan::new(offset, 0.into()),
                kind,
            }
        }
        nom_supreme::error::GenericErrorTree::Stack { .. } => todo!(),
        nom_supreme::error::GenericErrorTree::Alt(_) => todo!(),
    })
}

// could we get rid of this silly PhantomData? I just want a central place to attach my parser
// types
struct Parser<E>(PhantomData<E>);

impl<'a, E: ParseError<Span<'a>>> Parser<E> {
    fn parse_one_subset(input: Span<'a>) -> IResult<Span<'a>, Subsets, E> {
        map(
            separated_list1(
                tag(","),
                preceded(
                    space0,
                    separated_pair(
                        cc::u32,
                        space1,
                        alt((
                            // map_res would require a nasty looking type for E; could make it
                            // easier by just setting ErrorTree as the type next time
                            map(tag("red"), |_| Color::Red),
                            map(tag("green"), |_| Color::Green),
                            map(tag("blue"), |_| Color::Blue),
                        )),
                    ),
                ),
            ),
            |cubes| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for (count, color) in cubes {
                    match color {
                        Color::Red => red += count,
                        Color::Green => green += count,
                        Color::Blue => blue += count,
                    }
                }
                Subsets { red, green, blue }
            },
        )(input)
    }

    fn parse_all_subsets(input: Span<'a>) -> IResult<Span<'a>, Vec<Subsets>, E> {
        separated_list0(preceded(space0, tag(";")), Parser::parse_one_subset)(input)
    }

    fn parse_one_game(input: Span<'a>) -> IResult<Span<'a>, Game, E> {
        map(
            pair(
                delimited(tag("Game "), cc::u32, tag(":")),
                Parser::parse_all_subsets,
            ),
            |(id, subsets)| Game { id, subsets },
        )(input)
    }

    pub fn parse_all_games(input: Span<'a>) -> IResult<Span<'a>, Vec<Game>, E> {
        terminated(
            separated_list0(line_ending, Parser::parse_one_game),
            alt((eof, line_ending)),
        )(input)
    }
}
