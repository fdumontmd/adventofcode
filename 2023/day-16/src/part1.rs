use aoc_utils::grid::{Grid, Taxicab};

use crate::{
    custom_error::AocError,
    puzzle::{solve_from, Direction, Tile},
};
#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    Ok(format!("{}", solve_from(&grid, (0, 0), Direction::Right)))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[rstest]
    #[case(INPUT, "46")]
    #[case(include_str!("../input.txt"), "7415")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
