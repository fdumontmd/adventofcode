use crate::{custom_error::AocError, parser::parse};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let puzzle = parse(input)?;

    // I mean, yes, it is dumb, but fast enough for Day 03
    let sum: u32 = puzzle
        .symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .filter_map(|s| {
            let parts: Vec<_> = puzzle
                .numbers
                .iter()
                .filter(|n| n.rect.contains(&s.pos))
                .collect();
            if parts.len() == 2 {
                Some(parts.into_iter().map(|n| n.n).product::<u32>())
            } else {
                None
            }
        })
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
        assert_eq!("467835", process(INPUT)?);
        Ok(())
    }
}
