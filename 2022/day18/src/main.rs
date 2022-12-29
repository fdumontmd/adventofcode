use std::collections::HashSet;

use aoc_utils::union_find::UnionFind;
use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

type Coord = (i32, i32, i32);

fn neighbours(pos: Coord) -> impl Iterator<Item = Coord> {
    [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ]
    .into_iter()
    .map(move |d| (d.0 + pos.0, d.1 + pos.1, d.2 + pos.2))
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (x, y, z) = l.split(',').tuples().next().unwrap();
            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect()
}

fn part_01(grid: &[Coord]) -> usize {
    let model: HashSet<Coord> = HashSet::from_iter(grid.iter().cloned());
    let mut faces = 0;
    for pos in &model {
        for n in neighbours(*pos) {
            if !model.contains(&n) {
                faces += 1;
            }
        }
    }
    faces
}

fn dim(grid: &[Coord]) -> (Coord, Coord) {
    let min_x = grid.iter().map(|p| p.0).min().unwrap();
    let max_x = grid.iter().map(|p| p.0).max().unwrap();
    let min_y = grid.iter().map(|p| p.1).min().unwrap();
    let max_y = grid.iter().map(|p| p.1).max().unwrap();
    let min_z = grid.iter().map(|p| p.2).min().unwrap();
    let max_z = grid.iter().map(|p| p.2).max().unwrap();
    ((min_x, min_y, min_z), (max_x, max_y, max_z))
}

struct Id {
    base: (i32, i32, i32),
    dim: (usize, usize, usize),
}

impl Id {
    fn id(&self, pos: Coord, face: usize) -> usize {
        let x = (pos.0 - self.base.0) as usize;
        let y = (pos.1 - self.base.1) as usize;
        let z = (pos.2 - self.base.2) as usize;
        x + self.dim.0 * (y + self.dim.1 * (z + self.dim.2 * face))
    }
}

fn part_02(grid: &[Coord]) -> usize {
    let model: HashSet<Coord> = HashSet::from_iter(grid.iter().cloned());
    let mut faces = 0;

    let ((x_min, y_min, z_min), (x_max, y_max, z_max)) = dim(grid);

    let x_len = (x_max - x_min + 4) as usize;
    let y_len = (y_max - y_min + 4) as usize;
    let z_len = (z_max - z_min + 4) as usize;
    let id = Id {
        base: (x_min - 2, y_min - 2, z_min - 2),
        dim: (x_len, y_len, z_len),
    };

    // idea: join block faces to empty neighbours
    // faces are identified by number (0..6)
    // empty coordinates don't have faces, so use face = 0
    // then we can check for each block wether they are in the
    // same group as one outside coordinate
    let mut uf = UnionFind::new();

    for x in x_min - 1..=x_max + 1 {
        for y in y_min - 1..=y_max + 1 {
            for z in z_min - 1..=z_max + 1 {
                let pos = (x, y, z);
                if model.contains(&pos) {
                    for (face, n) in neighbours(pos).enumerate() {
                        if !model.contains(&n) {
                            uf.join(id.id(pos, face), id.id(n, 0));
                        }
                    }
                } else {
                    for n in neighbours(pos) {
                        if !model.contains(&n) {
                            uf.join(id.id(pos, 0), id.id(n, 0));
                        }
                    }
                }
            }
        }
    }
    let root_leader = uf.leader(id.id((x_min - 1, y_min - 1, z_min - 1), 0));

    for face in 0..6 {
        for pos in &model {
            if uf.leader(id.id(*pos, face)) == root_leader {
                faces += 1;
            }
        }
    }

    faces
}

fn main() {
    let grid = parse_input(INPUT);
    println!("Part 1: {}", part_01(&grid));
    println!("Part 2: {}", part_02(&grid));
}

#[cfg(test)]
mod test {
    use crate::{parse_input, part_01, part_02, INPUT};

    static TEST_INPUT: &str = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_part_1() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(64, part_01(&grid));
    }

    #[test]
    fn test_part_2() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(58, part_02(&grid));
    }

    #[test]
    fn real_part_1() {
        let grid = parse_input(INPUT);
        assert_eq!(4242, part_01(&grid));
    }

    #[test]
    fn real_part_2() {
        let grid = parse_input(INPUT);
        assert_eq!(2428, part_02(&grid));
    }
}
