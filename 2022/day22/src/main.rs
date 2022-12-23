// The code is not complete and whatever is present is a complete mess.
use aoc_utils::num::extended_gcd;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt::Display;

static INPUT: &str = include_str!("input.txt");
static INPUT_SIDE_MAPPING: [Neighbours; 6] = [
    Neighbours {
        up: Side(5, SideMapping::RightTurn),
        right: Side(1, SideMapping::Simple),
        down: Side(2, SideMapping::Simple),
        left: Side(3, SideMapping::Flip),
    },
    Neighbours {
        up: Side(5, SideMapping::Simple),
        right: Side(4, SideMapping::Flip),
        down: Side(2, SideMapping::RightTurn),
        left: Side(0, SideMapping::Simple),
    },
    Neighbours {
        up: Side(0, SideMapping::Simple),
        right: Side(1, SideMapping::LeftTurn),
        down: Side(4, SideMapping::Simple),
        left: Side(3, SideMapping::LeftTurn),
    },
    Neighbours {
        up: Side(2, SideMapping::RightTurn),
        right: Side(4, SideMapping::Simple),
        down: Side(5, SideMapping::Simple),
        left: Side(0, SideMapping::Flip),
    },
    Neighbours {
        up: Side(2, SideMapping::Simple),
        right: Side(1, SideMapping::Flip),
        down: Side(5, SideMapping::RightTurn),
        left: Side(3, SideMapping::Simple),
    },
    Neighbours {
        up: Side(3, SideMapping::Simple),
        right: Side(4, SideMapping::LeftTurn),
        down: Side(1, SideMapping::Simple),
        left: Side(0, SideMapping::LeftTurn),
    },
];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn turn_left(&self) -> Self {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Facing::Right => (1, 0),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Up => (0, -1),
        }
    }

    fn score(&self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GridItem {
    Nothing,
    Empty,
    Wall,
    Path,
}
impl GridItem {
    pub(crate) fn parse(b: u8) -> GridItem {
        match b {
            b' ' => GridItem::Nothing,
            b'.' => GridItem::Empty,
            b'#' => GridItem::Wall,
            _ => panic!("Unrecognized grid item {}", b),
        }
    }
}

impl Display for GridItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            GridItem::Nothing => ' ',
            GridItem::Empty => '.',
            GridItem::Wall => '#',
            GridItem::Path => '@',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Default)]
struct Grid {
    grid: Vec<Vec<GridItem>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for g in row {
                write!(f, "{}", g)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .take_while(|l| !l.trim().is_empty())
            .map(|l| l.bytes().map(GridItem::parse).collect())
            .collect();
        Self { grid }
    }
}

fn parse_all(input: &str) -> (Grid, Vec<String>) {
    let grid = Grid::parse(input);
    (grid, parse_instructions(input))
}

struct Pos {
    grid: Grid,
    pos: (usize, usize),
    facing: Facing,
}

impl Pos {
    fn new(grid: Grid) -> Self {
        let x = grid.grid[0].iter().position(|g| *g == GridItem::Empty);
        Pos {
            grid,
            pos: (x.unwrap(), 0),
            facing: Facing::Right,
        }
    }

    fn dim(&self) -> (usize, usize) {
        (
            self.grid.grid.iter().map(|r| r.len()).max().unwrap(),
            self.grid.grid.len(),
        )
    }

    fn move_by_count(&mut self, count: usize) {
        for _ in 0..count {
            if !self.move_forward() {
                break;
            }
        }
    }

    fn grid_at(&self, col: usize, row: usize) -> GridItem {
        if row < self.grid.grid.len() && col < self.grid.grid[row].len() {
            self.grid.grid[row][col]
        } else {
            GridItem::Nothing
        }
    }

    fn move_forward(&mut self) -> bool {
        let delta = self.facing.delta();
        let (max_col, max_row) = self.dim();
        let mut cur_col = self.pos.0;
        let mut cur_row = self.pos.1;
        loop {
            let new_col = match cur_col.checked_add_signed(delta.0) {
                Some(c) => c % max_col,
                None => max_col - cur_col - 1,
            };
            let new_row = match cur_row.checked_add_signed(delta.1) {
                Some(r) => r % max_row,
                None => max_row - cur_row - 1,
            };

            if self.grid_at(new_col, new_row) == GridItem::Wall {
                return false;
            }
            (cur_col, cur_row) = (new_col, new_row);

            // keep looking until we hit a wall or reach an empty coord
            if self.grid_at(new_col, new_row) == GridItem::Nothing {
                continue;
            }

            self.pos = (cur_col, cur_row);
            self.grid.grid[cur_row][cur_col] = GridItem::Path;
            return true;
        }
    }

    fn turn_left(&mut self) {
        self.facing = self.facing.turn_left();
    }

