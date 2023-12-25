use std::collections::HashMap;

use crate::{custom_error::AocError, solve_at};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let sum = input.lines().map(count_matches).sum::<usize>();
    Ok(format!("{sum}"))
}

fn count_matches(l: &str) -> usize {
    let puzzle = l.split_whitespace().collect::<Vec<_>>();
    assert_eq!(2, puzzle.len());
    let pattern = puzzle[0].as_bytes();
    let pattern = pattern
        .iter()
        .cloned()
        .chain(Some(b'?'))
        .cycle()
        .take(pattern.len() * 5 + 4)
        .collect::<Vec<_>>();
    let block_lens = puzzle[1]
        .split(',')
        .map(|b| b.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let block_lens = block_lens
        .iter()
        .cloned()
        .cycle()
        .take(5 * block_lens.len())
        .collect::<Vec<_>>();
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    solve_at(0, 0, &pattern, &block_lens, &mut memo)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[rstest]
    #[case(INPUT, "525152")]
    #[case(include_str!("../input.txt"), "3384337640277")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
