use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");
const INPUT_SIZE: (i64, i64) = (101, 103);

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct Robot {
    pos: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    fn from_str(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let pos = if let Some(pos) = parts[0].strip_prefix("p=") {
            let pos: Vec<i64> = pos.split(',').map(|c| c.parse().unwrap()).collect();
            (pos[0], pos[1])
        } else {
            panic!("cannot parse {line}")
        };
        let velocity = if let Some(vel) = parts[1].strip_prefix("v=") {
            let vel: Vec<i64> = vel.split(',').map(|c| c.parse().unwrap()).collect();
            (vel[0], vel[1])
        } else {
            panic!("cannot parse {line}")
        };

        Robot { pos, velocity }
    }

    fn position_after(&self, time: i64, size: (i64, i64)) -> (i64, i64) {
        (
            (self.pos.0 + self.velocity.0 * time).rem_euclid(size.0),
            (self.pos.1 + self.velocity.1 * time).rem_euclid(size.1),
        )
    }

    fn quadrant_after(&self, time: i64, size: (i64, i64)) -> Option<Quadrant> {
        let pos = self.position_after(time, size);

        let left = 0..size.0 / 2;
        let right = size.0 / 2 + 1..size.0;
        let top = 0..size.1 / 2;
        let bottom = size.1 / 2 + 1..size.1;

        match (
            left.contains(&pos.0),
            right.contains(&pos.0),
            top.contains(&pos.1),
            bottom.contains(&pos.1),
        ) {
            (true, false, true, false) => Some(Quadrant::TopLeft),
            (true, false, false, true) => Some(Quadrant::BottomLeft),
            (false, true, true, false) => Some(Quadrant::TopRight),
            (false, true, false, true) => Some(Quadrant::BottomRight),
            _ => None,
        }
    }

    fn step(&mut self, size: (i64, i64)) {
        self.pos = self.position_after(1, size);
    }
}

fn safety(robots: &Vec<Robot>, time: i64, size: (i64, i64)) -> usize {
    let mut robot_quadrants: HashMap<Quadrant, usize> = HashMap::new();
    robots
        .iter()
        .filter_map(|r| r.quadrant_after(time, size))
        .for_each(|q| *robot_quadrants.entry(q).or_insert(0) += 1);
    robot_quadrants.values().product()
}

fn part1(input: &str, size: (i64, i64)) -> usize {
    let robots: Vec<Robot> = input.lines().map(Robot::from_str).collect();
    safety(&robots, 100, size)
}

fn display(size: (i64, i64), robots: &Vec<Robot>) {
    let mut grid = vec![vec![0; size.0 as usize]; size.1 as usize];
    for r in robots {
        grid[r.pos.1 as usize][r.pos.0 as usize] += 1;
    }

    for row in grid {
        for col in row {
            if col == 0 {
                print!(".");
            } else if col < 10 {
                print!("{col}");
            } else {
                // many
                print!("*");
            }
        }
        println!();
    }
}

// after visiting reddit, found out that unique positions were
// enough.
// looking at the display, not clear that anything else would
// work... many robots are not involved in the drawing of the
// tree, so concentration of robots positions would not work
fn part2(input: &str, size: (i64, i64)) -> usize {
    let mut robots: Vec<Robot> = input.lines().map(Robot::from_str).collect();

    'step: for steps in 1.. {
        let mut unique_pos = true;
        robots.iter_mut().for_each(|r| r.step(size));

        let mut positions = HashSet::new();
        for r in &robots {
            if positions.contains(&r.pos) {
                continue 'step;
            }
            positions.insert(r.pos);
        }
        // confirmed from display that first time when positions are unique,
        // there's a chrismas tree
        //println!("{steps}");
        //display(size, &robots);
        return steps;
    }
    unreachable!()
}

fn main() {
    println!("part 1: {}", part1(INPUT, INPUT_SIZE));
    println!("part 2: {}", part2(INPUT, INPUT_SIZE));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    const TEST_INPUT_SIZE: (i64, i64) = (11, 7);

    #[test_case(TEST_INPUT, TEST_INPUT_SIZE, 12; "test input")]
    #[test_case(INPUT, INPUT_SIZE, 230461440; "input")]
    fn test_part1(input: &str, size: (i64, i64), safety: usize) {
        assert_eq!(safety, part1(input, size));
    }

    #[test_case(INPUT, INPUT_SIZE, 6668; "input")]
    fn test_part2(input: &str, size: (i64, i64), steps: usize) {
        assert_eq!(steps, part2(input, size));
    }
}
