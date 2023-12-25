use crate::{custom_error::AocError, puzzle};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let puzzle = puzzle::parse(input)?;

    let mut state = "AAA";
    loop {
        for (i, d) in puzzle.directions.iter().cycle().enumerate() {
            if state == "ZZZ" {
                return Ok(format!("{i}"));
            }
            state = d.follow(&puzzle.turns[state]);
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    static INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    #[rstest]
    #[case(INPUT1, "2")]
    #[case(INPUT2, "6")]
    #[case(include_str!("../input.txt"), "17873")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
