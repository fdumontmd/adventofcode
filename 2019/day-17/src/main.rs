use std::{fmt::Display, ops::Index, str::FromStr};

use anyhow::{bail, Error};
use intcode::Computer;
static INPUT: &str = include_str!("input.txt");

struct View {
    view: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for View {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut view = vec![];
        let mut width = 0;
        let mut computer: Computer = s.parse()?;
        while let Some(o) = computer.wait_until_output() {
            let o: u8 = o.try_into()?;
            if o == 10 {
                if width == 0 {
                    width = view.len();
                }
            } else {
                view.push(o);
            }
        }
        let height = view.len() / width;
        Ok(View {
            view,
            width,
            height,
        })
    }
}

impl Index<(usize, usize)> for View {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.view[x + self.width * y]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Orientation {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Orientation::Up),
            b'v' => Ok(Orientation::Down),
            b'<' => Ok(Orientation::Left),
            b'>' => Ok(Orientation::Right),
            _ => bail!("unknown orientation {value}"),
        }
    }
}

impl Orientation {
    fn delta(&self) -> (isize, isize) {
        match self {
            Orientation::Up => (0, -1),
            Orientation::Down => (0, 1),
            Orientation::Left => (-1, 0),
            Orientation::Right => (1, 0),
        }
    }

    fn angle(&self) -> isize {
        match self {
            Orientation::Up => 90,
            Orientation::Down => 270,
            Orientation::Left => 180,
            Orientation::Right => 0,
        }
    }

    fn orient(&self, to: Orientation) -> Vec<Command> {
        let diff_angle = to.angle() - self.angle();

        let diff_angle = if diff_angle < 0 {
            diff_angle + 360
        } else {
            diff_angle
        };

        match diff_angle {
            0 => vec![],
            90 => vec![Command::Left],
            180 => vec![Command::Left, Command::Left],
            270 => vec![Command::Right],
            _ => panic!("unrecognized angle: {diff_angle}"),
        }
    }

    fn is_opposite(&self, to: Orientation) -> bool {
        match self {
            Orientation::Up => to == Orientation::Down,
            Orientation::Down => to == Orientation::Up,
            Orientation::Left => to == Orientation::Right,
            Orientation::Right => to == Orientation::Left,
        }
    }
}

impl View {
    fn neighbours(
        &self,
        pos: (usize, usize),
    ) -> impl Iterator<Item = (Orientation, (usize, usize))> {
        let width = self.width;
        let height = self.height;
        [
            Orientation::Up,
            Orientation::Down,
            Orientation::Left,
            Orientation::Right,
        ]
        .into_iter()
        .filter_map(move |o| {
            let d = o.delta();
            pos.0
                .checked_add_signed(d.0)
                .and_then(|x| pos.1.checked_add_signed(d.1).map(|y| (o, (x, y))))
        })
        .filter(move |(_, (x, y))| *x < width && *y < height)
    }

    fn forward(&self, pos: (usize, usize), o: Orientation) -> Option<(usize, usize)> {
        let width = self.width;
        let height = self.height;
        let d = o.delta();
        pos.0
            .checked_add_signed(d.0)
            .and_then(|x| pos.1.checked_add_signed(d.1).map(|y| (x, y)))
            .filter(move |(x, y)| *x < width && *y < height)
    }

    fn compute_route(&self) -> Result<Route, Error> {
        let mut route = vec![];
        let pos = self
            .view
            .iter()
            .position(|c| *c != b'.' && *c != b'#')
            .unwrap();
        let mut cur_pos = (pos % self.width, pos / self.width);
        let mut o = Orientation::try_from(self[cur_pos])?;
        for n in self.neighbours(cur_pos) {
            if self[n.1] == b'#' {
                route.append(&mut o.orient(n.0));
                o = n.0;
                break;
            }
        }

        loop {
            // move forward until either None or self[pos] != b'#'
            let mut steps = 0;
            while let Some(p) = self.forward(cur_pos, o) {
                if self[p] != b'#' {
                    break;
                }
                steps += 1;
                cur_pos = p;
            }

            route.push(Command::Forward(steps));

            let mut found_turn = false;

            for n in self.neighbours(cur_pos) {
                if self[n.1] == b'#' && !o.is_opposite(n.0) {
                    found_turn = true;
                    route.append(&mut o.orient(n.0));
                    o = n.0;
                    break;
                }
            }

            if !found_turn {
                break;
            }
        }

        let route = Route(route);

        Ok(route)
    }
}

