use aoc_utils::grid::{Grid, Taxicab};

const INPUT: &str = include_str!("input.txt");

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct Packet<'a> {
    grid: &'a Grid<char, Taxicab>,
    pos: (usize, usize),
    direction: Direction,
}

impl<'a> Packet<'a> {
    fn new(grid: &'a Grid<char, Taxicab>) -> Self {
        let idx = grid
            .iter()
            .enumerate()
            .find_map(|(idx, c)| if *c == '|' { Some(idx) } else { None })
            .unwrap();
        let pos = grid.idx_to_pos(idx);
        Self {
            grid,
            pos,
            direction: Direction::Down,
        }
    }
}

impl Iterator for Packet<'_> {
    type Item = Option<char>;

    fn next(&mut self) -> Option<Self::Item> {
        // compute next position based on direction
        // if next pos is out of grid or blank, return
        // None
        //
        // check what's under next position:
        // if | or -, or letter, keep going
        // if +, check for turn: compute
        // neighbours except previous pos, and
        // adjust direction
        //
        // if current pos is letter, return Just(Just(l))
        // else return Just(None)

        let delta = self.direction.delta();
        let next_pos = self
            .pos
            .0
            .checked_add_signed(delta.0)
            .and_then(|x| self.pos.1.checked_add_signed(delta.1).map(|y| (x, y)));

        match next_pos {
            None => None,
            Some(np) => {
                if np.0 >= self.grid.width()
                    || np.1 >= self.grid.height()
                    || self.grid[np].is_whitespace()
                {
                    None
                } else {
                    let v = if self.grid[np].is_alphabetic() {
                        Some(self.grid[np])
                    } else {
                        None
                    };

                    if self.grid[np] == '+' {
                        // check the neighbours of np, minus self.pos
                        // the np -> remaining neighbour is new direction
                        let n: Vec<_> = self
                            .grid
                            .neighbours(np)
                            .filter(|&p| p != self.pos && !self.grid[p].is_whitespace())
                            .collect();
                        assert_eq!(1, n.len());
                        let n = n[0];

                        self.direction = if n.0 == np.0 {
                            // vertical change
                            if n.1 > np.1 {
                                Direction::Down
                            } else {
                                Direction::Up
                            }
                        } else {
                            // horizontal change
                            if n.0 > np.0 {
                                Direction::Right
                            } else {
                                Direction::Left
                            }
                        };
                    }

                    self.pos = np;
                    Some(v)
                }
            }
        }
    }
}

fn part1(input: &str) -> String {
    let grid: Grid<char, Taxicab> = Grid::try_from(input).unwrap();

    let packet = Packet::new(&grid);
    let mut buf = String::new();

    packet.for_each(|l| {
        if let Some(l) = l {
            buf.push(l)
        }
    });
    buf
}

fn part2(input: &str) -> usize {
    let grid: Grid<char, Taxicab> = Grid::try_from(input).unwrap();

    let packet = Packet::new(&grid);
    packet.into_iter().count() + 1
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "     |          
     |  +--+    
     A  |  C    
 F---|--|-E---+ 
     |  |  |  D 
     +B-+  +--+ 
";

    #[test_case(TEST_INPUT, "ABCDEF")]
    #[test_case(INPUT, "BPDKCZWHGT")]
    fn test_part1(input: &str, output: &str) {
        assert_eq!(output, &part1(input));
    }

    #[test_case(TEST_INPUT, 38)]
    #[test_case(INPUT, 17728)]
    fn test_part2(input: &str, steps: usize) {
        assert_eq!(steps, part2(input));
    }
}
