use std::cmp::Reverse;

use aoc_utils::grid::{Grid, Taxicab};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn forward(&self, pos: (usize, usize), grid: &Grid<u8, Taxicab>) -> Option<(usize, usize)> {
        let delta = match self {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };

        let pos = pos
            .0
            .checked_add_signed(delta.0)
            .and_then(|x| pos.1.checked_add_signed(delta.1).map(|y| (x, y)));

        pos.filter(|&(x, y)| x < grid.width() && y < grid.height())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Step {
    Forward,
    TurnLeft,
    TurnRight,
}

impl Step {
    fn direction(&self, direction: Direction) -> Direction {
        match self {
            Step::Forward => direction,
            Step::TurnLeft => match direction {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
            },
            Step::TurnRight => match direction {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct State {
    pub pos: (usize, usize),
    pub direction: Direction,
    pub straight: u8,
    pub heat_loss: usize,
}

impl State {
    pub fn new(dir: Direction) -> Self {
        State {
            pos: (0, 0),
            direction: dir,
            straight: 1,
            heat_loss: 0,
        }
    }

    pub fn successors(&self, grid: &Grid<u8, Taxicab>) -> Vec<Self> {
        let mut succs = Vec::new();

        if self.straight < 3 {
            if let Some(pos) = self.direction.forward(self.pos, grid) {
                succs.push(State {
                    pos,
                    direction: self.direction,
                    straight: self.straight + 1,
                    heat_loss: self.heat_loss + grid[pos] as usize,
                });
            }
        }

        let turn_left = Step::TurnLeft.direction(self.direction);
        if let Some(pos) = turn_left.forward(self.pos, grid) {
            succs.push(State {
                pos,
                direction: turn_left,
                straight: 1,
                heat_loss: self.heat_loss + grid[pos] as usize,
            });
        }

        let turn_right = Step::TurnRight.direction(self.direction);
        if let Some(pos) = turn_right.forward(self.pos, grid) {
            succs.push(State {
                pos,
                direction: turn_right,
                straight: 1,
                heat_loss: self.heat_loss + grid[pos] as usize,
            });
        }

        succs
    }

    pub fn ultra_successors(&self, grid: &Grid<u8, Taxicab>) -> Vec<Self> {
        let mut succs = Vec::new();

        if self.straight < 10 {
            if let Some(pos) = self.direction.forward(self.pos, grid) {
                succs.push(State {
                    pos,
                    direction: self.direction,
                    straight: self.straight + 1,
                    heat_loss: self.heat_loss + grid[pos] as usize,
                });
            }
        }

        if self.straight > 3 {
            let turn_left = Step::TurnLeft.direction(self.direction);
            if let Some(pos) = turn_left.forward(self.pos, grid) {
                succs.push(State {
                    pos,
                    direction: turn_left,
                    straight: 1,
                    heat_loss: self.heat_loss + grid[pos] as usize,
                });
            }

            let turn_right = Step::TurnRight.direction(self.direction);
            if let Some(pos) = turn_right.forward(self.pos, grid) {
                succs.push(State {
                    pos,
                    direction: turn_right,
                    straight: 1,
                    heat_loss: self.heat_loss + grid[pos] as usize,
                });
            }
        }

        succs
    }

    pub fn get_key(self, grid: &Grid<u8, Taxicab>) -> ((Reverse<usize>, usize), Self) {
        ((Reverse(self.heat_loss), grid.pos_to_idx(self.pos)), self)
    }
}
