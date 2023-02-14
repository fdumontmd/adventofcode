use colorized::*;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
};

static INPUT: &str = include_str!("input.txt");

// looks like but TEST and INPUT have 10x10 tiles
const SIDE_LEN: usize = 10;

#[derive(Clone, Eq, PartialEq)]
struct SquareGrid {
    side_len: usize,
    grid: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq)]
struct Tile {
    id: u64,
    grid: Vec<u8>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Tile {
    fn from_str(input: &str) -> Self {
        let input: Vec<_> = input.lines().collect();
        assert_eq!(SIDE_LEN + 1, input.len());
        let Some(id) = input[0].strip_prefix("Tile ") else { panic!("cannot parse Tile id: {}", input[0])};
        let Some(id) = id.strip_suffix(':')else { panic!("cannot parse Tile id: {}", input[0])};
        let id = id.parse().unwrap();
        let mut grid = Vec::new();
        for l in &input[1..] {
            grid.extend(l.bytes());
        }
        Self { id, grid }
    }

    fn get_edge_ids(&self) -> Vec<u32> {
        let mut top = 0;
        let mut bottom = 0;
        for x in 0..SIDE_LEN {
            top *= 2;
            top += (self[(x, 0)] == b'#') as u32;
            bottom *= 2;
            bottom += (self[(SIDE_LEN - x - 1, SIDE_LEN - 1)] == b'#') as u32;
        }

        let mut left = 0;
        let mut right = 0;
        for y in 0..SIDE_LEN {
            left *= 2;
            left += (self[(0, SIDE_LEN - y - 1)] == b'#') as u32;
            right *= 2;
            right += (self[(SIDE_LEN - 1, y)] == b'#') as u32;
        }

        vec![top, right, bottom, left]
    }

    fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Left => {
                let mut other = self.clone();
                for (idx, b) in self.grid.iter().enumerate() {
                    let (x, y) = (idx % SIDE_LEN, idx / SIDE_LEN);
                    other[(y, SIDE_LEN - x - 1)] = *b;
                }
                self.grid = other.grid;
            }
            Turn::Around => self.grid.reverse(),
            Turn::Right => {
                self.turn(Turn::Left);
                self.turn(Turn::Around);
            }
        }
    }

    fn flip_horizontally(&mut self) {
        self.grid = self
            .grid
            .chunks(SIDE_LEN)
            .flat_map(|c| c.iter().cloned().rev())
            .collect()
    }

    fn flip_vertically(&mut self) {
        self.grid = self
            .grid
            .chunks(SIDE_LEN)
            .rev()
            .flat_map(|c| c.iter().cloned())
            .collect()
    }

    fn align_left(&mut self, edge_id: u32) {
        let redge_id = reverse_edge_id(edge_id);
        if let Some(pos) = self
            .get_edge_ids()
            .into_iter()
            .position(|id| id == redge_id)
        {
            // just rotation
            match pos {
                0 => self.turn(Turn::Left),
                1 => self.turn(Turn::Around),
                2 => self.turn(Turn::Right),
                _ => {}
            }
        } else if let Some(pos) = self.get_edge_ids().into_iter().position(|id| id == edge_id) {
            match pos {
                0 => {
                    self.turn(Turn::Left);
                    self.flip_vertically();
                }
                1 => self.flip_horizontally(),
                2 => {
                    self.turn(Turn::Right);
                    self.flip_vertically();
                }
                _ => self.flip_vertically(),
            }
        }
        assert_eq!(redge_id, self.get_edge_ids()[3]);
    }

    fn align_top(&mut self, edge_id: u32) {
        //  bottom edge is reverse of top edge, so invert first
        let redge_id = reverse_edge_id(edge_id);
        if let Some(pos) = self
            .get_edge_ids()
            .into_iter()
            .position(|id| id == redge_id)
        {
            match pos {
                0 => {}
                1 => self.turn(Turn::Left),
                2 => self.turn(Turn::Around),
                _ => self.turn(Turn::Right),
            }
        } else if let Some(pos) = self.get_edge_ids().into_iter().position(|id| id == edge_id) {
            match pos {
                0 => self.flip_horizontally(),
                1 => {
                    self.turn(Turn::Left);
                    self.flip_horizontally();
                }
                2 => self.flip_vertically(),
                _ => {
                    self.turn(Turn::Right);
                    self.flip_horizontally();
                }
            }
        }
        assert_eq!(redge_id, self.get_edge_ids()[0]);
    }

    fn assert_match_edge(&self, other: &Tile, direction: Direction) {
        let (m, o) = match direction {
            Direction::Top => (0, 2),
            Direction::Right => (1, 3),
            Direction::Bottom => (2, 0),
            Direction::Left => (3, 1),
        };

        assert_eq!(
            self.get_edge_ids()[m],
            reverse_edge_id(other.get_edge_ids()[o])
        );
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, b) in self.grid.iter().enumerate() {
            if idx > 0 && idx % SIDE_LEN == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", *b as char)?;
        }
        Ok(())
    }
}

