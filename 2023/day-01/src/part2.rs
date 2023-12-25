use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let first_digit = [
                line.find("one").map(|p| (p, 1)),
                line.find("two").map(|p| (p, 2)),
                line.find("three").map(|p| (p, 3)),
                line.find("four").map(|p| (p, 4)),
                line.find("five").map(|p| (p, 5)),
                line.find("six").map(|p| (p, 6)),
                line.find("seven").map(|p| (p, 7)),
                line.find("eight").map(|p| (p, 8)),
                line.find("nine").map(|p| (p, 9)),
                line.find(|c: char| c.is_ascii_digit())
                    .map(|p| (p, line.chars().nth(p).unwrap().to_digit(10).unwrap())),
            ]
            .into_iter()
            .flatten()
            .min()
            .unwrap();
            let last_digit = [
                line.rfind("one").map(|p| (p, 1)),
                line.rfind("two").map(|p| (p, 2)),
                line.rfind("three").map(|p| (p, 3)),
                line.rfind("four").map(|p| (p, 4)),
                line.rfind("five").map(|p| (p, 5)),
                line.rfind("six").map(|p| (p, 6)),
                line.rfind("seven").map(|p| (p, 7)),
                line.rfind("eight").map(|p| (p, 8)),
                line.rfind("nine").map(|p| (p, 9)),
                line.rfind(|c: char| c.is_ascii_digit())
                    .map(|p| (p, line.chars().nth(p).unwrap().to_digit(10).unwrap())),
            ]
            .into_iter()
            .flatten()
            .max()
            .unwrap();
            first_digit.1 * 10 + last_digit.1
        })
        .sum();
    miette::Result::Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    static INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[tracing::instrument(level = "trace", skip(input, result))]
    #[rstest]
    #[case(INPUT, "281")]
    #[case(include_str!("../input.txt"), "53866")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
