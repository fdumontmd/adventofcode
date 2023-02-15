use itertools::{Itertools, MinMaxResult};
use std::collections::BTreeSet;

static INPUT: &str = include_str!("input.txt");

enum Fold {
    X(isize),
    Y(isize),
}

impl Fold {
    fn fold(&self, points: &BTreeSet<(isize, isize)>) -> BTreeSet<(isize, isize)> {
        let mut new_points = BTreeSet::new();

        for point in points {
            let point = match self {
                Fold::X(x) => {
                    if point.0 > *x {
                        (2 * *x - point.0, point.1)
                    } else {
                        *point
                    }
                }
                Fold::Y(y) => {
                    if point.1 > *y {
                        (point.0, 2 * *y - point.1)
                    } else {
                        *point
                    }
                }
            };
            new_points.insert(point);
        }

        new_points
    }
}

fn parse_input(input: &str) -> (BTreeSet<(isize, isize)>, Vec<Fold>) {
    let mut points = BTreeSet::new();
    let parts: Vec<_> = input.split("\n\n").collect();
    for line in parts[0].lines() {
        let coords: Vec<_> = line.split(',').collect();
        let x = coords[0].parse().unwrap();
        let y = coords[1].parse().unwrap();
        points.insert((x, y));
    }

    let folds = parts[1]
        .lines()
        .map(|l| {
            if let Some(x) = l.strip_prefix("fold along x=") {
                Fold::X(x.parse().unwrap())
            } else if let Some(y) = l.strip_prefix("fold along y=") {
                Fold::Y(y.parse().unwrap())
            } else {
                panic!("cannot parse folding instruction {}", l)
            }
        })
        .collect();

    (points, folds)
}

fn part_1(input: &str) -> usize {
    let (points, folds) = parse_input(input);

    folds[0].fold(&points).len()
}

fn part_2(input: &str) {
    let (mut points, folds) = parse_input(input);

    for fold in folds {
        points = fold.fold(&points);
    }

    let (xmin, xmax) = match points.iter().map(|(x, _)| *x).minmax() {
        MinMaxResult::OneElement(x) => (x, x),
        MinMaxResult::MinMax(min, max) => (min, max),
        MinMaxResult::NoElements => panic!("no points?"),
    };
    let (ymin, ymax) = match points.iter().map(|(_, y)| *y).minmax() {
        MinMaxResult::OneElement(y) => (y, y),
        MinMaxResult::MinMax(min, max) => (min, max),
        MinMaxResult::NoElements => panic!("no points?"),
    };

    println!("Part 2:");

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    part_2(INPUT);
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    static TEST_INPUT: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part_1() {
        assert_eq!(17, part_1(TEST_INPUT));
    }
}
