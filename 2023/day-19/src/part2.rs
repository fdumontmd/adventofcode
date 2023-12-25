use std::{collections::HashMap, ops::Range};

use crate::{
    custom_error::AocError,
    parser::{parse_puzzle, Category, Comparison, Workflow},
};

// the code used to be dumber, so some dumbness still peeks through
#[derive(Debug, Clone)]
struct Part {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl Part {
    fn new() -> Self {
        Part {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }

    fn len(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

struct Cut {
    category: Category,
    target: u32,
    comparison: Comparison,
}

impl Cut {
    fn apply(&self, part: &mut Part) {
        let cat = match self.category {
            Category::X => &mut part.x,
            Category::M => &mut part.m,
            Category::A => &mut part.a,
            Category::S => &mut part.s,
        };
        let range = self.comparison.to_range(self.target);
        *cat = (cat.start.max(range.start))..(cat.end.min(range.end));
        //cat.retain(|i| self.comparison.eval(*i, self.target));
    }

    fn invert(&self) -> Self {
        let (comparison, target) = self.comparison.invert(self.target);
        Self {
            comparison,
            target,
            ..*self
        }
    }
}

fn eval_workflow(
    workflow: &Workflow<'static>,
    workflows: &HashMap<&'static str, Workflow<'static>>,
    mut part: Part,
) -> usize {
    let mut output = 0;
    for condition in &workflow.conditions {
        match condition {
            crate::parser::Condition::Command(c) => match c {
                crate::parser::Command::Goto(wf) => {
                    output += eval_workflow(&workflows[wf], workflows, part);
                    return output;
                }
                crate::parser::Command::Accept => {
                    output += part.len();
                    return output;
                }
                crate::parser::Command::Reject => return output,
            },
            crate::parser::Condition::Compare(category, comparison, target, c) => {
                let cut = Cut {
                    category: *category,
                    comparison: *comparison,
                    target: *target,
                };
                let orig_len = part.len();
                let mut applied = part.clone();
                cut.apply(&mut applied);
                cut.invert().apply(&mut part);
                assert_eq!(orig_len, applied.len() + part.len());

                match c {
                    crate::parser::Command::Goto(wf) => {
                        output += eval_workflow(&workflows[wf], workflows, applied);
                    }
                    crate::parser::Command::Accept => {
                        output += applied.len();
                    }
                    crate::parser::Command::Reject => {}
                }
            }
        }
    }
    panic!("cannot evaluate {}", workflow.name)
}

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let (workflows, _) = parse_puzzle(input)?;
    let workflows: HashMap<&'static str, Workflow<'static>> =
        HashMap::from_iter(workflows.into_iter().map(|w| (w.name, w)));
    // each workflow is used only once, so no need for caching
    // idea: pass a part with full range to workflow 'in', and eval each condition
    // each condition applies a cut (redution of valid range), and maybe pass the part to a
    // sub workflow; A returns the passed range; R stops evaluation
    // each condition cut the original part range in two: one that applies
    // and one that does not; the one that does not is passed to the next
    // condition
    // a x>1234:R only passes the part range that does not apply to the next
    // condition
    let start = &workflows["in"];
    let part = Part::new();
    let output = eval_workflow(start, &workflows, part);

    Ok(format!("{}", output))
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
    #[case(INPUT, "167409079868000")]
    #[case(include_str!("../input.txt"), "131029523269531")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
