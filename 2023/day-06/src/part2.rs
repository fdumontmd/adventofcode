use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let [time, dist] = <_>::try_from(input.lines().collect::<Vec<_>>()).unwrap();
    let time = time.strip_prefix("Time:").unwrap();
    let dist = dist.strip_prefix("Distance:").unwrap();
    let time: i64 = time.replace(' ', "").parse().unwrap();
    let dist: i64 = dist.replace(' ', "").parse().unwrap();

    // looks like a integer-valued quadratic equation
    let d = time * time - 4 * dist;
    let d = (d as f64).sqrt().floor() as i64;
    let t1 = (-time + d) / 2;
    let t1 = t1.abs();
    let t2 = (-time - d) / 2;
    let t2 = t2.abs();

    // completely unscientific calibration... find the smallest/largest time around
    // the computed solutions; seems to work well enough for both test and actual
    // input
    let t1 = (t1 - 2..t1 + 2)
        .filter(|t| t * (time - t) > dist)
        .min()
        .unwrap();
    let t2 = (t2 - 2..t2 + 2)
        .filter(|t| t * (time - t) > dist)
        .max()
        .unwrap();

    // add 1 as both t1 and t2 are included
    Ok(format!("{}", t2 - t1 + 1))
}

#[tracing::instrument]
pub fn process_brute_force(input: &str) -> Result<String, AocError> {
    let [time, dist] = <_>::try_from(input.lines().collect::<Vec<_>>()).unwrap();
    let time = time.strip_prefix("Time:").unwrap();
    let dist = dist.strip_prefix("Distance:").unwrap();
    let time: i64 = time.replace(' ', "").parse().unwrap();
    let dist: i64 = dist.replace(' ', "").parse().unwrap();
    let count = (0..time).filter(|p| (time - p) * p > dist).count();
    Ok(format!("{}", count))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[rstest]
    #[case(INPUT, "71503")]
    #[case(include_str!("../input.txt"), "29432455")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
