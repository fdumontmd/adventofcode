use crate::custom_error::AocError;
use std::convert::TryFrom;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let [time, dist] = <_>::try_from(input.lines().collect::<Vec<_>>()).unwrap();
    let time = time.strip_prefix("Time:").unwrap();
    let times: Vec<_> = time
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();
    let dist = dist.strip_prefix("Distance:").unwrap();
    let dists: Vec<_> = dist
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect();

    let res: usize = times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(t, d)| (0..t).filter(|p| (t - p) * p > d).count())
        .product();
    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[rstest]
    #[case(INPUT, "288")]
    #[case(include_str!("../input.txt"), "219849")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
