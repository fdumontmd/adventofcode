use anyhow::*;
use intcode::*;
use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        use Direction::*;
        match *self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }

    fn turn_left(&mut self) {
        use Direction::*;
        *self = match *self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    fn turn_right(&mut self) {
        use Direction::*;
        *self = match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

struct Robot {
    computer: Computer,
    position: (isize, isize),
    direction: Direction,
    panel: HashSet<(isize, isize)>,
    painted_panel: HashSet<(isize, isize)>,
}

impl Robot {
    fn new(code: &str, init_white: bool) -> Result<Self> {
        let mut robot = Robot {
            computer: code.parse()?,
            position: (0, 0),
            direction: Direction::Up,
            panel: HashSet::new(),
            painted_panel: HashSet::new(),
        };
        if init_white {
            robot.panel.insert(robot.position);
        }
        Ok(robot)
    }

    fn step(&mut self) {
        let input = if self.panel.contains(&self.position) {
            1
        } else {
            0
        };
        self.computer.add_input(input);

        if let Some(out) = self.computer.wait_until_output() {
            if out == 1 {
                self.panel.insert(self.position);
            } else {
                self.panel.remove(&self.position);
            }
            self.painted_panel.insert(self.position);
        }

        if let Some(out) = self.computer.wait_until_output() {
            if out == 1 {
                self.direction.turn_right();
            } else {
                self.direction.turn_left();
            }
        }
        let delta = self.direction.delta();
        self.position.0 += delta.0;
        self.position.1 += delta.1;
    }

    fn is_stopped(&self) -> bool {
        self.computer.is_stopped()
    }

    fn painted_panel_count(&self) -> usize {
        self.painted_panel.len()
    }
}

fn part_1() -> Result<usize> {
    let mut robot = Robot::new(INPUT, false)?;
    while !robot.is_stopped() {
        robot.step();
    }

    Ok(robot.painted_panel_count())
}

fn part_2() -> Result<()> {
    println!("part 2");
    let mut robot = Robot::new(INPUT, true)?;
    while !robot.is_stopped() {
        robot.step();
    }

    let lx = robot.panel.iter().map(|c| c.0).min().unwrap();
    let ly = robot.panel.iter().map(|c| c.1).min().unwrap();

    let hx = robot.panel.iter().map(|c| c.0).max().unwrap();
    let hy = robot.panel.iter().map(|c| c.1).max().unwrap();

    for row in ly..=hy {
        for col in lx..=hx {
            if robot.panel.contains(&(col, row)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("part 1: {}", part_1()?);
    part_2()
}
