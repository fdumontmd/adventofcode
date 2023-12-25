use crate::{custom_error::AocError, puzzle::solve_for_shift};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    solve_for_shift(input, 2)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[rstest]
    #[case(INPUT, "374")]
    #[case(include_str!("../input.txt"), "9329143")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
