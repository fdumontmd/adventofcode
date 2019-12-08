use std::convert::TryInto;
use std::fmt::Display;

static INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&mut self) {
        use Turn::*;
        match *self {
            Left => *self = Straight,
            Straight => *self = Right,
            Right => *self = Left,
        }
    }
}

impl Direction {
    fn new(b: u8) -> Self {
        use Direction::*;
        match b {
            b'^' => Up,
            b'>' => Right,
            b'v' => Down,
            b'<' => Left,
            _ => unreachable!(),
        }
    }

    fn delta(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }

    fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Straight => {}
            Turn::Left => {
                *self = match *self {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                };
            }
            Turn::Right => {
                *self = match *self {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                };
            }
        }
    }

    fn orient(&mut self, road: u8) {
        use Direction::*;
        match (&self, road) {
            (Up, b'|') => {}
            (Right, b'-') => {}
            (Down, b'|') => {}
            (Left, b'-') => {}
            (_, b'+') => {
                panic!("crossing should be handled by owning struct");
            }
            (Up, b'/') => {
                *self = Right;
            }
            (Up, b'\\') => {
                *self = Left;
            }
            (Right, b'/') => {
                *self = Up;
            }
            (Right, b'\\') => {
                *self = Down;
            }
            (Down, b'/') => {
                *self = Left;
            }
            (Down, b'\\') => {
                *self = Right;
            }
            (Left, b'/') => {
                *self = Down;
            }
            (Left, b'\\') => {
                *self = Up;
            }
            _ => panic!(format!("invalid road {} for direction {:?}", road, self)),
        }
    }

    fn get_byte(&self) -> u8 {
        use Direction::*;
        match *self {
            Up => b'^',
            Right => b'>',
            Down => b'v',
            Left => b'<',
        }
    }

    fn get_default_road(&self) -> u8 {
        use Direction::*;
        match *self {
            Up | Down => b'|',
            Right | Left => b'-',
        }
    }
}

struct Cart {
    position: (usize, usize),
    direction: Direction,
    turn: Turn,
    crashed: bool,
}

impl Cart {
    fn new(position: (usize, usize), cart_byte: u8) -> Self {
        Cart {
            position,
            direction: Direction::new(cart_byte),
            turn: Turn::Left,
            crashed: false,
        }
    }

    fn get_default_road(&self) -> u8 {
        self.direction.get_default_road()
    }

    fn move_to(&mut self, position: (usize, usize), road: u8) {
        match road {
            b'+' => {
                let turn = &self.turn;
                self.direction.turn(*turn);
                self.turn.next();
            }
            _ => self.direction.orient(road),
        }
        self.position = position;
    }

    fn new_pos(&self) -> (usize, usize) {
        let delta = self.direction.delta();
        (
            (self.position.0 as isize + delta.0).try_into().unwrap(),
            (self.position.1 as isize + delta.1).try_into().unwrap(),
        )
    }

    fn get_byte(&self) -> u8 {
        self.direction.get_byte()
    }

    fn crash(&mut self) {
        self.crashed = true;
    }

    fn is_crashed(&self) -> bool {
        self.crashed
    }
}

static CART_BYTES: &[u8] = b"^>v<";

struct Map {
    lines: Vec<Vec<u8>>,
    carts: Vec<Cart>,
    tick_work: Vec<usize>,
}

impl Map {
    fn new(desc: &str) -> Self {
        let mut lines: Vec<Vec<u8>> = desc.lines().map(|l| l.bytes().collect()).collect();
        let mut carts: Vec<Cart> = Vec::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, b) in line.iter().enumerate() {
                match b {
                    b'^' | b'v' | b'<' | b'>' => carts.push(Cart::new((col, row), *b)),
                    _ => {}
                }
            }
        }

        for c in carts.iter() {
            lines[c.position.1][c.position.0] = c.get_default_road()
        }

        Map {
            lines,
            carts,
            tick_work: Vec::new(),
        }
    }

    fn load_tick_work(&mut self) {
        let mut positions: Vec<_> = self
            .carts
            .iter()
            .enumerate()
            .map(|(idx, c)| (c.position, idx))
            .collect();
        positions.sort_by(|a, b| (b.0).1.cmp(&(a.0).1).then((b.0).0.cmp(&(a.0).0)));
        self.tick_work = positions.into_iter().map(|c| c.1).collect();
    }

    fn get_with_cart(&self, position: (usize, usize)) -> u8 {
        if let Some(c) = self.carts.iter().find(|c| c.position == position) {
            if !c.is_crashed() {
                return c.get_byte();
            }
        }
        self.lines[position.1][position.0]
    }

    fn tick_once(&mut self) -> std::result::Result<(), (usize, usize)> {
        if self.tick_work.is_empty() {
            self.load_tick_work();
        }

        if let Some(idx) = self.tick_work.pop() {
            let cart = &self.carts[idx];
            if !cart.is_crashed() {
                let new_pos = cart.new_pos();

                let b = self.get_with_cart(new_pos);

                if CART_BYTES.iter().any(|&cb| cb == b) {
                    self.carts.get_mut(idx).map(|c| c.crash());
                    self.carts
                        .iter_mut()
                        .find(|c| c.position == new_pos)
                        .map(|c| c.crash());
                    return Err(new_pos);
                }

                self.carts.get_mut(idx).map(|c| c.move_to(new_pos, b));
            }
        } else {
            unreachable!();
        }

        Ok(())
    }

    fn count_running(&self) -> usize {
        self.carts.iter().filter(|c| !c.is_crashed()).count()
    }

    fn tick(&mut self) -> Option<(usize, usize)> {
        let _ = self.tick_once();

        if self.tick_work.is_empty() {
            if self.count_running() == 1 {
                return self
                    .carts
                    .iter()
                    .filter(|c| !c.is_crashed())
                    .map(|c| c.position)
                    .last();
            } else if self.count_running() == 0 {
                panic!("all carts crashed");
            }
        }

        None
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.lines.len() {
            for col in 0..self.lines[row].len() {
                write!(f, "{}", self.get_with_cart((col, row)) as char)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn run_to_crash(desc: &str) -> (usize, usize) {
    let mut map = Map::new(desc);
    loop {
        match map.tick_once() {
            Ok(_) => {}
            Err(pos) => {
                return pos;
            }
        }
    }
}

fn run_to_one(desc: &str) -> (usize, usize) {
    let mut map = Map::new(desc);

    loop {
        if let Some(pos) = map.tick() {
            return pos;
        }
    }
}

fn part_1() -> (usize, usize) {
    run_to_crash(INPUT)
}

fn part_2() -> (usize, usize) {
    run_to_one(INPUT)
}

fn main() {
    println!("part 1: {:?}", part_1());
    println!("part 2: {:?}", part_2());
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST: &str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;

    #[test]
    fn test_crash_pos() {
        assert_eq!(run_to_crash(TEST), (7, 3));
    }

    static TEST_2: &str = r#"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;

    #[test]
    fn test_last_cart() {
        assert_eq!(run_to_one(TEST_2), (6, 4));
    }
}
