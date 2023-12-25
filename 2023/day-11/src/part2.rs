use crate::custom_error::AocError;
use crate::puzzle::solve_for_shift;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    solve_for_shift(input, 1000000)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../input.txt");
        assert_eq!("710674907809", process(input)?);
        Ok(())
    }

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
    #[case(INPUT, 10, "1030")]
    #[case(INPUT, 100, "8410")]
    fn test_solve_for(
        #[case] input: &str,
        #[case] shift: usize,
        #[case] res: &str,
    ) -> miette::Result<()> {
        assert_eq!(res, solve_for_shift(input, shift)?);
        Ok(())
    }
}
