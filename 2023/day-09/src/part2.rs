use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let prev: i64 = input
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            predict_prev_number(sequence)
        })
        .sum();
    Ok(format!("{prev}"))
}

fn predict_prev_number(mut sequence: Vec<i64>) -> i64 {
    let mut first = Vec::new();
    loop {
        first.push(*sequence.first().unwrap());
        sequence = sequence.windows(2).map(|c| c[1] - c[0]).collect();
        if sequence.iter().all(|n| *n == 0) {
            first.reverse();
            return first.into_iter().fold(0, |p, n| n - p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("2", process(INPUT)?);
        Ok(())
    }
}
