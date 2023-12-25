use std::fmt::Display;

// if you're going into one direction, you're
// coming from the opposite direction
// so going South means coming from the North
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }

    pub fn from_delta(d: (isize, isize)) -> Option<Self> {
        match d {
            (0, -1) => Some(Direction::North),
            (0, 1) => Some(Direction::South),
            (-1, 0) => Some(Direction::West),
            (1, 0) => Some(Direction::East),
            _ => None,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        DIRECTIONS.iter().cloned()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Start,   // S
    Ground,  // .
    Vert,    // |
    Horiz,   // -
    NE,      // L
    NW,      // J
    SW,      // 7
    SE,      // F
    Outside, // O
    Inside,  // I
    Loop,    // *
}

impl Tile {
    pub fn step(&self, dir: Direction) -> Direction {
        match self {
            Tile::Start => dir,
            Tile::Ground => panic!("stepped on ground"),
            Tile::Vert => dir,
            Tile::Horiz => dir,
            Tile::NE => match dir {
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                _ => panic!("Reached NE from {:?}", dir),
            },
            Tile::NW => match dir {
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                _ => panic!("Reached NW from {:?}", dir),
            },
            Tile::SW => match dir {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                _ => panic!("Reached SW from {:?}", dir),
            },
            Tile::SE => match dir {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                _ => panic!("Reached SE from {:?}", dir),
            },
            Tile::Outside | Tile::Inside | Tile::Loop => panic!("stepped on ground"),
        }
    }

    pub fn valid_for_dir(&self, dir: Direction) -> bool {
        match self {
            Tile::Start => true,
            Tile::Ground => false,
            Tile::Vert => dir == Direction::North || dir == Direction::South,
            Tile::Horiz => dir == Direction::East || dir == Direction::West,
            Tile::NE => dir == Direction::South || dir == Direction::West,
            Tile::NW => dir == Direction::South || dir == Direction::East,
            Tile::SW => dir == Direction::North || dir == Direction::East,
            Tile::SE => dir == Direction::North || dir == Direction::West,
            Tile::Outside | Tile::Inside | Tile::Loop => false,
        }
    }
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Tile::Start,
            b'.' => Tile::Ground,
            b'|' => Tile::Vert,
            b'-' => Tile::Horiz,
            b'L' => Tile::NE,
            b'J' => Tile::NW,
            b'7' => Tile::SW,
            b'F' => Tile::SE,
            _ => panic!("Unknown tile {}", value as char),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Start => 'S',
                Tile::Ground => '.',
                Tile::Vert => '|',
                Tile::Horiz => '-',
                Tile::NE => 'L',
                Tile::NW => 'J',
                Tile::SW => '7',
                Tile::SE => 'F',
                Tile::Outside => 'O',
                Tile::Inside => 'I',
                Tile::Loop => '*',
            },
        )
    }
}
