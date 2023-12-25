use aoc_utils::grid::{Grid, Taxicab};

use crate::{
    custom_error::AocError,
    puzzle::{solve_from, Direction, Tile},
};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    // idea: compress the map (which is very sparse) by replacing
    // it with a graph joining the edges, mirrors and splitters
    // (combined with directions) to each others

    // dumb but correct
    let max = (0..grid.width())
        .map(|c| solve_from(&grid, (c, 0), Direction::Down))
        .chain((0..grid.width()).map(|c| solve_from(&grid, (c, grid.height() - 1), Direction::Up)))
        .chain((0..grid.height()).map(|r| solve_from(&grid, (0, r), Direction::Right)))
        .chain(
            (0..grid.height()).map(|r| solve_from(&grid, (grid.width() - 1, r), Direction::Left)),
        )
        .max()
        .unwrap();

    Ok(format!("{max}"))
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
    #[case(INPUT, "51")]
    #[case(include_str!("../input.txt"), "7943")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
