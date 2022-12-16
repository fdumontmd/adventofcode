use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::Index,
};

static INPUT: &str = include_str!("input.txt");

struct Map {
    heights: Vec<u8>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .filter_map(move |delta| {
                pos.0
                    .checked_add_signed(delta.0)
                    .and_then(|x| pos.1.checked_add_signed(delta.1).map(|y| (x, y)))
            })
            .filter(move |p| p.0 < self.width && p.1 < self.height)
    }

    fn reachable_iter(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let h = Map::height(*self.index(pos));
        self.neighbours(pos).filter(move |&p| {
            let target = Map::height(*self.index(p));
            target <= h + 1
        })
    }

    fn height(h: u8) -> u8 {
        match h {
            b'E' => b'z',
            b'S' => b'a',
            _ => h,
        }
    }

    fn rev_reachable_iter(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let h = Map::height(*self.index(pos));
        self.neighbours(pos).filter(move |&p| {
            let target = Map::height(*self.index(p));
            h <= target + 1
        })
    }
}

impl TryFrom<&str> for Map {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value
            .lines()
            .filter_map(|l| {
                let l = l.trim();
                if l.is_empty() {
                    None
                } else {
                    Some(l)
                }
            })
            .collect();
        let height = lines.len();
        let width = if height > 0 { lines[0].len() } else { 0 };
        let heights: Vec<_> = lines.into_iter().flat_map(|l| l.bytes()).collect();
        let start = if let Some(start) = heights.iter().position(|&h| h == b'S') {
            (start % width, start / width)
        } else {
            return Err("Could not find the starting point".into());
        };
        let end = if let Some(end) = heights.iter().position(|&h| h == b'E') {
            (end % width, end / width)
        } else {
            return Err("Could not find the end point".into());
        };

        Ok(Map {
            heights,
            width,
            height,
            start,
            end,
        })
    }
}

fn part_01(input: &str) -> Result<usize, String> {
    let mut seen = HashSet::new();
    let mut paths = BinaryHeap::new();

    let map = Map::try_from(input)?;

    seen.insert(map.start);
    for pos in map.reachable_iter(map.start) {
        paths.push((Reverse(1), pos));
        seen.insert(pos);
    }

    while let Some((dist, pos)) = paths.pop() {
        if pos == map.end {
            return Ok(dist.0);
        } else {
            for next in map.reachable_iter(pos) {
                if !seen.contains(&next) {
                    seen.insert(next);
                    paths.push((Reverse(dist.0 + 1), next));
                }
            }
        }
    }

    Err("No path found to end point".into())
}

fn part_02(input: &str) -> Result<usize, String> {
    let mut seen = HashSet::new();
    let mut paths = BinaryHeap::new();

    let map = Map::try_from(input)?;

    seen.insert(map.end);
    for pos in map.rev_reachable_iter(map.end) {
        paths.push((Reverse(1), pos));
        seen.insert(pos);
    }

    while let Some((dist, pos)) = paths.pop() {
        if map[pos] == b'a' || map[pos] == b'S' {
            return Ok(dist.0);
        } else {
            for next in map.rev_reachable_iter(pos) {
                if !seen.contains(&next) {
                    seen.insert(next);
                    paths.push((Reverse(dist.0 + 1), next));
                }
            }
        }
    }

    Err("No path found to end point".into())
}

impl Index<(usize, usize)> for Map {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.heights[index.1 * self.width + index.0]
    }
}

fn main() -> Result<(), String> {
    println!("Part 1: {}", part_01(INPUT)?);
    println!("Part 2: {}", part_02(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02, INPUT};

    static TEST_INPUT: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    #[test]
    fn test_part_1() {
        assert_eq!(Ok(31), part_01(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Ok(29), part_02(TEST_INPUT));
    }

    #[test]
    fn real_part_1() {
        assert_eq!(Ok(412), part_01(INPUT));
    }

    #[test]
    fn real_part_2() {
        assert_eq!(Ok(402), part_02(INPUT));
    }
}