enum Turn {
    Left,
    Around,
    Right,
}

fn reverse_edge_id(edge_id: u32) -> u32 {
    assert!(edge_id < (1 << SIDE_LEN));
    edge_id.reverse_bits() >> (32 - SIDE_LEN)
}

impl Index<(usize, usize)> for Tile {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0 + SIDE_LEN * index.1]
    }
}

impl IndexMut<(usize, usize)> for Tile {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0 + SIDE_LEN * index.1]
    }
}

struct Tiles {
    tiles: Vec<Tile>,
}

impl Tiles {
    fn from_str(input: &str) -> Self {
        let tiles = input
            .split("\n\n")
            .filter(|i| !i.is_empty())
            .map(Tile::from_str)
            .collect();
        Self { tiles }
    }

    fn find_tile(&self, tile_id: u64) -> &Tile {
        self.tiles.iter().find(|t| t.id == tile_id).unwrap()
    }

    fn find_tile_mut(&mut self, tile_id: u64) -> &mut Tile {
        self.tiles.iter_mut().find(|t| t.id == tile_id).unwrap()
    }

    fn find_compabible_tiles(&self, tile_id: u64) -> Vec<u64> {
        let tile = self.find_tile(tile_id);
        let side_ids = tile.get_edge_ids();

        self.tiles
            .iter()
            .filter(|t| {
                if t.id == tile_id {
                    false
                } else {
                    t.get_edge_ids()
                        .iter()
                        .any(|id| side_ids.contains(id) || side_ids.contains(&reverse_edge_id(*id)))
                }
            })
            .map(|t| t.id)
            .collect()
    }

