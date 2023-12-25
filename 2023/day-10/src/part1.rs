use aoc_utils::grid::{Grid, Taxicab};

use crate::{
    custom_error::AocError,
    puzzle::{Direction, Tile},
};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let g: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();
    let start = g.iter().position(|t| *t == Tile::Start).unwrap();
    let start = g.idx_to_pos(start);

    // just need to know how to start; there should be two directions we can follow
    let mut exits = Vec::new();
    // consider rewriting using grid.around_pos instead of this

    // West
    if start.0 > 0 {
        let np = (start.0 - 1, start.1);
        if g[np].valid_for_dir(Direction::West) {
            exits.push((Direction::West, np));
        }
    }

    // East
    if start.0 < g.width() - 1 {
        let np = (start.0 + 1, start.1);
        if g[np].valid_for_dir(Direction::East) {
            exits.push((Direction::East, np));
        }
    }

    // North
    if start.1 > 0 {
        let np = (start.0, start.1 - 1);
        if g[np].valid_for_dir(Direction::North) {
            exits.push((Direction::North, np));
        }
    }

    // South
    if start.1 < g.height() - 1 {
        let np = (start.0, start.1 + 1);
        if g[np].valid_for_dir(Direction::South) {
            exits.push((Direction::South, np));
        }
    }

    assert_eq!(2, exits.len());

    let mut pos = start;
    let mut dir = exits[0].0;
    let mut steps = 0;

    loop {
        let d = dir.delta();
        pos = (
            pos.0.checked_add_signed(d.0).unwrap(),
            pos.1.checked_add_signed(d.1).unwrap(),
        );
        steps += 1;
        if g[pos] == Tile::Start {
            break;
        }
        dir = g[pos].step(dir);
    }

    Ok(format!("{}", steps / 2))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT11: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";
    static INPUT12: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    static INPUT21: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    static INPUT22: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[rstest]
    #[case(INPUT11, "4")]
    #[case(INPUT12, "4")]
    #[case(INPUT21, "8")]
    #[case(INPUT22, "8")]
    #[case(include_str!("../input.txt"), "6717")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
