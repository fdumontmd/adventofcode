static INPUT: &str = include_str!("input.txt");

type Register = [i64; 4];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Input {
    Immediate,
    Register,
}

impl Input {
    fn eval(&self, opi: i64, register: &Register) -> i64 {
        match self {
            Immediate => opi,
            Input::Register => register[opi as usize],
        }
    }
}

mod functions {
    pub fn add(a: i64, b: i64) -> i64 {
        a + b
    }
    pub fn mul(a: i64, b: i64) -> i64 {
        a * b
    }
    pub fn ban(a: i64, b: i64) -> i64 {
        a & b
    }
    pub fn bor(a: i64, b: i64) -> i64 {
        a | b
    }
    pub fn set(a: i64, _b: i64) -> i64 {
        a
    }
    pub fn gt(a: i64, b: i64) -> i64 {
        (a > b).into()
    }
    pub fn eq(a: i64, b: i64) -> i64 {
        (a == b).into()
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct Operation {
    name: &'static str,
    a: Input,
    b: Input,
    f: fn(i64, i64) -> i64,
}

impl Operation {
    const fn new(name: &'static str, a: Input, b: Input, f: fn(i64, i64) -> i64) -> Self {
        Self { name, a, b, f }
    }
}

use std::collections::BTreeSet;
use std::fmt::Display;

use miette::GraphicalReportHandler;
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, GenericErrorTree},
    final_parser::final_parser,
};
use Input::*;
static OPERATIONS: [Operation; 16] = [
    Operation::new("addr", Register, Register, functions::add),
    Operation::new("addi", Register, Immediate, functions::add),
    Operation::new("mulr", Register, Register, functions::mul),
    Operation::new("muli", Register, Immediate, functions::mul),
    Operation::new("banr", Register, Register, functions::ban),
    Operation::new("bani", Register, Immediate, functions::ban),
    Operation::new("borr", Register, Register, functions::bor),
    Operation::new("bori", Register, Immediate, functions::bor),
    Operation::new("setr", Register, Immediate, functions::set),
    Operation::new("seti", Immediate, Immediate, functions::set),
    Operation::new("gtir", Immediate, Register, functions::gt),
    Operation::new("gtri", Register, Immediate, functions::gt),
    Operation::new("gtrr", Register, Register, functions::gt),
    Operation::new("eqir", Immediate, Register, functions::eq),
    Operation::new("eqri", Register, Immediate, functions::eq),
    Operation::new("eqrr", Register, Register, functions::eq),
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    opcode: u8,
    a: i64,
    b: i64,
    c: usize,
}

impl Instruction {
    fn eval(&self, oper: &Operation, register: &Register) -> Register {
        let mut res = *register;
        let a = oper.a.eval(self.a, register);
        let b = oper.b.eval(self.b, register);
        let c = (oper.f)(a, b);
        res[self.c] = c;
        res
    }
}

mod parser;

#[derive(Debug)]
struct TestCase {
    before: Register,
    instr: Instruction,
    after: Register,
}

impl TestCase {
    fn matches(&self, operation: &Operation) -> bool {
        self.instr.eval(operation, &self.before) == self.after
    }
}

impl Display for TestCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Before: ")?;
        f.debug_list().entries(self.before).finish()?;
        writeln!(f)?;
        writeln!(
            f,
            "{} {} {} {}",
            self.instr.opcode, self.instr.a, self.instr.b, self.instr.c
        )?;

        write!(f, "After: ")?;
        f.debug_list().entries(self.after).finish()?;
        writeln!(f)
    }
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput {
    #[source_code]
    src: &'static str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

use parser::{parse_input, Span};

fn parse(input: &'static str) -> (Vec<TestCase>, Vec<Instruction>) {
    let input = Span::new(input);
    let input_res: Result<_, ErrorTree<Span>> = final_parser(parse_input::<ErrorTree<Span>>)(input);
    match input_res {
        Ok(input) => input,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: INPUT,
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack {
                    base: _,
                    contexts: _,
                } => todo!("stack"),
                GenericErrorTree::Alt(_) => todo!("alt"),
            }
            panic!();
        }
    }
}

fn part_01(input: &'static str) -> usize {
    let (test_cases, _) = parse(input);
    test_cases
        .into_iter()
        .filter_map(|tc| {
            let matching_op = OPERATIONS.iter().filter(|o| tc.matches(o)).count();
            if matching_op >= 3 {
                Some(true)
            } else {
                None
            }
        })
        .count()
}

fn part_02(input: &'static str) -> i64 {
    let (test_cases, program) = parse(input);
    // build a map from opcode to all possible instructions (i.e. opcode -> set of instructions)
    // iterate over test cases, removing instructions that do not match
    let mut mapping: Vec<BTreeSet<usize>> =
        Vec::from_iter((0..16).map(|_| BTreeSet::from_iter(0..16)));

    test_cases.into_iter().for_each(|tc| {
        OPERATIONS.iter().enumerate().for_each(|(idx, o)| {
            if !tc.matches(o) {
                mapping[tc.instr.opcode as usize].remove(&idx);
            }
        });
    });

    loop {
        if mapping.iter().all(|m| m.len() == 1) {
            break;
        }

        let singles: Vec<usize> = mapping
            .iter()
            .filter_map(|m| {
                if m.len() == 1 {
                    Some(*m.first().unwrap())
                } else {
                    None
                }
            })
            .collect();

        if singles.is_empty() {
            panic!("cannot simplify");
        }

        for m in &mut mapping {
            if m.len() > 1 {
                for s in &singles {
                    m.remove(s);
                }
            }
        }
    }

    let mapping: Vec<_> = mapping.into_iter().map(|m| *m.first().unwrap()).collect();
    let mut register = [0; 4];
    for i in program.into_iter() {
        register = i.eval(&OPERATIONS[mapping[i.opcode as usize]], &register);
    }
    register[0]
}

fn main() {
    println!("part 1: {}", part_01(INPUT));
    println!("part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod tests {
    static TEST_CASE: &str = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
";

    use nom_locate::LocatedSpan;

    use super::parser::parse_test_case;
    use super::*;
    #[test]
    fn test_matches() {
        let test_case = TestCase {
            before: [3, 2, 1, 1],
            instr: Instruction {
                opcode: 9,
                a: 2,
                b: 1,
                c: 2,
            },
            after: [3, 2, 2, 1],
        };
        let matching_op = OPERATIONS.iter().filter(|o| test_case.matches(o)).count();
        assert_eq!(3, matching_op);
    }

    #[test]
    fn test_part_01() {
        let test_case = match parse_test_case::<ErrorTree<Span>>(LocatedSpan::new(TEST_CASE)) {
            Ok((_, tc)) => tc,
            Err(_) => panic!("could not parse TEST_CASE"),
        };
        let matching_op = OPERATIONS.iter().filter(|o| test_case.matches(o)).count();
        assert_eq!(3, matching_op);
    }
}
