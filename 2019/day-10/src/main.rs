use std::cmp::{Ordering};
use std::collections::{HashSet, BTreeMap};

static INPUT: &str = include_str!("input.txt");

type Angle = (isize, isize);

fn gcd(a: isize, b: isize) -> isize {
    let mut a = a.abs();
    let mut b = b.abs();

    if a < b {
        std::mem::swap(&mut a, &mut b);
    }

    loop {
        if b == 0 {
            return a;
        }

        let tmp = a % b;
        a = b;
        b = tmp;
    }
}

fn compute_angle_and_dist(base: (usize, usize), target: (usize, usize)) -> (Angle, Float) {
    let angle = (target.0 as isize - base.0 as isize, target.1 as isize - base.1 as isize);
    match angle {
        (0, 0) => ((0, 0), Float(0f32)),
        (a, b) => {
            let div = gcd(a, b);
            ((a/div, b/div), Float((a.abs() as f32).hypot(b.abs() as f32)))
        }
    }
}

fn count_observables(desc: &Vec<&[u8]>, base: (usize, usize)) -> usize {
    let mut angles = HashSet::new();
    for col in 0..desc.len() {
        for row in 0..desc[col].len() {
            if desc[col][row] == b'#' {
                if (row, col) != base {
                    angles.insert(compute_angle_and_dist(base, (row, col)).0);
                }
            }
        }
    }
    angles.len()
}

fn compute_all_observables(desc: &Vec<&[u8]>) -> Vec<(usize, (usize, usize))> {
    let mut bases = Vec::new();
    for col in 0..desc.len() {
        for row in 0..desc[col].len() {
            if desc[col][row] == b'#' {
                let base = (row, col);
                bases.push((count_observables(desc, base), base));
            }
        }
    }
    bases
}

fn find_best(desc: &str) -> (usize ,(usize, usize)) {
    let desc: Vec<&[u8]> = desc.lines().map(|l| l.as_bytes()).collect();

    let bases = compute_all_observables(&desc);

    *bases.iter().max().unwrap()
}

fn angle_to_gradiant(a: Angle) -> Float {
    // swap x and y axis so that angle that go clockwise from vertical
    // are mapped to angle that go counter clockwise from horizontal
    let g = (a.1 as f32).atan2(a.0 as f32) + std::f32::consts::FRAC_PI_2;
    let g = if g < 0f32 {
            g + std::f32::consts::PI * 2f32
        } else {
            g
        };

    let g = if g > 2f32 * std::f32::consts::PI {
        g - 2f32 * std::f32::consts::PI 
    } else {
        g
    };
    Float(g)
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
struct Float(f32);

impl Eq for Float {
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn map_all_targets(desc: &Vec<&[u8]>, base: (usize, usize)) -> Vec<Vec<(usize, usize)>>{
    let mut all_targets = BTreeMap::new();
    for col in 0..desc.len() {
        for row in 0..desc[col].len() {
            if desc[col][row] == b'#' && (row, col) != base {
                let target = (row, col);
                let (angle, dist) = compute_angle_and_dist(base, target);
                let gradiant = angle_to_gradiant(angle);
                all_targets.entry(gradiant).or_insert(Vec::new()).push((dist, target));
            }
        }
    }

    let mut targets = Vec::new();
    for (_k, mut v) in all_targets {
        v.sort();
        v.reverse();
        targets.push(v.into_iter().map(|t| t.1).collect());
    }

    targets
}

struct Targets {
    targets: Vec<Vec<(usize, usize)>>,
    pos: usize,
    empty: bool,
}

impl Targets {
    fn new(targets: Vec<Vec<(usize, usize)>>) -> Self {
        Targets {
            targets,
            pos: 0,
            empty: false,
        }
    }

    fn move_pos_to_next(&mut self) {
        for _ in 0..self.targets.len() {
            self.pos += 1;
            self.pos = self.pos % self.targets.len();
            if !self.targets[self.pos].is_empty() {
                break;
            }
        }

        if self.targets[self.pos].is_empty() {
            self.empty = true;
        }
    }
}

impl Iterator for Targets {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.empty {
            let t = self.targets[self.pos].pop();
            self.move_pos_to_next();
            t
        } else {
            None
        }

    }
}

fn get_target_iterator(desc: &str, base: (usize, usize)) -> Targets {
    let desc: Vec<&[u8]> = desc.lines().map(|l| l.as_bytes()).collect();

    Targets::new(map_all_targets(&desc, base))
}

fn part_1() -> usize {
    find_best(INPUT).0
}

fn part_2() -> usize {
    let mut iter = get_target_iterator(INPUT, find_best(INPUT).1);
    iter.nth(199).map(|(r, c)| r*100+c).unwrap()
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_1: &str = r#".#..#
.....
#####
....#
...##"#;

    #[test]
    fn test_3_4() {
        assert_eq!(find_best(TEST_1), (8, (3, 4)));
    }

    static TEST_2: &str = r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#;

    #[test]
    fn test_5_8() {
        assert_eq!(find_best(TEST_2), (33, (5, 8)));
    }

    static TEST_3: &str = r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#;

    #[test]
    fn test_1_2() {
        assert_eq!(find_best(TEST_3), (35, (1, 2)));
    }

    static TEST_4: &str = r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#;

    #[test]
    fn test_6_3() {
        assert_eq!(find_best(TEST_4), (41, (6, 3)));
    }

    static TEST_5: &str = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;

    #[test]
    fn test_11_13() {
        assert_eq!(find_best(TEST_5), (210, (11, 13)));
    }

    #[test]
    fn test_200_target() {

        let targets: Vec<_> = get_target_iterator(TEST_5, (11, 13)).collect();

        assert_eq!(targets.len(), 299);
        assert_eq!(targets[0], (11,12));
        assert_eq!(targets[1], (12,1));
        assert_eq!(targets[2], (12,2));
        assert_eq!(targets[9], (12,8));
        assert_eq!(targets[19], (16,0));
        assert_eq!(targets[49], (16,9));
        assert_eq!(targets[99], (10,16));
        assert_eq!(targets[199], (8,2));
        assert_eq!(targets[200], (10,9));
        assert_eq!(targets[298], (11,1));
    }
}
