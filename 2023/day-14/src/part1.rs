use aoc_utils::grid::{Grid, Taxicab};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let mut load = 0;
    let grid: Grid<char, Taxicab> = Grid::try_from(input).unwrap();

    for c in 0..grid.width() {
        // for each column, iterate from the first (North) row
        let mut empty = 0;
        for r in 0..grid.height() {
            match grid[(c, r)] {
                'O' => {
                    load += grid.height() - empty;
                    empty += 1;
                }
                '#' => {
                    empty = r + 1;
                }
                _ => {}
            }
        }
    }

    Ok(format!("{load}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
";

    #[rstest]
    #[case(INPUT, "136")]
    #[case(include_str!("../input.txt"), "108792")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
