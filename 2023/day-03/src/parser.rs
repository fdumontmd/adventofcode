use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, none_of},
    combinator::{consumed, map},
    multi::many0,
    sequence::{preceded, terminated},
    IResult,
};
use nom_locate::LocatedSpan;
use nom_supreme::{error::ErrorTree, final_parser::final_parser};

use crate::custom_error::AocError;

#[derive(Debug)]
pub struct Point(u32, u32);
#[derive(Debug)]
pub struct Rect(Point, Point);

impl Rect {
    pub fn contains(&self, pos: &Point) -> bool {
        self.0 .0 <= pos.0 && self.1 .0 >= pos.0 && self.0 .1 <= pos.1 && self.1 .1 >= pos.1
    }
}
#[derive(Debug)]
pub struct Number {
    pub n: u32,
    pub rect: Rect,
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol: char,
    pub pos: Point,
}

#[derive(Debug)]
pub struct Puzzle {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
}

// we'll only deal with static str anyway
type Span = LocatedSpan<&'static str>;

pub fn parse(input: &'static str) -> Result<Puzzle, AocError> {
    final_parser(parse_internal)(Span::new(input)).map_err(|e: ErrorTree<Span>| match e {
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

fn parse_internal(input: Span) -> IResult<Span, Puzzle, ErrorTree<Span>> {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let (input, lines) = many0(terminated(line, newline))(input)?;
    for mut line in lines {
        numbers.append(&mut line.0);
        symbols.append(&mut line.1);
    }

    Ok((input, Puzzle { numbers, symbols }))
}

fn line(input: Span) -> IResult<Span, (Vec<Number>, Vec<Symbol>), ErrorTree<Span>> {
    let (input, locations) = preceded(
        many0(tag(".")),
        many0(terminated(number_or_symbol, many0(tag(".")))),
    )(input)?;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for location in locations {
        match location {
            Location::Number(n) => numbers.push(n),
            Location::Symbol(s) => symbols.push(s),
        }
    }

    Ok((input, (numbers, symbols)))
}

enum Location {
    Number(Number),
    Symbol(Symbol),
}

fn from_located(l: &Span) -> Rect {
    let (x, y) = (l.get_column() as u32, l.location_line());
    let len = l.fragment().len() as u32;

    let (x_top, y_top) = (x.saturating_add_signed(-1), y.saturating_add_signed(-1));
    let (x_bottom, y_bottom) = (x + len, y + 1);
    Rect(Point(x_top, y_top), Point(x_bottom, y_bottom))
}

fn number_or_symbol(input: Span) -> IResult<Span, Location, ErrorTree<Span>> {
    alt((
        map(
            consumed(nom::character::complete::u32),
            |(l, n): (Span, u32)| {
                Location::Number(Number {
                    n,
                    rect: from_located(&l),
                })
            },
        ),
        map(
            consumed(none_of("0123456789.\n")),
            |(l, symbol): (Span, char)| {
                Location::Symbol(Symbol {
                    symbol,
                    pos: Point(l.get_column() as u32, l.location_line()),
                })
            },
        ),
    ))(input)
}
