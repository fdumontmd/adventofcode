use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    fmt::Display,
};

static INPUT: &str = include_str!("input.txt");

// build a map of the valley with each blizzard starting point and their
// orientation
// then it is easy to compute the position of all blizzards at all future
// time by using % width or height of the valley
// from there, do a BFS with a priority queue optimized for distance to
// exit

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Blizzard {
    pos: (usize, usize),
    direction: Direction,
}

impl Blizzard {
    fn try_new(start: (usize, usize), b: u8) -> Option<Self> {
        match b {
            b'^' => Some(Blizzard {
                pos: start,
                direction: Direction::Up,
            }),
            b'v' => Some(Blizzard {
                pos: start,
                direction: Direction::Down,
            }),
            b'<' => Some(Blizzard {
                pos: start,
                direction: Direction::Left,
            }),
            b'>' => Some(Blizzard {
                pos: start,
                direction: Direction::Right,
            }),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Valley {
    blizzards: Vec<Blizzard>,
    blizzard_pos: BTreeSet<(usize, usize)>,
    walls: BTreeSet<(usize, usize)>,
    // inner widht and height
    width: usize,
    height: usize,
    // really for debugging purposes
    party_pos: (usize, usize),
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blizzard_count: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        for b in &self.blizzards {
            *blizzard_count.entry(b.pos).or_default() += 1;
        }
        let blizzards: BTreeMap<(usize, usize), char> =
            BTreeMap::from_iter(self.blizzards.iter().map(|b| (b.pos, b.as_char())));

        for row in 0..self.height + 2 {
            for col in 0..self.width + 2 {
                if self.party_pos == (col, row) {
                    write!(f, "E")?;
                } else if self.walls.contains(&(col, row)) {
                    write!(f, "#")?;
                } else if let Some(b) = blizzards.get(&(col, row)) {
                    let Some(c) = blizzard_count.get(&(col, row)) else { panic!("blizzard not counted?")};
                    if *c == 1 {
                        write!(f, "{}", b)?;
                    } else {
                        write!(f, "{}", c)?;
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Valley {
    fn parse(input: &str) -> Self {
        let walls: BTreeSet<(usize, usize)> =
            input
                .lines()
                .filter(|l| !l.trim().is_empty())
                .enumerate()
                .flat_map(|(row, l)| {
                    l.bytes().enumerate().filter_map(move |(col, b)| {
                        if b == b'#' {
                            Some((col, row))
                        } else {
                            None
                        }
                    })
                })
                .collect();
        let mut blizzards: Vec<Blizzard> = input
            .lines()
            .filter(|l| !l.trim().is_empty())
            .enumerate()
            .flat_map(|(row, l)| {
                l.bytes()
                    .enumerate()
                    .filter_map(move |(col, b)| Blizzard::try_new((col, row), b))
            })
            .collect();
        blizzards.sort();

        let width = walls.last().unwrap().0 - 1;
        let height = walls.last().unwrap().1 - 1;
        let blizzard_pos = BTreeSet::from_iter(blizzards.iter().map(|b| b.pos));

        Self {
            blizzards,
            blizzard_pos,
            walls,
            height,
            width,
            party_pos: (1, 0),
        }
    }

    // almost there. need some modulo arithmetic that can handle this more robustly
    fn at_time(&self, minute: usize) -> Self {
        let minute = minute as isize;
        let mut blizzards: Vec<Blizzard> = self
            .blizzards
            .iter()
            .map(|b| Blizzard {
                pos: (
                    add_signed_modulo(b.pos.0 - 1, minute * b.delta().0, self.width) + 1,
                    add_signed_modulo(b.pos.1 - 1, minute * b.delta().1, self.height) + 1,
                ),
                ..*b
            })
            .collect();
        blizzards.sort();
        let blizzard_pos = BTreeSet::from_iter(blizzards.iter().map(|b| b.pos));
        Self {
            blizzards,
            blizzard_pos,
            ..self.clone()
        }
    }

    fn is_wall(&self, pos: (usize, usize)) -> bool {
        self.walls.contains(&pos)
    }
    fn is_blizzard(&self, pos: (usize, usize)) -> bool {
        self.blizzard_pos.contains(&pos)
    }
}

fn add_signed_modulo(x: usize, delta: isize, modulo: usize) -> usize {
    (x + modulo)
        .checked_add_signed(delta % modulo as isize)
        .unwrap()
        % modulo
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct SearchState {
    minute: Reverse<usize>,
    //distance: Reverse<usize>,
    pos: (usize, usize),
}

impl SearchState {
    fn new(pos: (usize, usize)) -> Self {
        SearchState {
            minute: Reverse(0),
            //distance: Reverse(usize::MIN),
            pos,
        }
    }

    fn next_state(&self, delta: (isize, isize), valley: &Valley) -> Option<Self> {
        let new_pos = (
            self.pos.0.checked_add_signed(delta.0),
            self.pos.1.checked_add_signed(delta.1),
        );

        if new_pos.0.is_none()
            || new_pos.1.is_none()
            || new_pos.0.unwrap() >= valley.width + 2
            || new_pos.1.unwrap() >= valley.height + 2
        {
            None
        } else {
            let new_pos = (new_pos.0.unwrap(), new_pos.1.unwrap());
            if valley.is_wall(new_pos) || valley.is_blizzard(new_pos) {
                None
            } else {
                Some(SearchState {
                    minute: Reverse(self.minute.0 + 1),
                    //distance: Reverse(target.0.abs_diff(new_pos.0) + target.1.abs_diff(new_pos.1)),
                    pos: new_pos,
                })
            }
        }
    }
}

fn search_exit(valley: &Valley, start_pos: (usize, usize), end_pos: (usize, usize)) -> usize {
    let init = SearchState::new(start_pos);
    let mut queue = BinaryHeap::new();
    queue.push(init);
    let mut valleys = vec![valley.at_time(0)];

    let mut seen_time = 0;
    let mut seen = BTreeSet::new();

    while let Some(state) = queue.pop() {
        if seen_time != state.minute.0 {
            seen.clear();
        }
        seen_time = state.minute.0;
        while valleys.len() <= state.minute.0 + 1 {
            valleys.push(valley.at_time(valleys.len()));
        }
        if seen.contains(&state.pos) {
            continue;
        }
        seen.insert(state.pos);
        if state.pos == end_pos {
            return state.minute.0;
        }

        queue.extend(
            [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)]
                .into_iter()
                .flat_map(|d| state.next_state(d, &valleys[state.minute.0 + 1])),
        );
    }

    0
}

fn part_01(input: &str) -> usize {
    let valley = Valley::parse(input);
    let start_pos = (1, 0);
    let end_pos = (valley.width, valley.height + 1);
    search_exit(&valley, start_pos, end_pos)
}

fn part_02(input: &str) -> usize {
    let valley = Valley::parse(input);
    let start_pos = (1, 0);
    let end_pos = (valley.width, valley.height + 1);
    let first_leg = search_exit(&valley, start_pos, end_pos);
    dbg!(first_leg);
    let valley = valley.at_time(first_leg);
    let second_leg = search_exit(&valley, end_pos, start_pos);
    dbg!(second_leg);
    let valley = valley.at_time(second_leg);
    let third_leg = search_exit(&valley, start_pos, end_pos);
    dbg!(third_leg);
    first_leg + second_leg + third_leg
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02};

    static TEST_INPUT: &str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part_01() {
        assert_eq!(18, part_01(TEST_INPUT));
    }

    #[test]
    fn test_part_02() {
        assert_eq!(54, part_02(TEST_INPUT));
    }
}
