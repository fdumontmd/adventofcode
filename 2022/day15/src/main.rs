use std::mem;

// TODO:
// rename Sensor; stupid name
// use Sensor::range instead of sensor.sensor.dist(&sensor.beacon)

use lazy_static::lazy_static;
use regex::Regex;

static INPUT: &str = include_str!("input.txt");
lazy_static! {
    static ref SENSOR_PARSER: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord(i32, i32);

impl Coord {
    fn dist(&self, other: &Self) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn ball_boundary_at(&self, dist: u32, row: i32) -> (i32, i32) {
        let dist = dist as i32;
        let y = self.1.abs_diff(row) as i32;

        if y > dist {
            return (0, 0);
        }

        let width = dist - y;
        (self.0 - width, self.0 + width + 1)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Sensor {
    sensor: Coord,
    beacon: Coord,
}

impl Sensor {
    fn parse(input: &str) -> Self {
        let captures = SENSOR_PARSER.captures(input).unwrap();
        Sensor {
            sensor: Coord(
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
            ),
            beacon: Coord(
                captures.get(3).unwrap().as_str().parse().unwrap(),
                captures.get(4).unwrap().as_str().parse().unwrap(),
            ),
        }
    }

    fn range(&self) -> u32 {
        self.sensor.dist(&self.beacon)
    }
}

#[derive(Debug)]
struct Line(Vec<(i32, i32)>);

impl Line {
    fn new(limit: i32) -> Self {
        Line(vec![(0, limit + 1)])
    }

    fn remove(&mut self, from: i32, to: i32) {
        let orig = mem::take(&mut self.0);
        self.0 = orig
            .into_iter()
            .filter_map(|(b, e)| {
                if from <= b && e <= to {
                    None
                } else if to < b || e < from {
                    Some(vec![(b, e)])
                } else if from <= b && to >= b && to < e {
                    Some(vec![(to, e)])
                } else if b < from && from <= e && e <= to {
                    Some(vec![(b, from)])
                } else {
                    Some(vec![(b, from), (to, e)])
                }
            })
            .flatten()
            .collect()
    }

    fn len(&self) -> usize {
        self.0.iter().map(|(b, e)| (e - b) as usize).sum()
    }
}

fn part_01(input: &str, row: i32) -> usize {
    let sensors: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Sensor::parse)
        .collect();

    let (from, to) = sensors
        .iter()
        .map(|s| {
            let dist = s.range();
            s.sensor.ball_boundary_at(dist, row)
        })
        .fold((i32::MAX, i32::MIN), |s, o| (s.0.min(o.0), s.1.max(o.1)));

    let mut line = Line::new(to);

    for sensor in sensors {
        let (from, to) = sensor.sensor.ball_boundary_at(sensor.range(), row);
        if from < to {
            line.remove(from, to);
        }
    }

    (to - line.len() as i32 - from) as usize
}

fn part_02(input: &str, limit: i32) -> u64 {
    let sensors: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Sensor::parse)
        .collect();
    for row in 0..=limit {
        let mut line = Line::new(limit);
        for sensor in &sensors {
            let (from, to) = sensor.sensor.ball_boundary_at(sensor.range(), row);
            if from < to {
                line.remove(from, to);
            }
        }
        if line.len() == 1 {
            let (b, _) = line.0[0];
            return b as u64 * 4000000 + row as u64;
        }
    }
    0
}

fn main() {
    println!("Part 1: {}", part_01(INPUT, 2000000));
    println!("Part 1: {}", part_02(INPUT, 4000000));
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02, INPUT};

    static TEST_INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part_01() {
        assert_eq!(26, part_01(TEST_INPUT, 10));
    }

    #[test]
    fn test_part_02() {
        assert_eq!(56000011, part_02(TEST_INPUT, 20));
    }

    #[test]
    fn real_part_01() {
        assert_eq!(4985193, part_01(INPUT, 2000000));
    }

    #[test]
    fn real_part_02() {
        assert_eq!(11583882601918, part_02(INPUT, 4000000));
    }
}
