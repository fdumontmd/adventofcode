use std::collections::HashSet;

use crate::custom_error::AocError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Trench {
    pub top_left: (isize, isize),
    pub bottom_right: (isize, isize),
}

impl Trench {
    pub fn area(&self) -> usize {
        (self.bottom_right.0 - self.top_left.0) as usize
            * (self.bottom_right.1 - self.top_left.1) as usize
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let xmin = self.top_left.0.max(other.top_left.0);
        let ymin = self.top_left.1.max(other.top_left.1);
        let xmax = self.bottom_right.0.min(other.bottom_right.0);
        let ymax = self.bottom_right.1.min(other.bottom_right.1);

        if xmin < xmax && ymin < ymax {
            Some(Trench {
                top_left: (xmin, ymin),
                bottom_right: (xmax, ymax),
            })
        } else {
            None
        }
    }
}

impl Direction {
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self == &Direction::Left || self == &Direction::Right
    }

    pub fn make_trench(&self, pos: (isize, isize), len: isize) -> Trench {
        match self {
            Direction::Up => Trench {
                top_left: (pos.0, pos.1 - len - 1),
                bottom_right: (pos.0 + 1, pos.1 - 1),
            },
            Direction::Down => Trench {
                top_left: (pos.0, pos.1 + 1),
                bottom_right: (pos.0 + 1, pos.1 + len + 1),
            },
            Direction::Left => Trench {
                top_left: (pos.0 - len - 1, pos.1),
                bottom_right: (pos.0 - 1, pos.1 + 1),
            },
            Direction::Right => Trench {
                top_left: (pos.0 + 1, pos.1),
                bottom_right: (pos.0 + len + 1, pos.1 + 1),
            },
        }
    }
}

pub fn parse_instruction_1(input: &str) -> (Direction, isize) {
    let parts: Vec<_> = input.split_whitespace().collect();
    let dir = parts[0];
    let len: isize = parts[1].parse().unwrap();

    let dir = match dir {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("unknown direction {dir} in {input}"),
    };
    (dir, len)
}

pub fn parse_instruction_2(input: &str) -> (Direction, isize) {
    let len = std::str::from_utf8(&input.as_bytes()[input.len() - 7..][..5]).unwrap();
    let len = isize::from_str_radix(len, 16).unwrap();
    let dir = match input.as_bytes()[input.len() - 2] {
        b'0' => Direction::Right,
        b'1' => Direction::Down,
        b'2' => Direction::Left,
        b'3' => Direction::Up,
        _ => panic!("unknown instruction {input}"),
    };
    (dir, len)
}

// only used to validate test and input data; implementation expects
// this check to return true, but does not actually call it.
pub fn sanity_check(input: &str) -> bool {
    let mut pos = (0, 0);
    let orig = Trench {
        top_left: (0, 0),
        bottom_right: (1, 1),
    };
    let mut trenches = vec![orig];
    // ignore the first cubic meter, it will overlap with the last
    // trench anyway
    let mut trench_area = 1;
    // test data and input are closed loops - only one intersection at (0,0)
    let mut prev_dir: Option<Direction> = None;
    for line in input.lines() {
        let (dir, dist) = parse_instruction_2(line);
        let delta = dir.delta();
        let trench = dir.make_trench(pos, dist);
        trench_area += trench.area();
        pos = (pos.0 + delta.0 * dist, pos.1 + delta.1 * dist);
        for i in trenches.iter().filter_map(|o| trench.intersection(o)) {
            trench_area -= i.area();
            if i != orig {
                eprintln!("Not a simple loop going back to (0,0)");
                return false;
            }
            println!("intersection: {i:?}");
        }
        trenches.push(trench);
        if let Some(prev_dir) = prev_dir {
            if prev_dir.is_horizontal() == dir.is_horizontal() {
                eprintln!("trenches do not alternate vertical/horizontal");
                return false;
            }
        }
        prev_dir = Some(dir);
    }
    eprintln!("trench area: {trench_area}");
    true
}
#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let mut pos = (0, 0);
    let mut trench_len = 0;
    let mut area = 0;
    for line in input.lines() {
        let (dir, len) = parse_instruction_2(line);
        trench_len += len;

        let delta = dir.delta();
        pos = (pos.0 + delta.0 * len, pos.1 + delta.1 * len);
        if dir == Direction::Left {
            area += len * pos.1;
        } else if dir == Direction::Right {
            area -= len * pos.1;
        }
    }

    // Using Pick's theorem,
    // we have A = I + B/2 - 1, where we know A and B,
    // so I = A - B/2 + 1, and we need I + B, so
    // I + B = A + B/2 + 1
    Ok(format!("{}", area + trench_len / 2 + 1))
}

