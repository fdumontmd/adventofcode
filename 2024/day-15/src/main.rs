use std::fmt::Display;

use aoc_utils::grid::Grid;

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Wall,
    Empty,
    Box,
    Robot,
    LeftBox,
    RightBox,
}
impl Tile {
    fn is_box(&self) -> bool {
        matches!(self, Tile::Box | Tile::LeftBox | Tile::RightBox)
    }
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Tile::Wall,
            b'.' => Tile::Empty,
            b'O' => Tile::Box,
            b'@' => Tile::Robot,
            b'[' => Tile::LeftBox,
            b']' => Tile::RightBox,
            _ => panic!("unknown tile {}", value as char),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = match self {
            Tile::Wall => '#',
            Tile::Empty => '.',
            Tile::Box => 'O',
            Tile::Robot => '@',
            Tile::LeftBox => '[',
            Tile::RightBox => ']',
        };
        write!(f, "{c}")
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Instruction::Up),
            b'v' => Ok(Instruction::Down),
            b'<' => Ok(Instruction::Left),
            b'>' => Ok(Instruction::Right),
            _ => Err(()),
        }
    }
}

impl Instruction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Instruction::Up => (0, -1),
            Instruction::Down => (0, 1),
            Instruction::Left => (-1, 0),
            Instruction::Right => (1, 0),
        }
    }

    fn target(&self, pos: (usize, usize)) -> (usize, usize) {
        let delta = self.delta();
        (
            pos.0.checked_add_signed(delta.0).unwrap(),
            pos.1.checked_add_signed(delta.1).unwrap(),
        )
    }
    fn is_vertical(&self) -> bool {
        matches!(self, Instruction::Up | Instruction::Down)
    }
}

// can_move is part2 specific as part1 does not need the kind of look ahead that is
// required for part2 vertical movements
// that means that can_move does not work for part1
fn can_move(grid: &Grid<Tile, ()>, pos: (usize, usize), instruction: Instruction) -> bool {
    let target = instruction.target(pos);
    if grid[target] == Tile::Empty {
        true
    } else if grid[target] == Tile::LeftBox {
        if instruction.is_vertical() {
            can_move(grid, target, instruction)
                && can_move(grid, (target.0 + 1, target.1), instruction)
        } else {
            can_move(grid, target, instruction)
        }
    } else if grid[target] == Tile::RightBox {
        if instruction.is_vertical() {
            can_move(grid, target, instruction)
                && can_move(grid, (target.0 - 1, target.1), instruction)
        } else {
            can_move(grid, target, instruction)
        }
    } else {
        false
    }
}

// try simple move that can succeed and be done, or fail. Does not handle parallel movement
// of large boxes; so can_move must be called before to confirm that the move is possible
fn try_move(grid: &mut Grid<Tile, ()>, pos: (usize, usize), instruction: Instruction) -> bool {
    let target = instruction.target(pos);
    if grid[target] == Tile::Wall {
        false
    } else if grid[target] == Tile::Empty
        || (instruction.is_vertical()
            && grid[target] == Tile::LeftBox
            && try_move(grid, target, instruction)
            && try_move(grid, (target.0 + 1, target.1), instruction))
        || (instruction.is_vertical()
            && grid[target] == Tile::RightBox
            && try_move(grid, target, instruction)
            && try_move(grid, (target.0 - 1, target.1), instruction))
        || (grid[target].is_box() && try_move(grid, target, instruction))
    {
        grid[target] = grid[pos];
        grid[pos] = Tile::Empty;
        true
    } else {
        false
    }
}

fn get_robot_pos(grid: &Grid<Tile, ()>) -> (usize, usize) {
    let idx = grid
        .iter()
        .enumerate()
        .find_map(|(idx, t)| if *t == Tile::Robot { Some(idx) } else { None })
        .unwrap();
    grid.idx_to_pos(idx)
}

fn gps_sum(grid: &Grid<Tile, ()>) -> usize {
    grid.iter()
        .enumerate()
        .filter_map(|(idx, t)| {
            if *t == Tile::Box || *t == Tile::LeftBox {
                let pos = grid.idx_to_pos(idx);
                Some(100 * pos.1 + pos.0)
            } else {
                None
            }
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut grid: Grid<Tile, ()> = Grid::try_from(parts[0]).unwrap();
    let instructions: Vec<Instruction> = parts[1]
        .trim()
        .bytes()
        .filter_map(|b| Instruction::try_from(b).ok())
        .collect();

    // find robot pos
    let mut pos = get_robot_pos(&grid);
    for i in instructions {
        if try_move(&mut grid, pos, i) {
            pos = i.target(pos);
        }
    }

    gps_sum(&grid)
}

fn part2(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let instructions: Vec<Instruction> = parts[1]
        .trim()
        .bytes()
        .filter_map(|b| Instruction::try_from(b).ok())
        .collect();
    // just rewrite map and apply grid again
    let mut new_map = String::new();
    for c in parts[0].chars() {
        new_map += match c {
            '#' => "##",
            '.' => "..",
            'O' => "[]",
            '@' => "@.",
            _ => "\n",
        }
    }
    let mut grid: Grid<Tile, ()> = Grid::try_from(&*new_map).unwrap();

    // find robot pos
    let mut pos = get_robot_pos(&grid);
    for i in instructions {
        if can_move(&grid, pos, i) && try_move(&mut grid, pos, i) {
            pos = i.target(pos);
        }
    }
    gps_sum(&grid)
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const SMALL_TEST_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const LARGE_TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test_case(SMALL_TEST_INPUT, 2028; "small test input")]
    #[test_case(LARGE_TEST_INPUT, 10092; "large test input")]
    #[test_case(INPUT, 1505963; "input")]
    fn test_part1(input: &str, gps_sum: usize) {
        assert_eq!(gps_sum, part1(input));
    }

    #[test_case(LARGE_TEST_INPUT, 9021; "large test input")]
    #[test_case(INPUT, 1543141; "input")]
    fn test_part2(input: &str, gps_sum: usize) {
        assert_eq!(gps_sum, part2(input));
    }
}
