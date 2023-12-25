use std::ops::Range;

use nom::{
    branch::alt,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use nom_locate::LocatedSpan;
use nom_supreme::{error::ErrorTree, final_parser::final_parser, tag::complete::tag};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn rating(&self) -> u64 {
        (self.x + self.m + self.a + self.s) as u64
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    pub fn eval(&self, part: &Part) -> u32 {
        match self {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command<'a> {
    Goto(&'a str),
    Accept,
    Reject,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Comparison {
    LessThan,
    GreaterThan,
}

impl Comparison {
    pub fn eval(&self, value: u32, target: u32) -> bool {
        match self {
            Comparison::LessThan => value < target,
            Comparison::GreaterThan => value > target,
        }
    }

    pub fn invert(&self, target: u32) -> (Comparison, u32) {
        match self {
            // chance of underflow here, but no <0 in the test or input data
            Comparison::LessThan => (Comparison::GreaterThan, target - 1),
            Comparison::GreaterThan => (Comparison::LessThan, target + 1),
        }
    }

    pub fn to_range(&self, target: u32) -> Range<u32> {
        match self {
            Comparison::LessThan => 1..target,
            Comparison::GreaterThan => target + 1..4001,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Condition<'a> {
    Command(Command<'a>),
    Compare(Category, Comparison, u32, Command<'a>),
}

impl<'a> Condition<'a> {
    pub fn eval(&self, part: &Part) -> Option<Command<'a>> {
        match self {
            Condition::Command(c) => Some(*c),
            Condition::Compare(cat, cmp, t, c) => {
                if cmp.eval(cat.eval(part), *t) {
                    Some(*c)
                } else {
                    None
                }
            }
        }
    }
}

pub struct Workflow<'a> {
    pub name: &'a str,
    pub conditions: Vec<Condition<'a>>,
}

impl<'a> Workflow<'a> {
    pub fn eval(&self, part: &Part) -> Command<'a> {
        for condition in &self.conditions {
            if let Some(c) = condition.eval(part) {
                return c;
            }
        }
        panic!("workflow {} did not evaluate successfully", self.name)
    }
}

pub type Span = LocatedSpan<&'static str>;

pub fn parse_puzzle(input: &'static str) -> Result<(Vec<Workflow<'static>>, Vec<Part>), AocError> {
    final_parser(parse_all)(Span::new(input))
        .map_err(|e: ErrorTree<Span>| AocError::from_error_tree(input, e))
}

fn parse_all(input: Span) -> IResult<Span, (Vec<Workflow<'static>>, Vec<Part>), ErrorTree<Span>> {
    separated_pair(parse_workflows, newline, parse_parts)(input)
}

fn parse_workflows(input: Span) -> IResult<Span, Vec<Workflow<'static>>, ErrorTree<Span>> {
    many1(terminated(parse_workflow, newline))(input)
}

fn parse_workflow(input: Span) -> IResult<Span, Workflow<'static>, ErrorTree<Span>> {
    let (input, (name, conditions)) = tuple((
        alpha1,
        preceded(
            tag("{"),
            terminated(separated_list1(tag(","), parse_condition), tag("}")),
        ),
    ))(input)?;
    Ok((
        input,
        Workflow {
            name: name.fragment(),
            conditions,
        },
    ))
}

fn parse_condition(input: Span) -> IResult<Span, Condition<'static>, ErrorTree<Span>> {
    alt((parse_comparison, map(parse_command, Condition::Command)))(input)
}
fn parse_comparison(input: Span) -> IResult<Span, Condition<'static>, ErrorTree<Span>> {
    use nom::character::complete as cc;
    let (input, (cat, comp, target, command)) = tuple((
        alt((
            map(tag("x"), |_| Category::X),
            map(tag("m"), |_| Category::M),
            map(tag("a"), |_| Category::A),
            map(tag("s"), |_| Category::S),
        )),
        alt((
            map(tag("<"), |_| Comparison::LessThan),
            map(tag(">"), |_| Comparison::GreaterThan),
        )),
        cc::u32,
        preceded(tag(":"), parse_command),
    ))(input)?;
    Ok((input, Condition::Compare(cat, comp, target, command)))
}

fn parse_command(input: Span) -> IResult<Span, Command<'static>, ErrorTree<Span>> {
    alt((
        map(tag("A"), |_| Command::Accept),
        map(tag("R"), |_| Command::Reject),
        map(alpha1, |name: Span| Command::Goto(name.fragment())),
    ))(input)
}

fn parse_parts(input: Span) -> IResult<Span, Vec<Part>, ErrorTree<Span>> {
    many1(terminated(parse_part, newline))(input)
}

fn parse_part(input: Span) -> IResult<Span, Part, ErrorTree<Span>> {
    use nom::character::complete as cc;
    let (input, (x, m, a, s)) = preceded(
        tag("{"),
        terminated(
            tuple((
                preceded(tag("x="), cc::u32),
                preceded(tag(",m="), cc::u32),
                preceded(tag(",a="), cc::u32),
                preceded(tag(",s="), cc::u32),
            )),
            tag("}"),
        ),
    )(input)?;
    Ok((input, Part { x, m, a, s }))
}
