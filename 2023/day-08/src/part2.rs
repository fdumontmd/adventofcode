use aoc_utils::num::extended_gcd;

use crate::{custom_error::AocError, puzzle};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let puzzle = puzzle::parse(input)?;

    // ok, turns out that ??Z do not go back to the corresponding ??A
    // so, no idea how this solution is even close to correct...
    // CRT would be better here? somehow for my input lcm is enough,
    // but can't really assume that for the general case
    // ok, from reddit, looks like lcm was enough for generated input
    // but would be nice to confirm that this was indeed the case
    let lcm = puzzle
        .turns
        .keys()
        .filter(|l| l.ends_with('A'))
        .cloned()
        .map(|start| {
            let mut state = start;
            'search: loop {
                for (i, d) in puzzle.directions.iter().cycle().enumerate() {
                    if state.ends_with('Z') {
                        break 'search i;
                    }
                    state = d.follow(&puzzle.turns[state]);
                }
            }
        })
        .fold(1, |s, n| (s * n as i64) / extended_gcd(s, n as i64).0);
    Ok(format!("{lcm}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[rstest]
    #[case(INPUT, "6")]
    #[case(include_str!("../input.txt"), "15746133679061")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
