use aoc_utils::*;
use anyhow::*;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashSet;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 1 {
            match s.as_bytes()[0] {
                b'U' => Ok(Direction::Up),
                b'R' => Ok(Direction::Right),
                b'D' => Ok(Direction::Down),
                b'L' => Ok(Direction::Left),
                _ => Err(anyhow!("Unkown direction {}", s)),
            }
        } else {
            Err(anyhow!("Unknown direction {}", s))
        }
    }
}

struct Segment {
    dir: Direction,
    len: usize,
}

impl FromStr for Segment {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() > 1 {
            let dir = s[0..1].parse::<Direction>()?;
            let len = s[1..].parse::<usize>()?;
            Ok(Segment{dir, len})
        } else {
            Err(anyhow!("Invalid segment {}", s))
        }
    }
}

struct Wire {
    path: Vec<Segment>,
}

impl Wire {
    fn get_points(&self) -> HashSet<(isize, isize)> {
        let mut point = (0, 0);
        let mut points = HashSet::new();

        for segment in &self.path {
            let delta = segment.dir.get_delta();
            for _ in 0..segment.len {
                point.0 += delta.0;
                point.1 += delta.1;
                points.insert(point);
            }
        }

        points
    }

    fn len_to(&self, target: (isize, isize)) -> Option<usize> {
        let mut point = (0, 0);
        let mut len = 0;
        for segment in &self.path {
            let delta = segment.dir.get_delta();
            for _ in 0..segment.len {
                point.0 += delta.0;
                point.1 += delta.1;
                len += 1;
                if point == target {
                    return Some(len);
                }
            }
        }
        None
    }
}

impl FromStr for Wire {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let path: Vec<Segment> = s.split(",").map(str::trim).map(str::parse::<Segment>).collect::<Result<Vec<_>>>().with_context(|| format!("cannot parse wire {}", s))?;
        Ok(Wire{path})
    }
}

struct Circuit {
    wire1: Wire,
    wire2: Wire,
}

impl FromStr for Circuit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let wire1 = lines.next().ok_or(anyhow!("not enough data for wire 1"))?.parse()?;
        let wire2 = lines.next().ok_or(anyhow!("not enough data for wire 2"))?.parse()?;
        Ok(Circuit{wire1, wire2})
    }
}

impl Circuit {
    fn intersection(&self) -> Vec<(isize, isize)> {
        let points1 = self.wire1.get_points();
        let points2 = self.wire2.get_points();

        points1.intersection(&points2).cloned().collect()
    }
    fn compute_min_intersection(&self) -> Option<(isize, isize)> {
        let mut intersect = self.intersection();
        intersect.sort_by(|p1, p2| {
            (p1.0.abs() + p1.1.abs()).cmp(&(p2.0.abs() + p2.1.abs()))
        });
        intersect.get(0).cloned()
    }

    fn compute_len_to(&self, target: (isize, isize)) -> Option<usize> {
        let len1 = self.wire1.len_to(target)?;
        let len2 = self.wire2.len_to(target)?;
        Some(len1 + len2)
    }

    fn best_len_to(&self) -> Option<usize> {
        self.intersection().into_iter().map(|p| self.compute_len_to(p))
            .collect::<Option<Vec<_>>>()
            .map(|mut v| {v.sort(); v})
            .and_then(|v: Vec<usize>| v.into_iter().min())
    }
}

fn main() -> Result<()> {
    let mut buf = String::new();
    get_input().read_to_string(&mut buf)?;
    let circuit = buf.parse::<Circuit>()?;
    let min = circuit.compute_min_intersection();
    println!("part 1: {:?}", min.map(|(x, y)| x.abs() + y.abs()));
    println!("Part 2: {:?}", circuit.best_len_to());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT1: &str = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;

    static INPUT2: &str = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;

    static INPUT3: &str = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;

    #[test]
    fn test_part_1_1() -> Result<()> {
        let circuit: Circuit = INPUT1.parse()?;
        assert_eq!(circuit.compute_min_intersection(), Some((3, 3)));
        Ok(())
    }

    #[test]
    fn test_part_1_2() -> Result<()> {
        let circuit: Circuit = INPUT2.parse()?;
        let min = circuit.compute_min_intersection();
        let min = min.map(|(x, y)| x.abs() + y.abs());
        assert_eq!(min, Some(159));
        Ok(())
    }

    #[test]
    fn test_part_1_3() -> Result<()> {
        let circuit: Circuit = INPUT3.parse()?;
        let min = circuit.compute_min_intersection();
        let min = min.map(|(x, y)| x.abs() + y.abs());
        assert_eq!(min, Some(135));
        Ok(())
    }

    #[test]
    fn test_part2_1() -> Result<()> {
        let circuit: Circuit = INPUT1.parse()?;
        assert_eq!(circuit.best_len_to(), Some(30));
        Ok(())
    }

    #[test]
    fn test_part2_2() -> Result<()> {
        let circuit: Circuit = INPUT2.parse()?;
        assert_eq!(circuit.best_len_to(), Some(610));
        Ok(())
    }

    #[test]
    fn test_part2_3() -> Result<()> {
        let circuit: Circuit = INPUT3.parse()?;
        assert_eq!(circuit.best_len_to(), Some(410));
        Ok(())
    }
}
