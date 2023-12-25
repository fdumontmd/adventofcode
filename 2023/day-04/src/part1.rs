use crate::{
    custom_error::AocError,
    parser::{self},
};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let mut score = 0;
    for line in input.lines() {
        let card = parser::parse(line)?;
        score += card.score();
    }
    Ok(format!("{}", score))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[rstest]
    #[case(INPUT, "13")]
    #[case(include_str!("../input.txt"), "24160")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
