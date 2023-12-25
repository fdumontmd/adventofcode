use std::collections::HashMap;

use crate::{
    custom_error::AocError,
    parser::{parse_puzzle, Command, Workflow},
};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let (workflows, parts) = parse_puzzle(input)?;
    let workflows: HashMap<&'static str, Workflow<'static>> =
        HashMap::from_iter(workflows.into_iter().map(|w| (w.name, w)));

    let sum: u64 = parts
        .into_iter()
        .filter_map(|part| {
            let mut cur_wf = "in";
            loop {
                match workflows[cur_wf].eval(&part) {
                    Command::Goto(wf) => cur_wf = wf,
                    Command::Accept => return Some(part),
                    Command::Reject => return None,
                }
            }
        })
        .map(|p| p.rating())
        .sum();
    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[rstest]
    #[case(INPUT, "19114")]
    #[case(include_str!("../input.txt"), "386787")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