    fn turn_right(&mut self) {
        self.facing = self.facing.turn_right();
    }

    fn execute_instructions(&mut self, instructions: &Vec<String>) {
        for i in instructions {
            match &**i {
                "L" => self.turn_left(),
                "R" => self.turn_right(),
                d => self.move_by_count(d.parse().unwrap()),
            }
        }
    }

    fn position_score(&self) -> usize {
        (self.pos.1 + 1) * 1000 + (self.pos.0 + 1) * 4 + self.facing.score()
    }
}

fn parse_instructions(input: &str) -> Vec<String> {
    let input = input.lines().last().unwrap();
    let mut instr = Vec::new();
    for (_, group) in &input.chars().group_by(|d| d.is_ascii_digit()) {
        instr.push(group.collect());
    }
    instr
}

fn part_01(input: &str) -> usize {
    let (grid, instructions) = parse_all(input);
    let mut pos = Pos::new(grid);
    pos.execute_instructions(&instructions);
    pos.position_score()
}

// need Eq for unit testing the side mapping generation algo later
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SideMapping {
    Simple, // faces are neighbours
    LeftTurn,
    RightTurn,
    Flip,
}

// unit test
impl SideMapping {
    fn convert_pos(
        &self,
        pos: (usize, usize),
        side_len: usize,
        facing: Facing,
    ) -> ((usize, usize), Facing) {
        match self {
            SideMapping::Simple => {
                let new_pos = match facing {
                    Facing::Right => (0, pos.1),
                    Facing::Down => (pos.0, 0),
                    Facing::Left => (side_len - 1, pos.1),
                    Facing::Up => (pos.0, side_len - 1),
                };
                (new_pos, facing)
            }
            SideMapping::LeftTurn => {
                let new_pos = match facing {
                    Facing::Right => (pos.1, side_len - 1),
                    Facing::Down => (0, side_len - pos.0 - 1),
                    Facing::Left => (pos.1, 0),
                    Facing::Up => (side_len - 1, side_len - pos.0 - 1),
                };
                (new_pos, facing.turn_left())
            }
            SideMapping::RightTurn => {
                let new_pos = match facing {
                    Facing::Right => (side_len - pos.1 - 1, 0),
                    Facing::Down => (side_len - 1, pos.0),
                    Facing::Left => (side_len - pos.1 - 1, side_len - 1),
                    Facing::Up => (0, pos.0),
                };
                (new_pos, facing.turn_right())
            }
            SideMapping::Flip => {
                let new_pos = match facing {
                    Facing::Right => (pos.0, side_len - pos.1 - 1),
                    Facing::Down => (side_len - pos.0 - 1, pos.1),
                    Facing::Left => (pos.0, side_len - pos.1 - 1),
                    Facing::Up => (side_len - pos.0 - 1, pos.1),
                };
                (new_pos, facing.turn_left().turn_left())
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Side(usize, SideMapping);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Neighbours {
    up: Side,
    right: Side,
    down: Side,
    left: Side,
}

// we need a way to map from one face to another, using either translation
// (from instance, from 1 to 4, or 2 to 3, in the test data), and rotation (from 1 to 3)
// rotations can be repeated, such as 1 to 6 in test data (and presumably up to 3 times)
// working assumption: the number of rotations is taxicab distance - 1. If 0, that's a
// translation (from 0 to 49, and vice versa, along the dimension)

// probably need to work from the flat coordinates (col, row): using BFS to iterate through
// all faces starting from the clearly neighbours one, and using distance (plus counting bends?)
// to match sides:
// - direct neighbours: side left to right or top to bottom
// - 2 bends (taxicab distance: 2): left to top, left to bottom, right to top, right to bottom
// (depending on bend)
// - 2 bends + 1 straight (taxicab distance: 3): left to left, right to right, top to top or bottom
// to bottom (flip)
// - 2 bends + 2 straights (taxicab distance: 4): 3 rotations
//
struct Cube {
    side_len: usize,
    faces: [Grid; 6],
}

fn neighbours(pos: (usize, usize)) -> impl Iterator<Item = Option<(usize, usize)>> {
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .into_iter()
        .map(move |d| {
            pos.0
                .checked_add_signed(d.0)
                .and_then(|c| pos.1.checked_add_signed(d.1).map(|r| (c, r)))
        })
}

impl Cube {
    fn parse(input: &str) -> Self {
        let max_row = input.lines().take_while(|l| !l.trim().is_empty()).count();
        let max_col = input
            .lines()
            .take_while(|l| !l.trim().is_empty())
            .map(|l| l.len())
            .max()
            .unwrap();

        let (side_len, _, _) = extended_gcd(max_row as i64, max_col as i64);
        let side_len = side_len as usize;

        let mut sides = BTreeMap::new();
        let mut faces: [Grid; 6] = Default::default();

        input
            .lines()
            .take_while(|l| !l.trim().is_empty())
            .chunks(side_len)
            .into_iter()
            .enumerate()
            .for_each(|(row, c)| {
                for line in c {
                    line.chars()
                        .chunks(side_len)
                        .into_iter()
                        .enumerate()
                        .for_each(|(col, side)| {
                            let side = side.collect::<String>();
                            if !side.trim().is_empty() {
                                let sides_len = sides.len();
                                // row, col order so that first item is also the starting face
                                let side_id = sides.entry((row, col)).or_insert(sides_len);
                                faces[*side_id].grid.push(
                                    side.bytes()
                                        .map(|b| -> GridItem { GridItem::parse(b) })
                                        .collect(),
                                );
                            }
                        });
                }
            });

        Cube { side_len, faces }
    }
}

// ok, let's assume I can compute a neighbour sidemap (i.e. from face idx to a Neighbour struct)
// input: map from (col, row) -> face idx should be enough, + taxicab metric
// remember:
// - start with direct neighbours, map them
// - for each neighbour, expand perpendicularly (so Up neighbour, expand Left and Right), if found
// those are also neighbours, for successive sides (Up then Left -> counter clockwise, Up then
// Right -> clockwise); direct neighbours + their perpendicular neighbours are the ring of the
// original face
// - do that for all faces
// - complete with taxicab distance for faces that are not complete yet; remember first side
// outside ring is opposite of face, so no contact
fn build_neighbour_sidemaps() -> [Neighbours; 6] {
    todo!()
}

struct CubePos {
    cube: Cube,
    connections: [Neighbours; 6],
    original_coords: [(usize, usize); 6],
    pos: (usize, usize),
    face: usize,
    facing: Facing,
}

impl CubePos {
    fn new(cube: Cube, original_coords: [(usize, usize); 6], connections: [Neighbours; 6]) -> Self {
        Self {
            cube,
            original_coords,
            connections,
            pos: (0, 0),
            face: 0,
            facing: Facing::Right,
        }
    }

    fn move_forward(&mut self) -> bool {
        let delta = self.facing.delta();
        let new_pos = (
            self.pos.0.checked_add_signed(delta.0),
            self.pos.1.checked_add_signed(delta.1),
        );
        if let (Some(col), Some(row)) = new_pos {
            if col < self.cube.side_len && row < self.cube.side_len {
                if self.cube.faces[self.face].grid[row][col] == GridItem::Empty {
                    self.pos = (col, row);
                    return true;
                } else {
                    return false;
                }
            }
        }

        // we're on the edge of a face, trying to turn to another face

        let side = match self.facing {
            Facing::Right => &self.connections[self.face].right,
            Facing::Down => &self.connections[self.face].down,
            Facing::Left => &self.connections[self.face].left,
            Facing::Up => &self.connections[self.face].up,
        };
        let new_face = side.0;
        let ((col, row), new_facing) =
            side.1
                .convert_pos(self.pos, self.cube.side_len, self.facing);
        if self.cube.faces[new_face].grid[row][col] == GridItem::Empty {
            self.pos = (col, row);
            self.facing = new_facing;
            self.face = new_face;
            return true;
        }
        false
    }

    fn move_by_count(&mut self, count: usize) {
        for _ in 0..count {
            if !self.move_forward() {
                break;
            }
        }
    }

    fn execute_instructions(&mut self, instructions: &Vec<String>) {
        for i in instructions {
            match &**i {
                "L" => self.turn_left(),
                "R" => self.turn_right(),
                d => self.move_by_count(d.parse().unwrap()),
            }
        }
    }

    fn position_score(&self) -> usize {
        // need to map back to original map
        let col = self.original_coords[self.face].0 * self.cube.side_len + self.pos.0;
        let row = self.original_coords[self.face].1 * self.cube.side_len + self.pos.1;
        (row + 1) * 1000 + (col + 1) * 4 + self.facing.score()
    }

    fn turn_left(&mut self) {
        self.facing = self.facing.turn_left();
    }

    fn turn_right(&mut self) {
        self.facing = self.facing.turn_right();
    }
}

// based on hardcoded mapping:
// taxicab dist - 1 is number of rotation:
// 0 -> simple
// 1 -> left or right turn (need to figure out logic, still)
// 2 -> flip
// 3 -> right or left turn
// 4 -> SIMPLE!!! ()
// should never be 5 as we only have 6 faces total so max taxicab dist is 5

// but for now, I'll hand compute the mapping and hardcode it
fn part_02(_input: &str) -> usize {
    let coords = [(1, 0), (2, 0), (1, 1), (0, 2), (1, 2), (0, 3)];
    let cube = Cube::parse(INPUT);
    let instructions = parse_instructions(INPUT);
    let mut pos = CubePos::new(cube, coords, INPUT_SIDE_MAPPING);
    pos.execute_instructions(&instructions);
    pos.position_score()
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{
        parse_instructions, part_01, part_02, Cube, CubePos, Facing, Neighbours, Side, SideMapping,
        INPUT,
    };
    use test_case::test_case;

    static TEST_INPUT: &str = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    static TEST_CUBE_NEIGHBOURS: [Neighbours; 6] = [
        Neighbours {
            up: Side(1, SideMapping::Flip),
            right: Side(5, SideMapping::Flip),
            down: Side(3, SideMapping::Simple),
            left: Side(2, SideMapping::LeftTurn),
        },
        Neighbours {
            up: Side(0, SideMapping::Flip),
            right: Side(2, SideMapping::Simple),
            down: Side(4, SideMapping::Flip),
            left: Side(5, SideMapping::RightTurn),
        },
        Neighbours {
            up: Side(0, SideMapping::RightTurn),
            right: Side(3, SideMapping::Simple),
            down: Side(4, SideMapping::LeftTurn),
            left: Side(1, SideMapping::Simple),
        },
        Neighbours {
            up: Side(0, SideMapping::Simple),
            right: Side(5, SideMapping::RightTurn),
            down: Side(4, SideMapping::Simple),
            left: Side(2, SideMapping::Simple),
        },
        Neighbours {
            up: Side(3, SideMapping::Simple),
            right: Side(5, SideMapping::Simple),
            down: Side(1, SideMapping::Flip),
            left: Side(2, SideMapping::RightTurn),
        },
        Neighbours {
            up: Side(3, SideMapping::LeftTurn),
            right: Side(0, SideMapping::Flip),
            down: Side(1, SideMapping::RightTurn),
            left: Side(4, SideMapping::Simple),
        },
    ];
    #[test]
    fn test_part_01() {
        assert_eq!(6032, part_01(TEST_INPUT));
    }

    #[test]
    fn real_part_01() {
        assert_eq!(106094, part_01(INPUT));
    }

    #[test]
    fn test_part_02() {
        let coords = [(2, 0), (0, 1), (1, 1), (2, 1), (2, 2), (3, 2)];
        let cube = Cube::parse(TEST_INPUT);
        let instructions = parse_instructions(TEST_INPUT);
        let mut pos = CubePos::new(cube, coords, TEST_CUBE_NEIGHBOURS);
        pos.execute_instructions(&instructions);
        assert_eq!(5031, pos.position_score());
    }

    #[test]
    fn real_part_02() {
        assert_eq!(162038, part_02(INPUT));
    }

    #[test_case(SideMapping::Simple, 5, (1, 0), Facing::Up, (1, 4), Facing::Up)]
    #[test_case(SideMapping::Simple, 5, (1, 4), Facing::Down, (1, 0), Facing::Down)]
    #[test_case(SideMapping::Simple, 5, (0, 1), Facing::Left, (4, 1), Facing::Left)]
    #[test_case(SideMapping::Simple, 5, (4, 1), Facing::Right, (0, 1), Facing::Right)]
    #[test_case(SideMapping::LeftTurn, 5, (1, 0), Facing::Up, (4, 3), Facing::Left)]
    #[test_case(SideMapping::LeftTurn, 5, (1, 4), Facing::Down, (0, 3), Facing::Right)]
    #[test_case(SideMapping::LeftTurn, 5, (0, 1), Facing::Left, (1, 0), Facing::Down)]
    #[test_case(SideMapping::LeftTurn, 5, (4, 1), Facing::Right, (1, 4), Facing::Up)]
    #[test_case(SideMapping::RightTurn, 5, (1, 0), Facing::Up, (0, 1), Facing::Right)]
    #[test_case(SideMapping::RightTurn, 5, (1, 4), Facing::Down, (4, 1), Facing::Left)]
    #[test_case(SideMapping::RightTurn, 5, (0, 1), Facing::Left, (3, 4), Facing::Up)]
    #[test_case(SideMapping::RightTurn, 5, (4, 1), Facing::Right, (3, 0), Facing::Down)]
    #[test_case(SideMapping::Flip, 5, (1, 0), Facing::Up, (3, 0), Facing::Down)]
    #[test_case(SideMapping::Flip, 5, (1, 4), Facing::Down, (3, 4), Facing::Up)]
    #[test_case(SideMapping::Flip, 5, (0, 1), Facing::Left, (0, 3), Facing::Right)]
    #[test_case(SideMapping::Flip, 5, (4, 1), Facing::Right, (4, 3), Facing::Left)]
    fn test_side_mappings(
        mapping: SideMapping,
        side_len: usize,
        from: (usize, usize),
        from_facing: Facing,
        to: (usize, usize),
        to_facing: Facing,
    ) {
        assert_eq!(
            (to, to_facing),
            mapping.convert_pos(from, side_len, from_facing)
        );
    }
}
