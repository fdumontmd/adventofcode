use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

use aoc_utils::grid::{Grid, Taxicab};

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Button {
    Up,
    Right,
    Down,
    Left,
    Press,
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.as_char();
        write!(f, "{c}")
    }
}

impl TryFrom<char> for Button {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Button::Up,
            '>' => Button::Right,
            'v' => Button::Down,
            '<' => Button::Left,
            'A' => Button::Press,
            _ => return Err(format!("'{value}' not a valid button")),
        })
    }
}

impl Button {
    fn from_movement(from: Pos, to: Pos) -> Self {
        match (to.0.cmp(&from.0), to.1.cmp(&from.1)) {
            (std::cmp::Ordering::Less, _) => Button::Left,
            (std::cmp::Ordering::Greater, _) => Button::Right,
            (_, std::cmp::Ordering::Less) => Button::Up,
            (_, std::cmp::Ordering::Greater) => Button::Down,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => Button::Press,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Button::Up => '^',
            Button::Right => '>',
            Button::Down => 'v',
            Button::Left => '<',
            Button::Press => 'A',
        }
    }
}

const NUMERIC_PAD: &str = "789
456
123
X0A";

const DIRECTIONAL_PAD: &str = "X^A
<v>";

type Pos = (usize, usize);
fn part1_one_line(grid: &Grid<char, Taxicab>, press_costs: [[usize; 5]; 5], line: &str) -> usize {
    // not good but let's get this working first
    let moves = format!("A{line}");
    let steps = moves
        .as_bytes()
        .windows(2)
        .map(|w| shortest_path(grid, press_costs, w[0] as char, w[1] as char))
        .sum::<usize>();

    if let Some(n) = line.strip_suffix("A") {
        let n: usize = n.parse().unwrap();
        steps * n
    } else {
        panic!("wrong input {line}")
    }
}

fn part1(input: &str) -> usize {
    let grid: Grid<char, Taxicab> = Grid::try_from(DIRECTIONAL_PAD).unwrap();
    let press_costs = build_press_costs(&grid, 2);
    let grid: Grid<char, Taxicab> = Grid::try_from(NUMERIC_PAD).unwrap();
    input
        .lines()
        .map(|line| part1_one_line(&grid, press_costs, line))
        .sum()
}

// what's the cost of pressing a button on the directional pad with `level` levels of indirection
// (level 0 == me)
fn build_press_costs(grid: &Grid<char, Taxicab>, level: i32) -> [[usize; 5]; 5] {
    use Button::*;
    if level == 0 {
        // uniform of 1 cost for user
        [[1; 5]; 5]
    } else {
        let mut current_press_costs = [[0; 5]; 5];
        let previous_press_costs = build_press_costs(grid, level - 1);

        for from in [Up, Right, Down, Left, Press] {
            // seen does not guarantee that each button will be evaluated just once; just that
            // we'll stop evaluating them quickly
            let mut seen = [false; 5];
            let from_pos = grid.idx_to_pos(grid.iter().position(|d| *d == from.as_char()).unwrap());
            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0), from_pos, Press));
            while let Some((Reverse(cost), pos, button)) = queue.pop() {
                let cd = Button::try_from(grid[pos]).unwrap();
                if cost > 0 && button == Press {
                    // first cost will be optimal, so if we have one don't update
                    if current_press_costs[from as usize][cd as usize] == 0 {
                        current_press_costs[from as usize][cd as usize] = cost;
                    }
                } else {
                    queue.push((
                        Reverse(cost + previous_press_costs[button as usize][Press as usize]),
                        pos,
                        Press,
                    ));
                }
                seen[cd as usize] = true;
                for n in grid.neighbours(pos) {
                    if grid[n] == 'X' {
                        continue;
                    }
                    let d = Button::try_from(grid[n]).unwrap();
                    if seen[d as usize] {
                        continue;
                    }
                    let next_button = Button::from_movement(pos, n);
                    queue.push((
                        Reverse(cost + previous_press_costs[button as usize][next_button as usize]),
                        n,
                        next_button,
                    ));
                }
            }
        }
        current_press_costs
    }
}

// compute shortest path between two numerical buttons taking into account
// the cost of pressing the directional buttons
fn shortest_path(
    grid: &Grid<char, Taxicab>,
    press_costs: [[usize; 5]; 5],
    from_n: char,
    to_n: char,
) -> usize {
    let from = grid.idx_to_pos(grid.iter().position(|c| *c == from_n).unwrap());
    let to = grid.idx_to_pos(grid.iter().position(|c| *c == to_n).unwrap());

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), from, Button::Press));

    while let Some((Reverse(cost), pos, button)) = queue.pop() {
        if pos == to {
            if button == Button::Press {
                return cost;
            } else {
                queue.push((
                    Reverse(cost + press_costs[button as usize][Button::Press as usize]),
                    pos,
                    Button::Press,
                ));
            }
        } else {
            for n in grid.neighbours(pos) {
                if grid[n] == 'X' {
                    continue;
                }
                let nd = Button::from_movement(pos, n);
                queue.push((
                    Reverse(cost + press_costs[button as usize][nd as usize]),
                    n,
                    nd,
                ));
            }
        }
    }

    unreachable!()
}

fn part2(input: &str) -> usize {
    let grid: Grid<char, Taxicab> = Grid::try_from(DIRECTIONAL_PAD).unwrap();
    let press_costs = build_press_costs(&grid, 25);
    let grid: Grid<char, Taxicab> = Grid::try_from(NUMERIC_PAD).unwrap();
    input
        .lines()
        .map(|line| part1_one_line(&grid, press_costs, line))
        .sum()
}
fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "029A
980A
179A
456A
379A";

    #[test_case(TEST_INPUT, 126384; "test input")]
    #[test_case(INPUT, 157892; "input")]
    fn test_part1(input: &str, complexity: usize) {
        assert_eq!(complexity, part1(input));
    }

    #[test_case(INPUT, 197015606336332; "input")]
    fn test_part2(input: &str, complexity: usize) {
        assert_eq!(complexity, part2(input));
    }
}