#[tracing::instrument]
pub fn process_slow(input: &str) -> Result<String, AocError> {
    /*
    if !sanity_check(input) {
        panic!("sanity check failed");
    }
        */
    // need the complete bounding box (xmin, ymin, xmax, ymax)
    // then iterate over the horizontal trenches, cutting
    // the complete bounding box into inside parts,
    // outside ones. When an inside box meets its closing
    // trench, add its area to the inside area count
    //
    // so need to keep track of: 1) what are the horizontal
    // trenches and 2) what are the inside bounds of the
    // trenches (we're already counting the trench area
    // separately)
    //
    // assumptions: simple loop, alternating vert and horiz,
    let mut pos = (0, 0);
    let mut top_left = pos;
    let mut bottom_right = pos;
    let mut trench_area = 0;
    let mut lines = Vec::new();
    for line in input.lines() {
        let (dir, len) = parse_instruction_2(line);
        trench_area += len;
        let delta = dir.delta();
        let to = (pos.0 + delta.0 * len, pos.1 + delta.1 * len);

        if dir.is_horizontal() {
            let xmin = pos.0.min(to.0);
            let xmax = pos.0.max(to.0);
            lines.push((pos.1, (xmin, xmax)));
        }

        pos = to;

        top_left = (top_left.0.min(pos.0), top_left.1.min(pos.1));
        bottom_right = (bottom_right.0.max(pos.0), bottom_right.1.max(pos.1));
    }

    // sort by y, then x
    lines.sort();
    let mut columns: HashSet<isize> = HashSet::new();

    let mut inside_area = 0;
    // ((from, to), y)
    let mut inside: Vec<((isize, isize), isize)> = Vec::new();
    for line in lines {
        // need to know where the columns are so I can compute the correct inside segment
        let left_wall = columns.contains(&line.1 .0);
        let right_wall = columns.contains(&line.1 .1);
        columns.remove(&line.1 .0);
        columns.remove(&line.1 .1);

        // check for inside segment intersecting the line; there could be more than one
        // for each: compute the intersection, then compute the area for that intersection
        // remove the intersection from the inside; if empty, remove the inside
        // should be enough to check that one end of the line is in the inside
        let intersections: Vec<_> = inside
            .iter()
            .filter(|i| !(line.1 .1 < i.0 .0 || line.1 .0 > i.0 .1))
            .cloned()
            .collect();

        if intersections.is_empty() {
            // we checked for columns above, they'll switch to the other
            // end of the line below, so !left_wall means there's now a column
            // on the left
            let lc = if !left_wall { 1 } else { 0 };
            let rc = if !right_wall { 1 } else { 0 };
            inside.push(((line.1 .0 + lc, line.1 .1 - rc), line.0 + 1));
        } else {
            for intersection in intersections {
                inside.swap_remove(inside.iter().position(|&i| i == intersection).unwrap());
                let inside_range = intersection.0 .0..=intersection.0 .1;
                if inside_range.contains(&line.1 .0) && inside_range.contains(&line.1 .1) {
                    // the whole of the line is below an inside block; add it all to the area
                    inside_area += (line.1 .1 + 1 - line.1 .0) * (line.0 - intersection.1);
                    inside.push(((intersection.0 .0, line.1 .0 - 1), intersection.1));
                    inside.push(((line.1 .1 + 1, intersection.0 .1), intersection.1));
                } else if line.1 .0 <= intersection.0 .0 && line.1 .1 >= intersection.0 .1 {
                    inside_area +=
                        (intersection.0 .1 + 1 - intersection.0 .0) * (line.0 - intersection.1);
                } else {
                    let left = line.1 .0.max(intersection.0 .0);
                    let right = line.1 .1.min(intersection.0 .1);
                    if left > right {
                        panic!("math error!");
                    }
                    let width = right - left + 1;
                    inside_area += width * (line.0 - intersection.1);
                    if inside_range.contains(&line.1 .0) {
                        inside.push(((intersection.0 .0, line.1 .0 - 1), intersection.1));
                    } else {
                        inside.push(((line.1 .1 + 1, intersection.0 .1), intersection.1));
                    }
                }
            }
        }

        // columns always continue on the other end of an horizontal segment
        if !left_wall {
            columns.insert(line.1 .0);
        }
        if !right_wall {
            columns.insert(line.1 .1);
        }
    }

    Ok(format!("{}", trench_area + inside_area))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[rstest]
    #[case(INPUT, "952408144115")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process_slow(input)?);
        Ok(())
    }

    #[rstest]
    #[case(INPUT, "952408144115")]
    #[case(include_str!("../input.txt"), "173152345887206")]
    fn test_process_fast(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
