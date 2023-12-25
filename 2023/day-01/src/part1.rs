use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let values: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect();

    let values: Vec<u32> = values
        .into_iter()
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .collect();

    let sum: u32 = values.into_iter().sum();

    miette::Result::Ok(format!("{}", sum))
    //todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    static INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static REAL_INPUT: &str = include_str!("../input.txt");

    #[tracing::instrument(level = "trace", skip(input, result))]
    #[rstest]
    #[case(INPUT, "142")]
    #[case(REAL_INPUT, "54159")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
