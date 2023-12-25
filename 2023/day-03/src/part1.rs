use crate::{custom_error::AocError, parser::parse};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let puzzle = parse(input)?;

    let sum: u32 = puzzle
        .numbers
        .iter()
        .filter(|n| puzzle.symbols.iter().any(|s| n.rect.contains(&s.pos)))
        .map(|n| n.n)
        .sum();
    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("4361", process(INPUT)?);
        Ok(())
    }
}
