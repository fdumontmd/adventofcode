use anyhow::{bail, Error};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fmt::Display,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<u8> for Tile {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Tile::Floor),
            b'L' => Ok(Tile::Empty),
            b'#' => Ok(Tile::Occupied),
            _ => bail!("Unknown tile type '{}'", value as char),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Floor => '.',
                Tile::Empty => 'L',
                Tile::Occupied => '#',
            }
        )
    }
}

// promote some of this code to aoc-utils
// AdjacencyModel: enum with a distance function
//  - four neighbours -> taxicab
//  - 8 neighbours -> max
// GridModel: width and height, idx to pos and back,
// and neighbours within the grid (taking into account
// the AdjacencyModel)

#[derive(Debug, Copy, Clone)]
pub enum AdjacencyModel {
    Cross,
    Square,
}

pub trait Distance {
    fn distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize;
}

pub struct Taxicab;

impl Distance for Taxicab {
    fn distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
        pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
    }
}

pub struct MaxDist;

impl Distance for MaxDist {
    fn distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
        pos1.0.abs_diff(pos2.0).max(pos1.1.abs_diff(pos2.1))
    }
}

pub struct Grid<T, D> {
    grid: Vec<T>,
    am: PhantomData<D>,
    width: usize,
    height: usize,
}

impl<T, D> Grid<T, D> {
    pub fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    pub fn pos_to_idx(&self, pos: (usize, usize)) -> usize {
        pos.0 + self.width * pos.1
    }

    fn around_pos(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width;
        let height = self.height;
        (-1..=1)
            .flat_map(move |dy| {
                (-1..=1).filter_map(move |dx| {
                    match (pos.0.checked_add_signed(dx), pos.1.checked_add_signed(dy)) {
                        (Some(x), Some(y)) => Some((x, y)),
                        _ => None,
                    }
                })
            })
            .filter(move |(x, y)| *x < width && *y < height)
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    fn with(&self, grid: Vec<T>) -> Self {
        Self { grid, ..*self }
    }
}

impl<T, D: Distance> Grid<T, D> {
    pub fn neighbours(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        self.around_pos(pos)
            .filter(move |p| D::distance(pos, *p) == 1)
    }
}

impl<T: TryFrom<u8>, D> TryFrom<&str> for Grid<T, D>
where
    Error: From<<T as std::convert::TryFrom<u8>>::Error>,
{
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut width = 0;
        let grid: Vec<T> = value
            .bytes()
            .enumerate()
            .filter_map(|(idx, b)| {
                if b == 10 {
                    if width == 0 {
                        width = idx;
                    }
                    None
                } else {
                    Some(b)
                }
            })
            .map(|b| T::try_from(b))
            .collect::<Result<Vec<_>, _>>()?;
        let height = grid.len() / width;
        Ok(Self {
            am: PhantomData,
            grid,
            width,
            height,
        })
    }
}

impl<T, D> Index<(usize, usize)> for Grid<T, D> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[self.pos_to_idx(index)]
    }
}

impl<T, D> IndexMut<(usize, usize)> for Grid<T, D> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.pos_to_idx(index);
        &mut self.grid[idx]
    }
}

impl<T: Eq, D> PartialEq for Grid<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
            && self.am == other.am
            && self.width == other.width
            && self.height == other.height
    }
}

impl<T: Eq, D> Eq for Grid<T, D> {}

impl<T: Display, D> Display for Grid<T, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, t) in self.grid.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{t}")?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq)]
struct Seating {
    seats: Grid<Tile, MaxDist>,
}

impl Seating {
    fn from_str(input: &str) -> Result<Self, Error> {
        let grid = Grid::try_from(input)?;
        Ok(Self { seats: grid })
    }

    fn round(&self) -> Self {
        let grid = self
            .seats
            .iter()
            .enumerate()
            .map(|(idx, t)| {
                let adjacents = self
                    .seats
                    .neighbours(self.seats.idx_to_pos(idx))
                    .map(|p| self.seats[p])
                    .filter(|t| *t == Tile::Occupied)
                    .count();
                if *t == Tile::Empty && adjacents == 0 {
                    Tile::Occupied
                } else if *t == Tile::Occupied && adjacents >= 4 {
                    Tile::Empty
                } else {
                    *t
                }
            })
            .collect();
        Self {
            seats: self.seats.with(grid),
        }
    }

    fn count_occupied(&self) -> usize {
        self.seats.iter().filter(|t| **t == Tile::Occupied).count()
    }