fn part_01(input: &str) -> Result<usize, Error> {
    let mut aligment_parameters = 0;
    let view: View = input.parse()?;
    for y in 1..view.height - 1 {
        for x in 1..view.width - 1 {
            if view[(x, y)] == b'#'
                && view[(x - 1, y)] == b'#'
                && view[(x + 1, y)] == b'#'
                && view[(x, y - 1)] == b'#'
                && view[(x, y + 1)] == b'#'
            {
                aligment_parameters += x * y;
            }
        }
    }
    Ok(aligment_parameters)
}

impl Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, c) in self.view.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", *c as char)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command {
    Right,
    Left,
    Forward(usize),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Right => write!(f, "R"),
            Command::Left => write!(f, "L"),
            Command::Forward(d) => write!(f, "{d}"),
        }
    }
}

struct Route(Vec<Command>);

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = self.0.first() {
            write!(f, "{c}")?;
        }
        for c in self.0.iter().skip(1) {
            write!(f, ",{c}")?;
        }
        Ok(())
    }
}

fn strip_commas(mut input: &str) -> &str {
    input = if let Some(i) = input.strip_prefix(',') {
        i
    } else {
        input
    };
    if let Some(i) = input.strip_suffix(',') {
        i
    } else {
        input
    }
}

impl Route {
    // return (main, A, B, C)
    fn compress_route(&self) -> (String, String, String, String) {
        // we need a pattern at the begining,
        // one at the end, and the rest
        // for a, scan from start,
        // then split string at a, and do same for b
        // on remaining string
        // if what's left are identical, call that c, and if the prog is
        // compressed down to less than 20 chars, return that
        // complexity due to commas
        let complete = format!("{self}");
        for a in complete
            .split(',')
            .scan(String::new(), |s, c| {
                if !s.is_empty() {
                    *s = format!("{s},{c}");
                } else {
                    *s = c.to_owned();
                }
                Some(s.clone())
            })
            .take_while(|s| s.len() <= 20)
        {
            let fragments: Vec<_> = complete
                .split(&a)
                .map(strip_commas)
                .filter(|f| !f.is_empty())
                .collect();
            // looking for a prefix in the first frament:
            if let Some(first) = fragments.first() {
                for b in first
                    .split(',')
                    .scan(String::new(), |s, c| {
                        if !s.is_empty() {
                            *s = format!("{s},{c}");
                        } else {
                            *s = c.to_owned();
                        }
                        Some(s.clone())
                    })
                    .take_while(|s| s.len() <= 20)
                {
                    let fragments: Vec<_> = fragments
                        .iter()
                        .flat_map(|f| f.split(&b))
                        .map(strip_commas)
                        .filter(|f| !f.is_empty() && f != &",")
                        .collect();
                    if let Some(&c) = fragments.first() {
                        let c = c.to_owned();
                        if fragments
                            .iter()
                            .all(|f| f.split(&c).all(|f| f == "," || f.is_empty()))
                        {
                            let prog = complete.replace(&a, "A").replace(&b, "B").replace(&c, "C");
                            if prog.len() <= 20 {
                                return (prog, a, b, c);
                            }
                        }
                    }
                }
            }
        }
        panic!("compression not found")
    }
}

fn part_02(input: &str) -> Result<i64, Error> {
    let view: View = input.parse()?;
    let route = view.compute_route()?;
    let (prog, a, b, c) = route.compress_route();

    //    let data = format!("{route}");

    // L,6,R,12,L,6,R,12,L,10,L,4,L,6,L,6,R,12,L,6,R,12,L,10,L,4,L,6,L,6,R,12,L,6,L,10,L,10,L,4,L,6,R,12,L,10,L,4,L,6,L,10,L,10,L,4,L,6,L,6,R,12,L,6,L,10,L,10,L,4,L,6

    let mut computer: Computer = input.parse()?;
    // switch to command mode
    computer.set_at(0, 2);
    for b in prog.bytes() {
        computer.add_input(b as i64);
    }
    computer.add_input(10);

    for b in a.bytes() {
        computer.add_input(b as i64);
    }
    computer.add_input(10);

    for b in b.bytes() {
        computer.add_input(b as i64);
    }
    computer.add_input(10);

    for b in c.bytes() {
        computer.add_input(b as i64);
    }
    computer.add_input(10);

    computer.add_input(b'n' as i64);
    computer.add_input(10);

    let mut dust = 0;
    while let Some(o) = computer.wait_until_output() {
        if let Ok(b) = u8::try_from(o) {
            print!("{}", b as char);
        } else {
            dust = o;
            break;
        }
    }

    Ok(dust)
}

fn main() -> Result<(), Error> {
    println!("part 1: {}", part_01(INPUT)?);
    println!("part 2: {}", part_02(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {}
