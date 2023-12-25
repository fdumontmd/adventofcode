use nom::character::complete::{self as cc, alpha1};
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use nom_locate::LocatedSpan;
use nom_supreme::{error::ErrorTree, final_parser::final_parser};

use crate::custom_error::AocError;

#[derive(Debug)]
pub struct Conversion {
    pub dest: u32,
    pub from: u32,
    pub len: u32,
}

impl Conversion {
    pub fn in_range(&self, src: u32) -> bool {
        self.from <= src && (src - self.from) < self.len
    }

    pub fn convert(&self, src: u32) -> u32 {
        (src - self.from) + self.dest
    }
}

pub struct Map {
    pub src: String,
    pub dst: String,
    pub conversions: Vec<Conversion>,
}

impl Map {
    pub fn convert(&self, src: u32) -> u32 {
        for conv in &self.conversions {
            if conv.in_range(src) {
                return conv.convert(src);
            }
        }
        src
    }

    pub fn map_range_to(&self, mut range: (u32, u32)) -> Vec<(u32, u32)> {
        let mut v = Vec::new();
        for c in &self.conversions {
            if range.0 < c.from && (c.from - range.0) <= range.1 {
                let prefix = c.from - range.0;
                v.push((range.0, prefix));
                range.1 -= prefix;
                range.0 = c.from;
            }

            if range.1 == 0 {
                break;
            }

            if c.in_range(range.0) {
                let pre = range.0 - c.from;
                let rest = c.len - pre;
                let converted = rest.min(range.1);
                let new_from = c.convert(range.0);
                range.1 -= converted;
                range.0 += converted;
                v.push((new_from, converted));
            }
        }
        if range.1 > 0 {
            v.push(range);
        }
        v
    }
}

pub struct Puzzle {
    pub seeds: Vec<u32>,
    pub maps: Vec<Map>,
}

impl Puzzle {
    pub fn part1(&self) -> u32 {
        self.seeds
            .iter()
            .map(|s| self.map_to_location(*s))
            .min()
            .unwrap()
    }

    // we exploit the fact that the maps are already sorted; otherwise we'd have to do that first
    pub fn map_to_location(&self, seed: u32) -> u32 {
        self.maps.iter().fold(seed, |l, m| m.convert(l))
    }

    pub fn part2(&self) -> u32 {
        let seeds = self.seeds.chunks(2).map(|c| (c[0], c[1]));

        // seeds is an Iterator, but not the same type as the Iterators down the pipeline, so
        // probably not possible to just use fold without Boxing, and don't want to pass a Vec either

        seeds
            .flat_map(|s| self.maps[0].map_range_to(s))
            .flat_map(|s| self.maps[1].map_range_to(s))
            .flat_map(|s| self.maps[2].map_range_to(s))
            .flat_map(|s| self.maps[3].map_range_to(s))
            .flat_map(|s| self.maps[4].map_range_to(s))
            .flat_map(|s| self.maps[5].map_range_to(s))
            .flat_map(|s| self.maps[6].map_range_to(s))
            //.flat_map(|s| self.map_range_to("location", s))
            .min()
            .map(|r| r.0)
            .unwrap()
    }
}

pub type Span = LocatedSpan<&'static str>;

pub fn parse(input: &'static str) -> Result<Puzzle, AocError> {
    final_parser(parse_int)(Span::new(input)).map_err(|e| AocError::from_error_tree(input, e))
}

fn parse_int(input: Span) -> IResult<Span, Puzzle, ErrorTree<Span>> {
    let (input, (seeds, maps)) = tuple((
        parse_seeds,
        preceded(many1(newline), many1(terminated(parse_map, many1(newline)))),
    ))(input)?;
    Ok((input, Puzzle { seeds, maps }))
}

fn parse_seeds(input: Span) -> IResult<Span, Vec<u32>, ErrorTree<Span>> {
    preceded(tag("seeds: "), separated_list1(space1, cc::u32))(input)
}

fn parse_map(input: Span) -> IResult<Span, Map, ErrorTree<Span>> {
    let (input, ((src, dst), convs)) = tuple((
        terminated(
            separated_pair(alpha1, tag("-to-"), alpha1),
            preceded(tag(" map:"), newline),
        ),
        separated_list1(newline, separated_list1(space1, cc::u32)),
    ))(input)?;
    let mut conversions: Vec<Conversion> = convs
        .into_iter()
        .map(|v| Conversion {
            dest: v[0],
            from: v[1],
            len: v[2],
        })
        .collect();
    conversions.sort_by_key(|m| m.from);
    Ok((
        input,
        Map {
            src: src.to_string(),
            dst: dst.to_string(),
            conversions,
        },
    ))
}
