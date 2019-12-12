use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
static INPUT: &str = include_str!("input.txt");

lazy_static! {
    static ref REGEX: Regex = Regex::new("<x= ?(-?\\d+), y= ?(-?\\d+), z= ?(-?\\d+)>").unwrap();
}

type Data = isize;

fn gcd(mut a: usize, mut b: usize) -> usize {
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Vector(Data, Data, Data);

impl Default for Vector {
    fn default() -> Self {
        Vector(0, 0, 0)
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "<x={:2}, y={:2}, z={:2}>", self.0, self.1, self.2)
    }
}

impl FromStr for Vector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        REGEX
            .captures(s)
            .map(|cap| {
                Vector(
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                )
            })
            .ok_or(anyhow!("cannot parse {}", s))
    }
}

fn delta(v1: &Vector, v2: &Vector) -> Vector {
    Vector(
        (v2.0 - v1.0).signum(),
        (v2.1 - v1.1).signum(),
        (v2.2 - v1.2).signum(),
    )
}

fn add(v: &Vector, delta: &Vector) -> Vector {
    Vector(v.0 + delta.0, v.1 + delta.1, v.2 + delta.2)
}

fn parse_input(desc: &str) -> Result<Vec<Vector>> {
    desc.lines().map(|line| line.parse::<Vector>()).collect()
}

// count digits required to represent number
fn count_digits(d: Data) -> usize {
    let mut d = d.abs() / 10;
    let mut count = 1;

    while d != 0 {
        count += 1;
        d /= 10;
    }

    count
}

fn count_digit_vector(v: &[Vector]) -> (usize, usize, usize) {
    let xc = v.iter().map(|p| count_digits(p.0)).max().unwrap();
    let yc = v.iter().map(|p| count_digits(p.1)).max().unwrap();
    let zc = v.iter().map(|p| count_digits(p.2)).max().unwrap();
    (xc, yc, zc)
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Planets {
    positions: [Vector; 4],
    velocities: [Vector; 4],
}

impl Planets {
    fn new(desc: &str) -> Result<Self> {
        let mut positions = [Vector::default(); 4];
        positions.copy_from_slice(&parse_input(desc)?);
        Ok(Planets {
            positions,
            velocities: [Vector::default(); 4],
        })
    }

    fn position_digit_counts(&self) -> (usize, usize, usize) {
        count_digit_vector(&self.positions)
    }

    fn velocity_digit_counts(&self) -> (usize, usize, usize) {
        count_digit_vector(&self.velocities)
    }
    fn step(&mut self) {
        let gravities: Vec<Vector> = self
            .positions
            .iter()
            .map(|p| {
                self.positions
                    .iter()
                    .map(|o| delta(p, o))
                    .fold(Vector::default(), |acc, v| add(&acc, &v))
            })
            .collect();
        let velocities: Vec<Vector> = self
            .velocities
            .iter()
            .zip(gravities.iter())
            .map(|(v, g)| add(v, g))
            .collect();
        let positions: Vec<Vector> = self
            .positions
            .iter()
            .zip(velocities.iter())
            .map(|(p, v)| add(p, v))
            .collect();

        self.positions.copy_from_slice(&positions);
        self.velocities.copy_from_slice(&velocities);
    }

    fn total_energy(&self) -> Data {
        self.positions
            .iter()
            .map(|p| p.0.abs() + p.1.abs() + p.2.abs())
            .zip(
                self.velocities
                    .iter()
                    .map(|v| v.0.abs() + v.1.abs() + v.2.abs()),
            )
            .map(|(p, v)| p * v)
            .sum()
    }
}

struct AlignedVector {
    vector: Vector,
    width: (usize, usize, usize),
}

impl AlignedVector {
    fn new(vector: Vector, width: (usize, usize, usize)) -> Self {
        AlignedVector { vector, width }
    }
}

impl Display for AlignedVector {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "<x={:xw$}, y={:yw$}, z={:zw$}>",
            self.vector.0,
            self.vector.1,
            self.vector.2,
            xw = self.width.0 + 1,
            yw = self.width.1 + 1,
            zw = self.width.2 + 1
        )
    }
}

impl Display for Planets {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let pdc = self.position_digit_counts();
        let vdc = self.velocity_digit_counts();
        for (p, v) in self.positions.iter().zip(self.velocities.iter()) {
            write!(
                f,
                "pos={}, vel={}\n",
                AlignedVector::new(*p, pdc),
                AlignedVector::new(*v, vdc)
            )?
        }
        Ok(())
    }
}

