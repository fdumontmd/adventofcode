use anyhow::Error;
use std::{
    convert::TryFrom,
    fmt::Display,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

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

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.grid.iter_mut()
    }

    pub fn replace_with(&self, grid: Vec<T>) -> Self {
        assert_eq!(grid.len(), self.grid.len());
        Self { grid, ..*self }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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
