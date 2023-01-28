use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use anyhow::{bail, Error};
use intcode::Computer;

static INPUT: &str = include_str!("input.txt");

fn new_pos(dir: i64, pos: (i64, i64)) -> (i64, i64) {
    match dir {
        1 => (pos.0, pos.1 - 1),
        2 => (pos.0, pos.1 + 1),
        3 => (pos.0 - 1, pos.1),
        4 => (pos.0 + 1, pos.1),
        _ => unreachable!(),
    }
}

fn part_01(input: &str) -> Result<usize, Error> {
    let comp: Computer = input.parse()?;

    let mut fringe = BinaryHeap::new();
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    for dir in 1..=4 {
        fringe.push((Reverse(1), dir, new_pos(dir, (0i64, 0i64)), comp.clone()))
    }

    while let Some((steps, dir, pos, mut comp)) = fringe.pop() {
        seen.insert(pos);
        comp.add_input(dir);
        let Some(output) = comp.wait_until_output() else { continue };
        match output {
            0 => {}
            1 => {
                for dir in 1..=4 {
                    let new_pos = new_pos(dir, pos);
                    if !seen.contains(&new_pos) {
                        fringe.push((Reverse(steps.0 + 1), dir, new_pos, comp.clone()))
                    }
                }
            }
            2 => return Ok(steps.0),
            _ => {
                bail!("Computer returned {output}")
            }
        }
    }

    bail!("cannot find oxygen")
}

#[derive(Debug)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

fn part_02(input: &str) -> Result<usize, Error> {
    let comp: Computer = input.parse()?;

    let mut fringe = BinaryHeap::new();
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    let mut map = HashMap::new();
    map.insert((0, 0), Tile::Empty);

    for dir in 1..=4 {
        fringe.push((Reverse(1), dir, new_pos(dir, (0i64, 0i64)), comp.clone()))
    }

    let mut oxygen_pos = (0, 0);

    while let Some((steps, dir, pos, mut comp)) = fringe.pop() {
        seen.insert(pos);
        comp.add_input(dir);
        let Some(output) = comp.wait_until_output() else { continue };
        match output {
            0 => {
                map.insert(pos, Tile::Wall);
            }
            1 => {
                map.insert(pos, Tile::Empty);
                for dir in 1..=4 {
                    let new_pos = new_pos(dir, pos);
                    if !seen.contains(&new_pos) {
                        fringe.push((Reverse(steps.0 + 1), dir, new_pos, comp.clone()));
                    }
                }
            }
            2 => {
                map.insert(pos, Tile::Oxygen);
                oxygen_pos = pos;

                for dir in 1..=4 {
                    let new_pos = new_pos(dir, pos);
                    if !seen.contains(&new_pos) {
                        fringe.push((Reverse(steps.0 + 1), dir, new_pos, comp.clone()));
                    }
                }
            }
            _ => {
                bail!("Computer returned {output}")
            }
        }
    }

    let mut minutes = 0;

    let mut fringe = Vec::new();

    for d in 1..=4 {
        let new_pos = new_pos(d, oxygen_pos);
        if let Some(Tile::Empty) = map.get(&new_pos) {
            fringe.push(new_pos);
        }
    }

    fringe.push(oxygen_pos);

    while !fringe.is_empty() {
        minutes += 1;
        let mut tmp = Vec::new();

        for pos in fringe {
            map.insert(pos, Tile::Oxygen);
            for d in 1..=4 {
                let new_pos = new_pos(d, pos);
                if let Some(Tile::Empty) = map.get(&new_pos) {
                    tmp.push(new_pos);
                }
            }
        }
        fringe = tmp;
    }

    Ok(minutes)
}

fn main() -> Result<(), Error> {
    println!("Part 1: {}", part_01(INPUT)?);
    println!("Part 2: {}", part_02(INPUT)?);
    Ok(())
}
