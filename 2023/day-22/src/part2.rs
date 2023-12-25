use std::collections::HashSet;

use crate::{custom_error::AocError, part1::compute_support};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let (supports, supported_by) = compute_support(input);

    let critical_bricks: Vec<usize> = supports
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| {
            if v.iter().filter(|s| supported_by[**s] == 1).count() > 0 {
                Some(idx)
            } else {
                None
            }
        })
        .collect();
    let count: usize = critical_bricks
        .iter()
        .map(|idx| {
            let mut supported_by = supported_by.clone();
            let mut falling: HashSet<usize> = HashSet::new();
            let mut queue = vec![*idx];
            while let Some(idx) = queue.pop() {
                if falling.contains(&idx) {
                    continue;
                }
                falling.insert(idx);

                for s in &supports[idx] {
                    supported_by[*s] -= 1;
                    if supported_by[*s] == 0 {
                        queue.push(*s);
                    }
                }
            }

            falling.len() - 1
        })
        .sum();

    //eprintln!("{critical_bricks:?}");

    Ok(format!("{count}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[rstest]
    #[case(INPUT, "7")]
    #[case(include_str!("../input.txt"), "96356")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