    // will find all possible corner tiles; and there are only
    // 4 in both test and actual input data
    fn find_candidate_corners(&self) -> Vec<u64> {
        self.tiles
            .iter()
            .filter_map(|tile| {
                if self.find_compabible_tiles(tile.id).len() == 2 {
                    Some(tile.id)
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_right_or_bottom_tiles(
        &self,
        tile: &Tile,
        map: &HashMap<(usize, usize), u64>,
    ) -> (Option<u64>, Option<u64>) {
        let compatibles = self.find_compabible_tiles(tile.id);
        let ids = tile.get_edge_ids();

        let right = ids[1];
        let bottom = ids[2];

        let right: Vec<_> = compatibles
            .iter()
            .cloned()
            .filter(|&id| {
                let t = self.find_tile(id);
                let ids = t.get_edge_ids();
                !map.values().any(|i| *i == id)
                    && (ids.contains(&right) || ids.contains(&reverse_edge_id(right)))
            })
            .collect();
        let bottom: Vec<_> = compatibles
            .into_iter()
            .filter(|id| {
                let t = self.find_tile(*id);
                let ids = t.get_edge_ids();
                !map.values().any(|i| i == id)
                    && (ids.contains(&bottom) || ids.contains(&reverse_edge_id(bottom)))
            })
            .collect();
        assert!(right.len() <= 1);
        assert!(bottom.len() <= 1);

        (right.first().cloned(), bottom.first().cloned())
    }

    fn add_right_or_bottom(&mut self, map: &mut HashMap<(usize, usize), u64>, pos: (usize, usize)) {
        let tile_id = *map.get(&pos).unwrap();
        let tile = self.find_tile(tile_id);
        let (r, b) = self.find_right_or_bottom_tiles(tile, map);
        let ids = tile.get_edge_ids();
        let r_id = ids[1];
        let b_id = ids[2];
        if let Some(r) = r {
            let r_tile = self.find_tile_mut(r);
            r_tile.align_left(r_id);
            map.insert((pos.0 + 1, pos.1), r);
        }

        if let Some(b) = b {
            let b_tile = self.find_tile_mut(b);
            b_tile.align_top(b_id);
            map.insert((pos.0, pos.1 + 1), b);
        }
        if r.is_some() {
            self.add_right_or_bottom(map, (pos.0 + 1, pos.1));
        }

        if b.is_some() {
            self.add_right_or_bottom(map, (pos.0, pos.1 + 1));
        }
    }

    fn build_map(&mut self) -> SquareGrid {
        let top_left_corner = self
            .find_candidate_corners()
            .into_iter()
            .find_map(|tile_id| {
                let tile = self.find_tile(tile_id);
                let compatibles = self.find_compabible_tiles(tile_id);
                let ids = tile.get_edge_ids();

                let right = ids[1];
                let bottom = ids[2];

                let right = compatibles.iter().cloned().find(|&id| {
                    let t = self.find_tile(id);
                    let ids = t.get_edge_ids();
                    ids.contains(&right) || ids.contains(&reverse_edge_id(right))
                });
                let bottom = compatibles.into_iter().find(|id| {
                    let t = self.find_tile(*id);
                    let ids = t.get_edge_ids();
                    ids.contains(&bottom) || ids.contains(&reverse_edge_id(bottom))
                });
                right.and_then(|r| bottom.map(move |b| (tile_id, r, b)))
            })
            .unwrap();

        let mut map = HashMap::new();
        map.insert((0, 0), top_left_corner.0);
        self.add_right_or_bottom(&mut map, (0, 0));

        // sanity check
        for (pos, tile_id) in &map {
            let tile = self.find_tile(*tile_id);
            for d in [
                Direction::Top,
                Direction::Right,
                Direction::Bottom,
                Direction::Left,
            ]
            .into_iter()
            {
                if let Some(other_pos_id) = pos_neighbour(*pos, d).and_then(|p| map.get(&p)) {
                    let other_tile = self.find_tile(*other_pos_id);
                    tile.assert_match_edge(other_tile, d);
                }
            }
        }

        let top_left = *map.keys().min().unwrap();
        assert_eq!((0, 0), top_left);
        let bottom_right = *map.keys().max().unwrap();
        assert_eq!(bottom_right.0, bottom_right.1);
        assert_eq!(
            self.tiles.len(),
            (bottom_right.0 + 1) * (bottom_right.1 + 1)
        );

        let side_len = (SIDE_LEN - 2) * (bottom_right.0 + 1);

        let mut grid = SquareGrid::new(side_len);

        for (pos, tile_id) in &map {
            let tile = self.find_tile(*tile_id);
            for x in 1..(SIDE_LEN - 1) {
                for y in 1..(SIDE_LEN - 1) {
                    grid[(
                        pos.0 * (SIDE_LEN - 2) + x - 1,
                        pos.1 * (SIDE_LEN - 2) + y - 1,
                    )] = tile[(x, y)];
                }
            }
        }

        // make sure we've written everything
        assert!(!grid.grid.iter().any(|b| *b == 0));

        grid
    }
}

fn pos_neighbour(pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Top => {
            if pos.1 > 0 {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
        Direction::Right => Some((pos.0 + 1, pos.1)),
        Direction::Bottom => Some((pos.0, pos.1 + 1)),
        Direction::Left => {
            if pos.0 > 0 {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
    }
}

impl SquareGrid {
    fn new(side_len: usize) -> Self {
        Self {
            side_len,
            grid: vec![0; side_len * side_len],
        }
    }
    fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Left => {
                let mut other = self.clone();
                for (idx, b) in self.grid.iter().enumerate() {
                    let (x, y) = (idx % self.side_len, idx / self.side_len);
                    other[(y, self.side_len - x - 1)] = *b;
                }
                self.grid = other.grid;
            }
            Turn::Around => self.grid.reverse(),
            Turn::Right => {
                self.turn(Turn::Left);
                self.turn(Turn::Around);
            }
        }
    }

    fn flip_horizontally(&mut self) {
        self.grid = self
            .grid
            .chunks(self.side_len)
            .flat_map(|c| c.iter().cloned().rev())
            .collect()
    }

    fn count_byte(&self, b: u8) -> usize {
        self.grid.iter().cloned().filter(|gb| *gb == b).count()
    }

    fn find_all_matching_pos<'b, 'a: 'b>(
        &'a self,
        byte: u8,
        pos: &'b [usize],
    ) -> impl Iterator<Item = usize> + 'b {
        let match_len = pos.last().unwrap() + 1;
        self.grid
            .windows(match_len)
            .enumerate()
            .filter_map(move |(idx, w)| {
                if pos.iter().all(|p| w[*p] == byte) {
                    Some(idx)
                } else {
                    None
                }
            })
    }
}

impl Display for SquareGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, b) in self.grid.iter().enumerate() {
            if idx > 0 && idx % self.side_len == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", *b as char)?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for SquareGrid {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0 + self.side_len * index.1]
    }
}

impl IndexMut<(usize, usize)> for SquareGrid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0 + self.side_len * index.1]
    }
}