    // iterate over the seats, and precompute the up to 8 visible
    // other seats
    fn advanced_seating_strategy(&self) -> AdvancedSeating {
        let mut neighbours = HashMap::new();

        self.seats
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == Tile::Empty)
            .for_each(|(idx, _)| {
                let mut visibles = Vec::new();
                let pos = self.seats.idx_to_pos(idx);

                // horizontal
                for x in (0..pos.0).rev() {
                    if self.seats[(x, pos.1)] == Tile::Empty {
                        visibles.push(self.seats.pos_to_idx((x, pos.1)));
                        break;
                    }
                }

                for x in pos.0 + 1..self.seats.width {
                    if self.seats[(x, pos.1)] == Tile::Empty {
                        visibles.push(self.seats.pos_to_idx((x, pos.1)));
                        break;
                    }
                }

                // vertical
                for y in (0..pos.1).rev() {
                    if self.seats[(pos.0, y)] == Tile::Empty {
                        visibles.push(self.seats.pos_to_idx((pos.0, y)));
                        break;
                    }
                }

                for y in pos.1 + 1..self.seats.height {
                    if self.seats[(pos.0, y)] == Tile::Empty {
                        visibles.push(self.seats.pos_to_idx((pos.0, y)));
                        break;
                    }
                }

                // diagonals
                for pos in (0..pos.0).rev().zip((0..pos.1).rev()) {
                    if self.seats[pos] == Tile::Empty {
                        visibles.push(self.seats.pos_to_idx(pos));
                        break;
                    }
                }

                for pos in (0..pos.0).rev().zip(pos.1 + 1..self.seats.height) {
                    if self.seats[pos] == Tile::Empty {
                        visibles.push(self.seats.pos_to_idx(pos));
                        break;
                    }
                }

                for pos in (pos.0 + 1..self.seats.width).zip((0..pos.1).rev()) {
                    if self.seats[pos] == Tile::Empty {
                        visibles.push(self.seats.pos_to_idx(pos));
                        break;
                    }
                }

                for pos in (pos.0 + 1..self.seats.width).zip(pos.1 + 1..self.seats.height) {
                    if self.seats[pos] == Tile::Empty {
                        visibles.push(self.seats.pos_to_idx(pos));
                        break;
                    }
                }

                neighbours.insert(idx, visibles);
            });

        AdvancedSeating {
            neighbours,
            occupied: vec![false; self.seats.width * self.seats.height],
            width: self.seats.width,
        }
    }
}

struct AdvancedSeating {
    width: usize,
    neighbours: HashMap<usize, Vec<usize>>,
    occupied: Vec<bool>,
}

impl AdvancedSeating {
    fn round(&mut self) -> bool {
        let occupied = self
            .occupied
            .iter()
            .enumerate()
            .map(|(idx, o)| {
                if let Some(visibles) = self.neighbours.get(&idx) {
                    let count = visibles.iter().filter(|n| self.occupied[**n]).count();
                    if *o {
                        count < 5
                    } else {
                        count == 0
                    }
                } else {
                    *o
                }
            })
            .collect();
        let res = self.occupied == occupied;
        self.occupied = occupied;
        res
    }

    fn count_occupied(&self) -> usize {
        self.occupied.iter().filter(|o| **o).count()
    }
}

impl Display for AdvancedSeating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, o) in self.occupied.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            let c = if self.neighbours.contains_key(&idx) {
                match o {
                    true => '#',
                    false => 'L',
                }
            } else {
                '.'
            };
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

fn part_1(input: &str) -> Result<usize, Error> {
    let mut seating = Seating::from_str(input)?;

    loop {
        let tmp = seating.round();
        if tmp == seating {
            break;
        }
        seating = tmp;
    }
    Ok(seating.count_occupied())
}

fn part_2(input: &str) -> Result<usize, Error> {
    let seating = Seating::from_str(input)?;
    let mut advanced_seating = seating.advanced_seating_strategy();
    while !advanced_seating.round() {}
    Ok(advanced_seating.count_occupied())
}

fn main() -> Result<(), Error> {
    println!("Part 1: {}", part_1(INPUT)?);
    println!("Part 2: {}", part_2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;

    #[test]
    fn check_1() {
        assert_eq!(37, part_1(TEST_INPUT).unwrap());
        assert_eq!(2329, part_1(INPUT).unwrap());
    }

    #[test]
    fn check_2() {
        assert_eq!(26, part_2(TEST_INPUT).unwrap());
        assert_eq!(2138, part_2(INPUT).unwrap());
    }
}
