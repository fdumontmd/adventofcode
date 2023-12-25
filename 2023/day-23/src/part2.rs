use std::collections::{HashMap, HashSet};

use aoc_utils::grid::{Grid, Taxicab};

use crate::{
    custom_error::AocError,
    part1::{Position, Tile},
};

#[derive(Debug, Clone)]
struct State {
    pos: Position,
    visited: HashSet<Position>,
    len: usize,
}

impl State {
    fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert((1, 0));
        State {
            pos: (1, 0),
            visited,
            len: 0,
        }
    }
}

// still not quite as fast as I'd like
// check below for possible optimizations
// https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/23.rs
#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    let mut branches = Vec::new();

    // find all branches: positions where valid neighbours > 2
    for pos in grid.iter().enumerate().filter_map(|(idx, t)| {
        if *t != Tile::Forest {
            Some(grid.idx_to_pos(idx))
        } else {
            None
        }
    }) {
        if grid
            .neighbours(pos)
            .filter(|n| grid[*n] != Tile::Forest)
            .count()
            > 2
        {
            branches.push(pos);
        }
    }

    branches.push((1, 0));

    let mut paths: HashMap<Position, Vec<(Position, usize)>> = HashMap::new();

    for b in branches {
        for n in grid.neighbours(b).filter(|n| grid[*n] != Tile::Forest) {
            let mut curr = n;
            let mut prev = b;
            // ugly hack to handle the fact that the origin is not a branch
            let mut dist = 0;
            loop {
                let neighbours: Vec<Position> = grid
                    .neighbours(curr)
                    .filter(|&next| next != prev && grid[next] != Tile::Forest)
                    .collect();
                if neighbours.len() == 1 {
                    prev = curr;
                    curr = neighbours[0];
                    dist += 1;
                } else {
                    paths.entry(b).or_default().push((curr, dist + 1));
                    break;
                }
            }
        }
    }

    let len = dfs((1, 0), grid.height() - 1, &paths, &mut HashSet::new(), 0);

    Ok(format!("{len}"))
}

fn dfs(
    cur: Position,
    target: usize,
    paths: &HashMap<Position, Vec<(Position, usize)>>,
    visited: &mut HashSet<Position>,
    len: usize,
) -> usize {
    if cur.1 == target {
        len
    } else {
        let mut best = None;
        for (n, d) in &paths[&cur] {
            if visited.contains(n) {
                continue;
            }
            visited.insert(*n);
            let l = dfs(*n, target, paths, visited, len + d);
            best = Some(best.unwrap_or(0).max(l));
            visited.remove(n);
        }
        best.unwrap_or(0)
    }
}

#[tracing::instrument]
pub fn process_slow(input: &str) -> Result<String, AocError> {
    let grid: Grid<Tile, Taxicab> = Grid::try_from(input).unwrap();

    let mut branches = Vec::new();

    // find all branches: positions where valid neighbours > 2
    for pos in grid.iter().enumerate().filter_map(|(idx, t)| {
        if *t != Tile::Forest {
            Some(grid.idx_to_pos(idx))
        } else {
            None
        }
    }) {
        if grid
            .neighbours(pos)
            .filter(|n| grid[*n] != Tile::Forest)
            .count()
            > 2
        {
            branches.push(pos);
        }
    }

    branches.push((1, 0));

    // for each branch, follow non-branching paths until end, and record
    // ends + dist
    // we start on the exit of a branch, so each has only one end

    let mut paths = HashMap::new();

    for b in branches {
        for n in grid.neighbours(b).filter(|n| grid[*n] != Tile::Forest) {
            let mut curr = n;
            let mut prev = b;
            // ugly hack to handle the fact that the origin is not a branch
            let mut dist = 0;
            loop {
                let neighbours: Vec<Position> = grid
                    .neighbours(curr)
                    .filter(|&next| next != prev && grid[next] != Tile::Forest)
                    .collect();
                if neighbours.len() == 1 {
                    prev = curr;
                    curr = neighbours[0];
                    dist += 1;
                } else {
                    paths.insert(n, (curr, dist));
                    break;
                }
            }
        }
    }

    let Some((b, d)) = paths.remove(&(1, 1)) else {
        panic!("starting point not mapped")
    };
    paths.insert((1, 0), (b, d + 1));

    let mut queue = vec![State::new()];

    let mut max_path: Option<State> = None;

    // not quite as fast as I'd like
    while let Some(mut s) = queue.pop() {
        let Some((dst, dist)) = paths.get(&s.pos) else {
            panic!("cannot find path from {:?}", s.pos)
        };
        if s.visited.contains(dst) {
            continue;
        }
        s.visited.insert(*dst);
        s.len += dist;
        s.pos = *dst;

        if s.pos.1 == grid.height() - 1 {
            let curr = max_path.as_ref().map(|s| s.len).unwrap_or(0);
            if s.len > curr {
                max_path = Some(s);
            }
        } else {
            for n in grid.neighbours(s.pos).filter(|n| grid[*n] != Tile::Forest) {
                let mut ns = s.clone();
                ns.len += 1;
                ns.pos = n;
                queue.push(ns);
            }
        }
    }

    let len = max_path.map(|s| s.len).unwrap();
    Ok(format!("{len}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    static INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    use super::*;
    #[rstest]
    #[case(INPUT, "154")]
    fn test_process_slow(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process_slow(input)?);
        Ok(())
    }

    #[rstest]
    #[case(INPUT, "154")]
    //#[case(include_str!("../input.txt"), "6538")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
