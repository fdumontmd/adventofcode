static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, PartialEq)]
struct Direction(isize, isize);

// North (0, 1)
// South (0, -1)
// East  (1, 0)
// West  (-1, 0)
impl Direction {
    fn from_u8(d: u8) -> Self {
        match d {
            b'N' => Direction::north(),
            b'S' => Direction::south(),
            b'E' => Direction::east(),
            b'W' => Direction::west(),
            _ => panic!("unknown direction {}", d as char),
        }
    }
    fn north() -> Self {
        Self(0, 1)
    }

    fn south() -> Self {
        Self(0, -1)
    }

    fn east() -> Self {
        Self(1, 0)
    }

    fn west() -> Self {
        Self(-1, 0)
    }

    fn orient(&self, cmd: &str) -> Self {
        let rm: ((isize, isize), (isize, isize)) = match cmd {
            "R90" | "L270" => ((0, 1), (-1, 0)),
            "R270" | "L90" => ((0, -1), (1, 0)),
            "R180" | "L180" => ((-1, 0), (0, -1)),
            _ => panic!("unknown rotation {cmd}"),
        };

        Self(
            self.0 * rm.0 .0 + self.1 * rm.0 .1,
            self.0 * rm.1 .0 + self.1 * rm.1 .1,
        )
    }
}

struct Waypoint(isize, isize);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Orientation {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Orientation {
    fn from_str(o: &str) -> Self {
        match o {
            "N" => Orientation::North,
            "S" => Orientation::South,
            "E" => Orientation::East,
            "W" => Orientation::West,
            "L" => Orientation::Left,
            "R" => Orientation::Right,
            "F" => Orientation::Forward,
            _ => panic!("unknown orientation {o}"),
        }
    }

    fn is_rotation(&self) -> bool {
        self == &Orientation::Left || self == &Orientation::Right
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Command(Orientation, isize);

impl Command {
    fn from_str(cmd: &str) -> Self {
        let (o, d) = cmd.split_at(1);
        let o = Orientation::from_str(o);
        let d = d.parse().unwrap();
        Self(o, d)
    }
}

impl Waypoint {
    fn new() -> Self {
        Self(10, 1)
    }

    fn orient(&self, Command(o, a): Command) -> Self {
        use Orientation::{Left, Right};
        let rm: ((isize, isize), (isize, isize)) = match (o, a) {
            (Right, 90) | (Left, 270) => ((0, 1), (-1, 0)),
            (Right, 270) | (Left, 90) => ((0, -1), (1, 0)),
            (Right, 180) | (Left, 180) => ((-1, 0), (0, -1)),
            _ => panic!("unknown rotation ({o:?},{a})"),
        };

        Self(
            self.0 * rm.0 .0 + self.1 * rm.0 .1,
            self.0 * rm.1 .0 + self.1 * rm.1 .1,
        )
    }

    fn slide(&self, Command(o, d): Command) -> Self {
        use Orientation::{East, North, South, West};
        let o = match o {
            North => Direction::north(),
            South => Direction::south(),
            East => Direction::east(),
            West => Direction::west(),
            _ => panic!("unknown slide ({o:?},{d})"),
        };
        Self(self.0 + o.0 * d, self.1 + o.1 * d)
    }

    fn update(&mut self, cmd: Command) {
        if cmd.0.is_rotation() {
            *self = self.orient(cmd);
        } else {
            *self = self.slide(cmd);
        }
    }
}

struct Ship {
    x: isize,
    y: isize,
    d: Direction,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            d: Direction::east(),
        }
    }

    fn process_cmd(&mut self, cmd: &str) {
        match cmd.as_bytes()[0] {
            b'R' | b'L' => self.d = self.d.orient(cmd),
            b'F' => {
                let Some(dist) = cmd.strip_prefix('F') else { panic!() };
                let dist: isize = dist.parse().unwrap();
                self.x += self.d.0 * dist;
                self.y += self.d.1 * dist;
            }
            b'N' | b'S' | b'E' | b'W' => {
                let d = Direction::from_u8(cmd.as_bytes()[0]);
                let dist: isize = cmd.split_at(1).1.parse().unwrap();
                self.x += d.0 * dist;
                self.y += d.1 * dist;
            }
            _ => panic!("unknown movement {cmd}"),
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

struct Ship2 {
    position: (isize, isize),
    waypoint: Waypoint,
}

impl Ship2 {
    fn new() -> Self {
        Self {
            position: (0, 0),
            waypoint: Waypoint::new(),
        }
    }

    fn process_cmd(&mut self, cmd: &str) {
        let cmd = Command::from_str(cmd);
        if cmd.0 == Orientation::Forward {
            self.position.0 += cmd.1 * self.waypoint.0;
            self.position.1 += cmd.1 * self.waypoint.1;
        } else {
            self.waypoint.update(cmd)
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.position.0.abs() + self.position.1.abs()
    }
}

fn part_1(input: &str) -> isize {
    let mut ship = Ship::new();
    for cmd in input.lines() {
        ship.process_cmd(cmd);
    }
    ship.manhattan_distance()
}

fn part_2(input: &str) -> isize {
    let mut ship = Ship2::new();
    for cmd in input.lines() {
        ship.process_cmd(cmd);
    }
    ship.manhattan_distance()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, Direction, INPUT};

    static TEST_INPUT: &str = r"F10
N3
F7
R90
F11";

    #[test]
    fn test_direction() {
        let init = Direction::east();
        assert_eq!(Direction::south(), init.orient("R90"));
        assert_eq!(Direction::north(), init.orient("L90"));
        assert_eq!(Direction::west(), init.orient("R180"));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(25, part_1(TEST_INPUT));
        assert_eq!(796, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(286, part_2(TEST_INPUT));
        assert_eq!(39446, part_2(INPUT));
    }
}
