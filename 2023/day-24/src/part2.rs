use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, AocError> {
    // adapt the z3 solver, because I'm not going to do that
    // myself
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[rstest]
    #[case(INPUT, "47")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
