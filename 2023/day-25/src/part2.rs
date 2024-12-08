use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, AocError> {
    Ok("finished!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("finished!", process(input)?);
        Ok(())
    }
}
