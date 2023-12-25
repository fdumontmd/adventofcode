use std::collections::{HashSet, VecDeque};

use aoc_utils::grid::{Grid, Taxicab};

use crate::{custom_error::AocError, part1::Tile};

type Position = (usize, usize);
type GridId = (isize, isize);

fn reachable_in(grid: &Grid<Tile, Taxicab>, from: (usize, usize), max_steps: usize) -> usize {
    let mut visited: HashSet<(bool, Position, GridId)> = HashSet::new();

    // need a Deque to ensure BFS; otherwise might reach max_steps through
    // too long a path and not reach plots that should have been reached
    let mut queue = VecDeque::new();
    queue.push_back((0, from, (0, 0)));

    while let Some((steps, pos, grid_id)) = queue.pop_front() {
        if visited.contains(&(steps % 2 == 0, pos, grid_id)) {
            continue;
        }

        visited.insert((steps % 2 == 0, pos, grid_id));

        if steps >= max_steps {
            continue;
        }

        for n in grid.neighbours(pos) {
            if grid[n] == Tile::Plot {
                queue.push_back((steps + 1, n, grid_id));
            }
        }
        if pos.0 == 0 {
            queue.push_back((
                steps + 1,
                (grid.width() - 1, pos.1),
                (grid_id.0 - 1, grid_id.1),
            ));
        }
        if pos.0 == grid.width() - 1 {
            queue.push_back((steps + 1, (0, pos.1), (grid_id.0 + 1, grid_id.1)));
        }

        if pos.1 == 0 {
            queue.push_back((
                steps + 1,
                (pos.0, grid.height() - 1),
                (grid_id.0, grid_id.1 - 1),
            ));
        }
        if pos.1 == grid.height() - 1 {
            queue.push_back((steps + 1, (pos.0, 0), (grid_id.0, grid_id.1 + 1)));
        }
    }
    visited
        .into_iter()
        .filter(|(steps, _, _)| *steps == (max_steps % 2 == 0))
        .count()
    /*
    // target number is odd, so some tiles will never be accessible.
    // even/odd separation: if your path len is odd, steps reached in even
    // steps are forever out of reach
    // BUT: changing from one grid to the next consumes 1 step, changing
    // the oddness of the number of steps, so the next grid accessible
    // tiles is changed. However, leaving then coming back costs an even
    // number of steps, so unreachable tiles remain unreachable

    // the actual input grid is such that reaching the edge can be done
    // straight from the center => compute the number of odd tiles from center,
    // + even tiles
    //
    // we have the relation that (26501365-65) / 131 == 202300
    //
    // need 66 steps to leave from center
    // all reachable plots are reachable in 260 steps at most
    */
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    // sanity check
    //println!("64: {}", reachable_in(input, 64));
    // f(n) is computing the number of reachable plots
    // in 65 + n * 131 steps. f(n) is a quadratic, so
    // f(202300) would be our solution
    // 65 + 131 * 202300 == 26501365
    let mut grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    let pos = grid.idx_to_pos(grid.iter().position(|&t| t == Tile::Start).unwrap());
    grid[pos] = Tile::Plot;
    /*
    println!("65: {}", reachable_in(&grid, 65));
    println!("65 + 131: {}", reachable_in(&grid, 65 + 131));
    println!("65 + 2 * 131: {}", reachable_in(&grid, 65 + 2 * 131));
    */

    let y0 = reachable_in(&grid, pos, 65);
    let y1 = reachable_in(&grid, pos, 65 + 131);
    let y2 = reachable_in(&grid, pos, 65 + 2 * 131);

    let a0 = y0;
    let a1 = y1 - y0;
    let a2 = (y2 - 2 * y1 + y0) / 2;

    // f(n) = a0 + (a1 - a2) * n + a2 * n^2
    /*
    let y3 = reachable_in(&grid, pos, 65 + 3 * 131);
    let check = a0 + (a1 - a2) * 3 + a2 * (3 * 3);
    assert_eq!(check, y3);
    */
    let count = a0 + (a1 - a2) * 202300 + a2 * (202300 * 202300);

    // ok, that's bad, but I used Wolfram Alpha... next is to compute
    // this quadratic myself, and input the target value

    // OUTLINE OF THE SOLUTION
    // as I already found out, the test data is useless; the actual input
    // is nicely structured so that the overall shape is going to be a diamond
    // (AKA a Taxicab Circle). The core idea here is that the number of reachable
    // plots is going to be constant in each unit grid, so that as we increase the
    // number of steps, the number of plots grows quadratically (the diamond extends
    // in two dimensions as the steps increase, hence x^2)
    // now, the smart idea: work out the first few values for a f(n) where n is a nice
    // variable taking into account that 65 + n * 131 is 26501365 when n = 202300,
    // then use an algo to find the corresponding quadratic.
    // see https://en.wikipedia.org/wiki/Newton_polynomial for a way to do so
    // actually used
    // https://pythonnumericalmethods.berkeley.edu/notebooks/chapter17.05-Newtons-Polynomial-Interpolation.html
    // instead as it is easier to understand

    Ok(format!("{count}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../input.txt");
        assert_eq!("625382480005896", process(input)?);
        Ok(())
    }

    static INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[rstest]
    #[case(INPUT, 6, 16)]
    #[case(INPUT, 10, 50)]
    #[case(INPUT, 50, 1594)]
    #[case(INPUT, 100, 6536)]
    #[case(INPUT, 500, 167004)]
    #[case(INPUT, 1000, 668697)]
    // takes more than 10 seconds in release mode, and way too long in debug
    //#[case(INPUT, 5000, 16733044)]
    #[case(include_str!("../input.txt"), 64, 3737)]
    fn test_reachable_in(#[case] input: &str, #[case] steps: usize, #[case] reachable: usize) {
        let mut grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();
        let pos = grid.idx_to_pos(grid.iter().position(|&t| t == Tile::Start).unwrap());
        grid[pos] = Tile::Plot;

        assert_eq!(reachable, reachable_in(&grid, pos, steps));
    }
}
