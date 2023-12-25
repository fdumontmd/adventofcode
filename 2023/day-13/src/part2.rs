use aoc_utils::grid::{Grid, Taxicab};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let mut sum = 0;
    for grid in input.split("\n\n") {
        let grid: Grid<char, Taxicab> = Grid::try_from(grid).unwrap();
        //
        // look for row
        'rows: for r in 1..grid.height() {
            let mut smudges = false;
            let m = r.min(grid.height() - r);

            for mr in 0..m {
                for c in 0..grid.width() {
                    if grid[(c, r - mr - 1)] != grid[(c, r + mr)] {
                        if smudges {
                            continue 'rows;
                        }
                        smudges = true;
                    }
                }
            }

            if smudges {
                sum += 100 * r;
            }
        }

        // look for columns
        'columns: for c in 1..grid.width() {
            let mut smudges = false;
            let m = c.min(grid.width() - c);

            for mc in 0..m {
                for r in 0..grid.height() {
                    if grid[(c - mc - 1, r)] != grid[(c + mc, r)] {
                        if smudges {
                            continue 'columns;
                        }
                        smudges = true;
                    }
                }
            }

            if smudges {
                sum += c;
            }
        }
    }

    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[rstest]
    #[case(INPUT, "400")]
    #[case(include_str!("../input.txt"), "30449")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
