use std::collections::HashSet;

use nom::character::complete::{self};
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
use nom::{
    character::complete::space1,
    sequence::{pair, preceded},
};
use nom_locate::LocatedSpan;
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::tag::complete::tag;

use crate::custom_error::AocError;

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub winners: HashSet<u32>,
    pub picks: Vec<u32>,
}

impl Card {
    pub fn wins(&self) -> usize {
        self.picks
            .iter()
            .filter(|p| self.winners.contains(p))
            .count()
    }
    pub fn score(&self) -> u32 {
        let w = self.wins();
        if w > 0 {
            1 << (w - 1)
        } else {
            0
        }
    }
}

pub type Span = LocatedSpan<&'static str>;

pub fn parse(input: &'static str) -> Result<Card, AocError> {
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

fn parse_internal(input: Span) -> IResult<Span, Card, ErrorTree<Span>> {
    let (input, id) = preceded(pair(tag("Card"), space1), complete::u64)(input)?;
    let (input, (winners, picks)) = preceded(
        pair(tag(":"), space1),
        separated_pair(
            separated_list0(space1, complete::u32),
            tuple((space1, tag("|"), space1)),
            separated_list0(space1, complete::u32),
        ),
    )(input)?;
    let id = id as usize;
    Ok((
        input,
        Card {
            id,
            winners: HashSet::from_iter(winners),
            picks,
        },
    ))
}
