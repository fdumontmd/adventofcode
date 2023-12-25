use std::collections::HashSet;

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
    let mut enclosure = HashSet::new();

    loop {
        let d = dir.delta();
        enclosure.insert(pos);
        pos = (
            pos.0.checked_add_signed(d.0).unwrap(),
            pos.1.checked_add_signed(d.1).unwrap(),
        );
        if g[pos] == Tile::Start {
            break;
        }
        dir = g[pos].step(dir);
    }

    let mut exits: Vec<Direction> = exits.into_iter().map(|d| d.0).collect();
    exits.sort();

    let start = match exits.as_slice() {
        [Direction::North, Direction::South] => Tile::Vert,
        [Direction::West, Direction::East] => Tile::Horiz,
        [Direction::North, Direction::West] => Tile::NW,
        [Direction::North, Direction::East] => Tile::NE,
        [Direction::South, Direction::West] => Tile::SW,
        [Direction::South, Direction::East] => Tile::SE,
        _ => panic!("unknown orientation {:?}", exits),
    };

    let mut g = g;

    g[pos] = start;

    let g = g;

    let mut count = 0;

    for y in 0..g.height() {
        let mut inside = false;
        let mut step_down = false;
        let mut step_up = false;
        for x in 0..g.width() {
            if enclosure.contains(&(x, y)) {
                let t = g[(x, y)];
                if t == Tile::Vert {
                    inside = !inside;
                    assert!(!step_up);
                    assert!(!step_down);
                }

                if t == Tile::NE {
                    step_down = true;
                }

                if t == Tile::SE {
                    step_up = true;
                }

                if t == Tile::SW {
                    if step_down {
                        inside = !inside;
                    }
                    step_up = false;
                    step_down = false;
                }

                if t == Tile::NW {
                    if step_up {
                        inside = !inside;
                    }
                    step_up = false;
                    step_down = false;
                }
            } else if inside {
                count += 1;
            }
        }
    }

    Ok(format!("{count}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT1: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    static INPUT2: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

    static INPUT3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    static INPUT4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[rstest]
    #[case(INPUT1, "4")]
    #[case(INPUT2, "4")]
    #[case(INPUT3, "8")]
    #[case(INPUT4, "10")]
    #[case(include_str!("../input.txt"), "381")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
