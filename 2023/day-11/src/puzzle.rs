use std::{collections::HashMap, fmt::Display};

use aoc_utils::grid::{Distance, Grid, Taxicab};

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Universe {
    Galaxy,
    Space,
}

impl From<u8> for Universe {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Universe::Galaxy,
            _ => Universe::Space,
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self == &Universe::Galaxy { '#' } else { '.' })
    }
}

pub fn parse_puzzle(input: &str) -> Grid<Universe, Taxicab> {
    Grid::try_from(input).unwrap()
}

#[tracing::instrument]
pub fn solve_for_shift(input: &str, shift: usize) -> Result<String, AocError> {
    let shift = shift - 1;
    let grid = parse_puzzle(input);

    let mut galaxies: HashMap<(usize, usize), (usize, usize)> =
        HashMap::from_iter(grid.iter().enumerate().filter_map(|(idx, u)| {
            if *u == Universe::Galaxy {
                let pos = grid.idx_to_pos(idx);
                Some((pos, pos))
            } else {
                None
            }
        }));

    let mut yshift = 0;

    for y in 0..grid.height() {
        let mut empty = true;
        for x in 0..grid.width() {
            if grid[(x, y)] == Universe::Galaxy {
                empty = false;
                galaxies.get_mut(&(x, y)).map(|g| {
                    g.1 += yshift;
                    g
                });
            }
        }
        if empty {
            yshift += shift;
        }
    }

    let mut xshift = 0;

    for x in 0..grid.width() {
        let mut empty = true;
        for y in 0..grid.height() {
            if grid[(x, y)] == Universe::Galaxy {
                empty = false;
                galaxies.get_mut(&(x, y)).map(|g| {
                    g.0 += xshift;
                    g
                });
            }
        }

        if empty {
            xshift += shift;
        }
    }

    let mut galaxies = Vec::from_iter(galaxies.values());

    let mut sum = 0;

    while let Some(g) = galaxies.pop() {
        sum += galaxies
            .iter()
            .map(|og| Taxicab::distance(*g, **og))
            .sum::<usize>();
    }

    Ok(format!("{sum}"))
}
