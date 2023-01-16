use nom::character::complete::{self as cc, line_ending, space1};
use nom::multi::{many1, separated_list0};
use nom::{
    bytes::complete::tag, error::ParseError, multi::separated_list1, sequence::tuple, IResult,
};
use nom_locate::LocatedSpan;

use crate::{Instruction, TestCase};

pub(crate) type Span<'a> = LocatedSpan<&'a str>;

// extract parse_instruction as it is used twice
pub(crate) fn parse_input<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, (Vec<TestCase>, Vec<Instruction>), E> {
    let (i, (test_cases, _)) = tuple((parse_all_test_cases, many1(line_ending)))(i)?;
    let (i, instructions) = separated_list0(line_ending, separated_list1(space1, cc::i64))(i)?;
    let (i, _) = line_ending(i)?;

    let registers: Vec<_> = instructions
        .into_iter()
        .map(|i| Instruction {
            opcode: i[0] as u8,
            a: i[1],
            b: i[2],
            c: i[3] as usize,
        })
        .collect();

    Ok((i, (test_cases, registers)))
}

pub(crate) fn parse_all_test_cases<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, Vec<TestCase>, E> {
    separated_list1(line_ending, parse_test_case)(i)
}

pub(crate) fn parse_test_case<'a, E: ParseError<Span<'a>>>(
    i: Span<'a>,
) -> IResult<Span<'a>, TestCase, E> {
    let (i, (_, before, _, _)) = tuple((
        tag("Before: ["),
        separated_list1(tag(", "), cc::i64),
        tag("]"),
        line_ending,
    ))(i)?;
    let (i, (instr, _)) = tuple((separated_list1(space1, cc::i64), line_ending))(i)?;
    let (i, (_, after, _, _)) = tuple((
        tag("After:  ["),
        separated_list1(tag(", "), cc::i64),
        tag("]"),
        line_ending,
    ))(i)?;
    Ok((
        i,
        TestCase {
            before: [before[0], before[1], before[2], before[3]],
            instr: Instruction {
                opcode: instr[0] as u8,
                a: instr[1],
                b: instr[2],
                c: instr[3] as usize,
            },
            after: [after[0], after[1], after[2], after[3]],
        },
    ))
}
