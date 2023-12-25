use std::collections::HashSet;

use aoc_utils::union_find::UnionFind;
use itertools::Itertools;

use crate::{
    custom_error::AocError,
    part2::{parse_instruction_1, Direction, Trench},
};

// damn, that's clever: https://github.com/MatteoPierro/adventofcode-2023/blob/main/src/day18.rs
// shoelace formula + pick's theorem: instant solution
// need some work still to convince myself that this actually works...
pub fn sanity_check(input: &str) -> bool {
    let mut pos = (0, 0);
    let orig = Trench {
        top_left: (0, 0),
        bottom_right: (1, 1),
    };
    let mut trenches = vec![orig];
    // ignore the first cubic meter, it will overlap with the last
    // trench anyway
    let mut trench_area = 1;
    // test data and input are closed loops - only one intersection at (0,0)
    let mut prev_dir: Option<Direction> = None;
    for line in input.lines() {
        let (dir, dist) = parse_instruction_1(line);
        let delta = dir.delta();
        let trench = dir.make_trench(pos, dist);
        trench_area += trench.area();
        pos = (pos.0 + delta.0 * dist, pos.1 + delta.1 * dist);
        for i in trenches.iter().filter_map(|o| trench.intersection(o)) {
            trench_area -= i.area();
            if i != orig {
                eprintln!("Not a simple loop going back to (0,0)");
                return false;
            }
            println!("intersection: {i:?}");
        }
        trenches.push(trench);
        if let Some(prev_dir) = prev_dir {
            if prev_dir.is_horizontal() == dir.is_horizontal() {
                eprintln!("trenches do not alternate vertical/horizontal");
                return false;
            }
        }
        prev_dir = Some(dir);
    }
    eprintln!("trench area: {trench_area}");
    true
}

// try that one too: https://en.wikipedia.org/wiki/Shoelace_formula
#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let mut pos = (0, 0);
    let mut trench_len = 0;
    let mut area = 0;
    for line in input.lines() {
        let (dir, len) = parse_instruction_1(line);
        trench_len += len;

        let delta = dir.delta();
        pos = (pos.0 + delta.0 * len, pos.1 + delta.1 * len);
        if dir == Direction::Left {
            area += len * pos.1;
        } else if dir == Direction::Right {
            area -= len * pos.1;
        }
    }

    // Using Pick's theorem,
    // we have A = I + B/2 - 1, where we know A and B,
    // so I = A - B/2 + 1, and we need I + B, so
    // I + B = A + B/2 + 1
    //
    // boundary count is correct; each unit of trench
    // adds one integral coordinate on the boundary
    //
    // Ok, here's why it works: we convert the problem
    // from a block one to a point and line one:
    // the trenches are just line around a shape
    // the blocks are mapped to integral coordinates
    // The length of the trench is the length of the line
    // is the number of points on the line.
    // So the remaining question is: how many inside points
    // in the shape (that gives us the number of blocks inside
    // the trench).
    // Using shoelace, we get the area in the line world,
    // then Pick to compute the inside points, then back to
    // block we compute I + B (same as A + B/2 + 1)

    Ok(format!("{}", area + trench_len / 2 + 1))
}

// probably should try this instead https://en.wikipedia.org/wiki/Flood_fill
// still slow, but maybe not as much for part 1
// Even-odd:
// https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule
// Nonzero rule:
// https://en.wikipedia.org/wiki/Nonzero-rule
#[tracing::instrument]
pub fn process_slow(input: &str) -> Result<String, AocError> {
    if !sanity_check(input) {
        panic!("failed sanity check");
    }
    // dumb and slow
    // rewrite for part 2 and use that instead
    let mut pos = (0, 0);

    // need a filling algorithm
    let mut map = HashSet::new();
    map.insert(pos);

    for line in input.lines() {
        // ignore colour for now
        let parts: Vec<_> = line.split_whitespace().collect();
        let dir = parts[0];
        let len: usize = parts[1].parse().unwrap();

        let delta = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("unknown direction {dir} in {line}"),
        };

        for _ in 0..len {
            pos = (pos.0 + delta.0, pos.1 + delta.1);
            map.insert(pos);
        }
    }

    let (xmin, xmax) = match map.iter().map(|(x, _)| x).minmax() {
        itertools::MinMaxResult::NoElements => panic!("no x dimension"),
        itertools::MinMaxResult::OneElement(&x) => (x, x),
        itertools::MinMaxResult::MinMax(&xmin, &xmax) => (xmin, xmax),
    };
    let (xmin, xmax) = (xmin - 1, xmax + 1);

    let (ymin, ymax) = match map.iter().map(|(_, y)| y).minmax() {
        itertools::MinMaxResult::NoElements => panic!("no y dimension"),
        itertools::MinMaxResult::OneElement(&y) => (y, y),
        itertools::MinMaxResult::MinMax(&ymin, &ymax) => (ymin, ymax),
    };

    let (ymin, ymax) = (ymin - 1, ymax + 1);

    let width = (xmax - xmin + 1) as usize;

    let mut uf = UnionFind::new();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let idx = (x - xmin) as usize + ((y - ymin) as usize) * width;
            let wall = map.contains(&(x, y));
            // check 4 neighbours
            // up
            let n = (x, (y - 1).clamp(ymin, ymax));
            let idx_n = (n.0 - xmin) as usize + ((n.1 - ymin) as usize) * width;
            if wall == map.contains(&n) {
                uf.join(idx, idx_n);
            }
            // left
            let n = ((x - 1).clamp(xmin, xmax), y);
            let idx_n = (n.0 - xmin) as usize + ((n.1 - ymin) as usize) * width;
            if wall == map.contains(&n) {
                uf.join(idx, idx_n);
            }
            // down
            let n = (x, (y + 1).clamp(ymin, ymax));
            let idx_n = (n.0 - xmin) as usize + ((n.1 - ymin) as usize) * width;
            if wall == map.contains(&n) {
                uf.join(idx, idx_n);
            }
            // right
            let n = ((x + 1).clamp(xmin, xmax), y);
            let idx_n = (n.0 - xmin) as usize + ((n.1 - ymin) as usize) * width;
            if wall == map.contains(&n) {
                uf.join(idx, idx_n);
            }
        }
    }
    let inside = uf.len() - uf.same_group(0).len();

    Ok(format!("{inside}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[rstest]
    #[case(INPUT, "62")]
    fn test_process_slow(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process_slow(input)?);
        Ok(())
    }

    #[rstest]
    #[case(INPUT, "62")]
    #[case(include_str!("../input.txt"), "47139")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
