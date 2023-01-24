#![feature(binary_heap_retain)]
#![feature(once_cell)]
// needs nightly compiler
use anyhow::bail;
use std::{
    cell::OnceCell,
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
    fmt::Display,
};

static INPUT: &[u8] = include_bytes!("input.txt");

type Position = (isize, isize);
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Door(Position, Position);

impl Door {
    fn new(from: Position, to: Position) -> Self {
        Self(from.min(to), from.max(to))
    }
}

fn move_to(from: Position, mvt: u8) -> Position {
    match mvt {
        b'N' => (from.0, from.1 - 1),
        b'S' => (from.0, from.1 + 1),
        b'E' => (from.0 + 1, from.1),
        b'W' => (from.0 - 1, from.1),
        _ => unreachable!(),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Token {
    Direction(u8),
    OpenParen,
    CloseParen,
    Pipe,
    Start,
    End,
    Unknown(u8),
    Eof,
}

impl Token {
    fn from_char(c: u8) -> Self {
        match c {
            b'N' | b'S' | b'W' | b'E' => Token::Direction(c),
            b'(' => Token::OpenParen,
            b')' => Token::CloseParen,
            b'|' => Token::Pipe,
            b'^' => Token::Start,
            b'$' => Token::End,
            _ => Token::Unknown(c),
        }
    }
}

struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a [u8]) -> Self {
        Lexer { input, pos: 0 }
    }

    fn peek(&self) -> Token {
        self.input
            .get(self.pos)
            .cloned()
            .map(Token::from_char)
            .unwrap_or(Token::Eof)
    }

    fn next(&mut self) -> Token {
        let t = self.peek();
        self.pos += 1;
        t
    }
}

#[derive(Debug)]
enum Path {
    Direction(u8),
    Sequence(Vec<Path>),
    Branches(Vec<Path>),
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Direction(d) => write!(f, "{}", *d as char),
            Path::Sequence(s) => {
                for p in s {
                    write!(f, "{p}")?;
                }
                Ok(())
            }
            Path::Branches(b) => {
                let mut first = true;
                write!(f, "(")?;
                for p in b {
                    if !first {
                        write!(f, "|")?;
                    }
                    write!(f, "{p}")?;
                    first = false;
                }
                write!(f, ")")
            }
        }
    }
}

impl Path {
    // we start with positions = {(0, 0)}
    // then, map self:
    // Direction(d): for each p in positions, add a door::new(p, move_to(p, d))
    // Segment: collect all doors, and check positions at end, and merge
    // Branching: pass current positions to each branch, collect all doors, and merge positions at
    // end
    // -> fn must take a mut set of doors, take input a set of positions, and return a set of
    // positions
    fn build_map(&self) -> Map {
        let mut doors = BTreeSet::new();
        let mut positions = BTreeSet::new();
        positions.insert((0, 0));

        self.build_map_inner(&positions, &mut doors);

        Map {
            doors,
            dists: OnceCell::new(),
        }
    }

    fn build_map_inner(
        &self,
        positions: &BTreeSet<Position>,
        doors: &mut BTreeSet<Door>,
    ) -> BTreeSet<Position> {
        match self {
            Path::Direction(d) => {
                let mut new_positions = BTreeSet::new();
                for p in positions {
                    let np = move_to(*p, *d);
                    doors.insert(Door::new(*p, np));
                    new_positions.insert(np);
                }
                new_positions
            }
            Path::Sequence(s) => {
                let mut new_positions = BTreeSet::from_iter(positions.iter().cloned());
                for p in s {
                    new_positions = p.build_map_inner(&new_positions, doors);
                }
                new_positions
            }
            Path::Branches(b) => {
                let mut new_positions = BTreeSet::new();
                for p in b {
                    new_positions.append(&mut p.build_map_inner(positions, doors));
                }
                new_positions
            }
        }
    }
}

struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    fn tag(&mut self, t: Token) -> Result<(), anyhow::Error> {
        let actual = self.lexer.next();
        if t != actual {
            bail!("Expected {:?}, got {:?}", t, actual);
        }
        Ok(())
    }

    fn validate(&mut self) -> Result<Path, anyhow::Error> {
        self.tag(Token::Start)?;

        let sequence = self.validate_sequence()?;

        self.tag(Token::End)?;
        Ok(Path::Sequence(sequence))
    }

    fn validate_sequence(&mut self) -> Result<Vec<Path>, anyhow::Error> {
        let mut sequence = Vec::new();
        loop {
            let t = self.lexer.peek();

            if t == Token::OpenParen {
                self.tag(Token::OpenParen)?;

                let branches = self.validate_branches()?;
                sequence.push(Path::Branches(branches));

                self.tag(Token::CloseParen)?;
            } else if let Token::Direction(d) = t {
                self.lexer.next();
                sequence.push(Path::Direction(d))
            } else {
                return Ok(sequence);
            }
        }
    }

    fn validate_branches(&mut self) -> Result<Vec<Path>, anyhow::Error> {
        let mut branches = Vec::new();
        loop {
            let sequence = self.validate_sequence()?;
            branches.push(Path::Sequence(sequence));
            let t = self.lexer.peek();
            if t != Token::Pipe {
                return Ok(branches);
            }
            self.lexer.next();
        }
    }
}

#[derive(Debug)]
struct Map {
    doors: BTreeSet<Door>,
    dists: OnceCell<Vec<usize>>,
}

impl Map {
    fn build_graph(&self) -> Vec<usize> {
        let rooms = BTreeSet::from_iter(self.doors.iter().flat_map(|d| [d.0, d.1].into_iter()));

        let rooms: Vec<Position> = Vec::from_iter(rooms.into_iter());

        let mut dist = vec![usize::MAX; rooms.len()];

        let init = rooms.binary_search(&(0, 0)).unwrap();
        dist[init] = 0;

        let mut q = BinaryHeap::new();
        for r in 0..rooms.len() {
            q.push((Reverse(dist[r]), r));
        }

        while let Some(u) = q.pop() {
            let u = u.1;
            for n in [b'N', b'S', b'W', b'E'].into_iter() {
                let neighbour = move_to(rooms[u], n);
                if self.doors.contains(&Door::new(rooms[u], neighbour)) {
                    let v = rooms.binary_search(&neighbour).unwrap();
                    let alt = dist[u] + 1;
                    if alt < dist[v] {
                        dist[v] = alt;
                        q.retain(|e| e.1 != v);
                        q.push((Reverse(dist[v]), v));
                        // ?? how to update q's priority for v?
                    }
                }
            }
        }

        dist
    }

    fn max_dist(&self) -> usize {
        let dists = self.dists.get_or_init(|| self.build_graph());
        dists.iter().cloned().max().unwrap_or(usize::MAX)
    }

    fn door_count(&self, min_dist: usize) -> usize {
        let dists = self.dists.get_or_init(|| self.build_graph());
        dists.iter().cloned().filter(|d| *d >= min_dist).count()
    }
}

fn main() {
    let mut p = Parser::new(INPUT);
    let p = p.validate().unwrap();

    let map = p.build_map();
    println!("part 1: {}", map.max_dist());
    println!("part 2: {}", map.door_count(1000));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{Parser, INPUT};

    #[test_case(b"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18, None)]
    #[test_case(b"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23, None)]
    #[test_case(
        b"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
        31,
        None
    )]
    #[test_case(INPUT, 3476, Some(8514))]
    fn test_both(input: &[u8], dist: usize, door_count: Option<usize>) {
        let mut p = Parser::new(input);
        let p = p.validate().unwrap();

        let map = p.build_map();
        assert_eq!(dist, map.max_dist());

        if let Some(door_count) = door_count {
            assert_eq!(door_count, map.door_count(1000));
        }
    }
}
