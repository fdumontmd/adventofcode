use crate::{custom_error::AocError, hash};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let input = input.trim();
    let sum: usize = input.split(',').map(hash).sum();
    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[rstest]
    #[case(INPUT, "1320")]
    #[case(include_str!("../input.txt"), "516657")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
