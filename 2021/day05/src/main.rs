use std::collections::HashMap;
use std::cmp::{min, max};


const INPUT: &str = include_str!("input");

fn parse_point(input: &str) -> (isize, isize) {
    let points: Vec<isize> = input.split(",").map(|n| n.parse().unwrap()).collect();
    (points[0], points[1])
}

fn part01(input: &str) -> usize {
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();

    for line in input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();
        let from = parse_point(points[0]);
        let to = parse_point(points[1]);

        if from.0 == to.0 {
            for y in min(from.1, to.1)..=max(from.1, to.1) {
                *map.entry((from.0, y)).or_default() += 1;
            }
        } else if from.1 == to.1 {
            for x in min(from.0, to.0)..=max(from.0, to.0) {
                *map.entry((x, from.1)).or_default() += 1;
            }

        }
    }

    map.values().filter(|&&v| v > 1).count()
}

fn part02(input: &str) -> usize {
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();

    for line in input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();
        let from = parse_point(points[0]);
        let to = parse_point(points[1]);

        if from.0 == to.0 {
            for y in min(from.1, to.1)..=max(from.1, to.1) {
                *map.entry((from.0, y)).or_default() += 1;
            }
        } else if from.1 == to.1 {
            for x in min(from.0, to.0)..=max(from.0, to.0) {
                *map.entry((x, from.1)).or_default() += 1;
            }
        } else if (from.0 - to.0).abs() == (from.1 - to.1).abs() {
            let dx = (to.0 - from.0).signum();
            let dy = (to.1 - from.1).signum();

            let mut cx = from.0;
            let mut cy = from.1;

            for _ in 0..=(to.0 - from.0).abs() {
                *map.entry((cx, cy)).or_default() += 1;
                cx += dx;
                cy += dy;
            }

        } else {
            eprintln!("{}", line);
            unreachable!();
        }
    }

    map.values().filter(|&&v| v > 1).count()
}

fn main() {
    println!("part 01: {}", part01(INPUT));
    println!("part 02: {}", part02(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part01() {
        assert_eq!(5, part01(TEST));
    }

    #[test]
    fn test_part02() {
        assert_eq!(12, part02(TEST));
    }
}
