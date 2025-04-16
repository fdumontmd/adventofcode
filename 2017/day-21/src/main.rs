use std::{collections::HashMap, fmt::Display, iter::zip};

const INIT: &str = ".#.
..#
###
";

struct Grid(Vec<Vec<u8>>);

impl Default for Grid {
    fn default() -> Self {
        Self(INIT.lines().map(|l| l.as_bytes().to_vec()).collect())
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.0 {
            for b in l {
                write!(f, "{}", *b as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|l| l.iter().filter(|b| **b == b'#').count())
            .sum()
    }

    fn step(&mut self, patterns: &Patterns) {
        if self.size() % 2 == 0 {
            let mut new_grid: Vec<Vec<u8>> = vec![vec![]; self.size() / 2 * 3];

            for (idx, lines) in self.0.chunks(2).enumerate() {
                for (top, bot) in zip(lines[0].chunks(2), lines[1].chunks(2)) {
                    let pat = [top, bot].concat().to_vec();
                    if let Some(t) = patterns.0.get(&pat) {
                        for (offset, l) in t.chunks(3).enumerate() {
                            new_grid[3 * idx + offset].extend_from_slice(l);
                        }
                    } else {
                        panic!("pattern not found: {pat:?}");
                    }
                }
            }

            self.0 = new_grid;
        } else {
            let mut new_grid: Vec<Vec<u8>> = vec![vec![]; self.size() / 3 * 4];

            for (idx, lines) in self.0.chunks(3).enumerate() {
                for (top, (mid, bot)) in zip(
                    lines[0].chunks(3),
                    zip(lines[1].chunks(3), lines[2].chunks(3)),
                ) {
                    let pat = [top, mid, bot].concat().to_vec();
                    if let Some(t) = patterns.0.get(&pat) {
                        for (offset, l) in t.chunks(4).enumerate() {
                            new_grid[4 * idx + offset].extend_from_slice(l);
                        }
                    } else {
                        panic!("pattern not found: {pat:?}");
                    }
                }
            }

            self.0 = new_grid;
        }
    }
}

struct Patterns(HashMap<Vec<u8>, Vec<u8>>);

fn mirror(mut input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // consume input
    input.iter_mut().for_each(|v| v.reverse());
    input
}

fn build_rotations(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut rotations = vec![];

    let l = input.len();

    let mut base = vec![];
    for r in input {
        for c in r {
            base.push(*c);
        }
    }
    rotations.push(base);

    let mut base = vec![];
    for c in (0..l).rev() {
        for r in input {
            base.push(r[c]);
        }
    }
    rotations.push(base);

    let mut base = vec![];
    for r in (0..l).rev() {
        for c in (0..l).rev() {
            base.push(input[r][c]);
        }
    }
    rotations.push(base);

    let mut base = vec![];
    for c in 0..l {
        for r in (0..l).rev() {
            base.push(input[r][c]);
        }
    }
    rotations.push(base);

    rotations
}

fn build_patterns(input: Vec<u8>) -> Vec<Vec<u8>> {
    let mut variants = vec![];

    let input: Vec<Vec<u8>> = input.split(|b| *b == b'/').map(|s| s.to_vec()).collect();

    variants.append(&mut build_rotations(&input));
    let input = mirror(input);
    variants.append(&mut build_rotations(&input));

    variants
}

impl Patterns {
    fn new(input: &str) -> Self {
        let mut targets: Vec<Vec<u8>> = vec![];
        let mut base_patterns = vec![];
        for l in input.lines() {
            let parts: Vec<_> = l.split(" => ").collect();
            base_patterns.push(parts[0].as_bytes().to_vec());
            targets.push(parts[1].bytes().filter(|b| *b != b'/').collect());
        }

        let mut patterns = HashMap::new();

        for (idx, base) in base_patterns.into_iter().enumerate() {
            for pattern in build_patterns(base) {
                patterns.insert(pattern, targets[idx].clone());
            }
        }

        Self(patterns)
    }
}

const INPUT: &str = include_str!("input.txt");

fn count_after(input: &str, steps: usize) -> usize {
    let patterns = Patterns::new(input);
    let mut grid = Grid::default();

    for _ in 0..steps {
        grid.step(&patterns);
    }

    grid.count()
}

fn part1(input: &str) -> usize {
    count_after(input, 5)
}

fn part2(input: &str) -> usize {
    count_after(input, 18)
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
";

    #[test_case(TEST_INPUT, 2, 12)]
    #[test_case(INPUT, 5, 188)]
    #[test_case(INPUT, 18, 2758764)]
    fn test(input: &str, steps: usize, count: usize) {
        assert_eq!(count, count_after(input, steps));
    }
}