fn part_1() -> Data {
    let mut planets = Planets::new(INPUT).unwrap();
    for _ in 0..1000 {
        planets.step();
    }
    planets.total_energy()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate([Data; 4], [Data; 4]);

impl Coordinate {
    fn step(&mut self) {
        let mut gravity: [Data; 4] = [0; 4];
        gravity.iter_mut().zip(self.0.iter()).for_each(|(g, &p)| {
            *g = self.0.iter().map(|o| (o - p).signum()).sum();
        });
        self.1.iter_mut().zip(gravity.iter()).for_each(|(v, &g)| *v += g);
        self.0.iter_mut().zip(self.1.iter()).for_each(|(p, &v)| *p += v);
    }

    fn cycle(&mut self) -> (usize, usize) {
        let mut seen = HashMap::new();
        seen.insert(*self, 0);

        for count in 0.. {
            self.step();

            if let Some(previous) = seen.insert(*self, count + 1) {
                return (previous, count + 1);
            }
        }
        unreachable!()
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate([0; 4], [0; 4])
    }
}

fn planets_cycle(desc: &str) -> usize {
    let planets = Planets::new(desc).unwrap();
    let mut coordinate = Coordinate::default();
    let xp: Vec<_> = planets.positions.iter().map(|&p| p.0).collect();
    coordinate.0.copy_from_slice(&xp);
    let xv: Vec<_> = planets.velocities.iter().map(|&v| v.0).collect();
    coordinate.1.copy_from_slice(&xv);

    let x_cycle = coordinate.cycle();

    let mut coordinate = Coordinate::default();
    let yp: Vec<_> = planets.positions.iter().map(|&p| p.1).collect();
    coordinate.0.copy_from_slice(&yp);
    let yv: Vec<_> = planets.velocities.iter().map(|&v| v.1).collect();
    coordinate.1.copy_from_slice(&yv);

    let y_cycle = coordinate.cycle();

    let mut coordinate = Coordinate::default();
    let zp: Vec<_> = planets.positions.iter().map(|&p| p.2).collect();
    coordinate.0.copy_from_slice(&zp);
    let zv: Vec<_> = planets.velocities.iter().map(|&v| v.2).collect();
    coordinate.1.copy_from_slice(&zv);

    let z_cycle = coordinate.cycle();

    let starts = vec![x_cycle.0, y_cycle.0, z_cycle.0];
    let start = starts.into_iter().max().unwrap();

    // then start + lcm 
    let cd = gcd(x_cycle.1, y_cycle.1);
    let xy = x_cycle.1 / cd * y_cycle.1;

    let cd = gcd(xy, z_cycle.1);
    start + xy / cd * z_cycle.1
}

fn part_2() -> usize {
    planets_cycle(INPUT)
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST: &str = r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#;

    static STEPS: &[&str] = &[
        r#"pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>
pos=<x= 2, y=-10, z=-7>, vel=<x= 0, y= 0, z= 0>
pos=<x= 4, y= -8, z= 8>, vel=<x= 0, y= 0, z= 0>
pos=<x= 3, y=  5, z=-1>, vel=<x= 0, y= 0, z= 0>
"#,
        r#"pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>
"#,
        r#"pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>
pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>
pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>
pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>
"#,
        r#"pos=<x= 5, y=-6, z=-1>, vel=<x= 0, y=-3, z= 0>
pos=<x= 0, y= 0, z= 6>, vel=<x=-1, y= 2, z= 4>
pos=<x= 2, y= 1, z=-5>, vel=<x= 1, y= 5, z=-4>
pos=<x= 1, y=-8, z= 2>, vel=<x= 0, y=-4, z= 0>
"#,
        r#"pos=<x= 2, y=-8, z= 0>, vel=<x=-3, y=-2, z= 1>
pos=<x= 2, y= 1, z= 7>, vel=<x= 2, y= 1, z= 1>
pos=<x= 2, y= 3, z=-6>, vel=<x= 0, y= 2, z=-1>
pos=<x= 2, y=-9, z= 1>, vel=<x= 1, y=-1, z=-1>
"#,
        r#"pos=<x=-1, y=-9, z= 2>, vel=<x=-3, y=-1, z= 2>
pos=<x= 4, y= 1, z= 5>, vel=<x= 2, y= 0, z=-2>
pos=<x= 2, y= 2, z=-4>, vel=<x= 0, y=-1, z= 2>
pos=<x= 3, y=-7, z=-1>, vel=<x= 1, y= 2, z=-2>
"#,
        r#"pos=<x=-1, y=-7, z= 3>, vel=<x= 0, y= 2, z= 1>
pos=<x= 3, y= 0, z= 0>, vel=<x=-1, y=-1, z=-5>
pos=<x= 3, y=-2, z= 1>, vel=<x= 1, y=-4, z= 5>
pos=<x= 3, y=-4, z=-2>, vel=<x= 0, y= 3, z=-1>
"#,
        r#"pos=<x= 2, y=-2, z= 1>, vel=<x= 3, y= 5, z=-2>
pos=<x= 1, y=-4, z=-4>, vel=<x=-2, y=-4, z=-4>
pos=<x= 3, y=-7, z= 5>, vel=<x= 0, y=-5, z= 4>
pos=<x= 2, y= 0, z= 0>, vel=<x=-1, y= 4, z= 2>
"#,
        r#"pos=<x= 5, y= 2, z=-2>, vel=<x= 3, y= 4, z=-3>
pos=<x= 2, y=-7, z=-5>, vel=<x= 1, y=-3, z=-1>
pos=<x= 0, y=-9, z= 6>, vel=<x=-3, y=-2, z= 1>
pos=<x= 1, y= 1, z= 3>, vel=<x=-1, y= 1, z= 3>
"#,
        r#"pos=<x= 5, y= 3, z=-4>, vel=<x= 0, y= 1, z=-2>
pos=<x= 2, y=-9, z=-3>, vel=<x= 0, y=-2, z= 2>
pos=<x= 0, y=-8, z= 4>, vel=<x= 0, y= 1, z=-2>
pos=<x= 1, y= 1, z= 5>, vel=<x= 0, y= 0, z= 2>
"#,
        r#"pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>
"#,
    ];

    #[test]
    fn test_move() -> Result<()> {
        let mut planets = Planets::new(TEST)?;
        assert_eq!(format!("{}", planets), STEPS[0]);
        for i in 1..11 {
            planets.step();
            assert_eq!(format!("{}", planets), STEPS[i]);
        }
        assert_eq!(planets.total_energy(), 179);
        Ok(())
    }

    static LONG_TEST: &str = r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#;

    #[test]
    fn test_basic_cycle() {
        assert_eq!(planets_cycle(TEST), 2772);
    }

    #[test]
    fn test_long_cycle() {
        assert_eq!(planets_cycle(LONG_TEST), 4686774924);
    }
}