fn part_1(input: &str) -> u64 {
    let tiles = Tiles::from_str(input);
    tiles.find_candidate_corners().iter().product()
}

static SNAKE: &str = r"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

const SNAKE_WIDTH: usize = 20;

fn convert_snake_to_pos_seq(snake: &str, side_len: usize) -> Vec<usize> {
    let pos: Vec<Vec<usize>> = snake
        .lines()
        .map(|l| {
            l.bytes()
                .enumerate()
                .filter_map(|(idx, b)| if b == b'#' { Some(idx) } else { None })
                .collect()
        })
        .collect();

    pos.into_iter()
        .enumerate()
        .flat_map(move |(idx, s)| s.into_iter().map(move |pos| pos + idx * side_len))
        .collect()
}

fn part_2(input: &str) -> usize {
    // need to build map; start from one of the corner, identifying the one that has a right and
    // bottom matching tiles, and work from there; looks like the building will be deterministic
    // so greedy algo should work
    //
    // then, one we have the map, rebuild it stripping the tile edges, and linearize
    // then, check the width of the map, and build a pattern that will match the snake (regex?)
    // by adding the space between the parts of the snake so that we can linearize the snake too
    // then search for matching positions, make sure we're close enough to the logical left edge
    // and change the snake Xs to Os
    // then count the remaining Xs
    let mut tiles = Tiles::from_str(input);
    let mut map = tiles.build_map();
    let snake = convert_snake_to_pos_seq(SNAKE, map.side_len);

    // check all possible orientation and flip
    let mut snake_pos = find_all_snake_starting_pos(&map, &snake);

    if snake_pos.is_empty() {
        map.turn(Turn::Left);
        snake_pos = find_all_snake_starting_pos(&map, &snake);
    }

    if snake_pos.is_empty() {
        map.turn(Turn::Left);
        snake_pos = find_all_snake_starting_pos(&map, &snake);
    }

    if snake_pos.is_empty() {
        map.turn(Turn::Left);
        snake_pos = find_all_snake_starting_pos(&map, &snake);
    }

    if snake_pos.is_empty() {
        map.flip_horizontally();
        snake_pos = find_all_snake_starting_pos(&map, &snake);
    }

    if snake_pos.is_empty() {
        map.turn(Turn::Left);
        snake_pos = find_all_snake_starting_pos(&map, &snake);
    }

    if snake_pos.is_empty() {
        map.turn(Turn::Left);
        snake_pos = find_all_snake_starting_pos(&map, &snake);
    }

    if snake_pos.is_empty() {
        map.turn(Turn::Left);
        snake_pos = find_all_snake_starting_pos(&map, &snake);
    }

    assert!(!snake_pos.is_empty());

    for pos in snake_pos {
        for delta in &snake {
            map.grid[pos + delta] = b'O';
        }
    }

    let res = map.count_byte(b'#');
    let mut color_map = ColoredSquareGrid::new(map);
    color_map.with_color(b'#', Colors::BlueFg);
    color_map.with_color(b'O', Colors::GreenFg);
    println!("{color_map}");
    res
}

fn find_all_snake_starting_pos(map: &SquareGrid, snake: &[usize]) -> Vec<usize> {
    let snake_pos: Vec<_> = map
        .find_all_matching_pos(b'#', snake)
        .filter(|p| p % map.side_len <= (map.side_len - SNAKE_WIDTH))
        .collect();
    snake_pos
}

struct ColoredSquareGrid {
    grid: SquareGrid,
    colors: HashMap<u8, Colors>,
}

impl ColoredSquareGrid {
    fn new(grid: SquareGrid) -> Self {
        Self {
            grid,
            colors: HashMap::new(),
        }
    }

    fn with_color(&mut self, b: u8, c: Colors) {
        self.colors.insert(b, c);
    }
}

impl Display for ColoredSquareGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, b) in self.grid.grid.iter().enumerate() {
            if idx > 0 && idx % self.grid.side_len == 0 {
                writeln!(f)?;
            }

            if let Some(c) = self.colors.get(b) {
                write!(f, "{}{}{}", c.value(), *b as char, Colors::Reset.value())?;
            } else {
                write!(f, "{}", *b as char)?;
            }
        }
        Ok(())
    }
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, reverse_edge_id, INPUT};

    static TEST_INPUT: &str = r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_invert_id() {
        assert_eq!(reverse_edge_id(reverse_edge_id(999)), 999);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(20899048083289, part_1(TEST_INPUT));
        assert_eq!(13224049461431, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(273, part_2(TEST_INPUT));
        assert_eq!(2231, part_2(INPUT));
    }
}
