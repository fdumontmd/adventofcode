use std::collections::{BinaryHeap, HashMap};

use aoc_utils::grid::{Grid, Taxicab};

use crate::{
    custom_error::AocError,
    puzzle::{Direction, State},
};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let mut grid: Grid<u8, Taxicab> = Grid::try_from(input).unwrap();
    grid.iter_mut().for_each(|b| *b -= b'0');
    let mut queue = BinaryHeap::new();
    queue.push(State::new(Direction::Right).get_key(&grid));
    queue.push(State::new(Direction::Down).get_key(&grid));

    let mut seen = HashMap::new();

    while let Some((_hl, s)) = queue.pop() {
        if s.pos.0 == grid.width() - 1 && s.pos.1 == grid.height() - 1 {
            return Ok(format!("{}", s.heat_loss));
        }

        // this is the problem
        if let Some(hl) = seen.get(&(s.pos, s.direction, s.straight)) {
            if *hl <= s.heat_loss {
                continue;
            }
        }

        seen.insert((s.pos, s.direction, s.straight), s.heat_loss);

        for succ in s.successors(&grid) {
            queue.push(succ.get_key(&grid));
        }
    }
    panic!("no solution found")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[rstest]
    #[case(INPUT, "102")]
    #[case(include_str!("../input.txt"), "758")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
