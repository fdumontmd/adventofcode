use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let next: i64 = input
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            predict_next_number(sequence)
        })
        .sum();
    Ok(format!("{next}"))
}

fn predict_next_number(mut sequence: Vec<i64>) -> i64 {
    let mut next = 0;
    loop {
        next += *sequence.last().unwrap();
        sequence = sequence.windows(2).map(|c| c[1] - c[0]).collect();
        if sequence.iter().all(|n| *n == 0) {
            return next;
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[rstest]
    #[case(INPUT, "114")]
    #[case(include_str!("../input.txt"), "1882395907")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
