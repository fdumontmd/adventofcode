use std::{
    collections::{BTreeSet, HashMap},
    ops::{Add, Mul, Sub},
};

const INPUT: &str = include_str!("input.txt");

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct Vec3(i64, i64, i64);

impl Vec3 {
    fn magnitude(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    fn cos(&self, rhs: &Vec3) -> f64 {
        let d = self.0 as f64 * rhs.0 as f64
            + self.1 as f64 * rhs.1 as f64
            + self.2 as f64 * rhs.2 as f64;
        d / (self.magnitude() as f64 * rhs.magnitude() as f64)
    }
}

impl TryFrom<&str> for Vec3 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(v) = value.strip_prefix("<") {
            if let Some(v) = v.strip_suffix(">") {
                let coords: Vec<_> = v.split(',').collect();
                let x: i64 = coords[0]
                    .trim()
                    .parse()
                    .map_err(|e| format!("cannot parse {value} as Vec3: {e}"))?;
                let y: i64 = coords[1]
                    .trim()
                    .parse()
                    .map_err(|e| format!("cannot parse {value} as Vec3: {e}"))?;
                let z: i64 = coords[2]
                    .trim()
                    .parse()
                    .map_err(|e| format!("cannot parse {value} as Vec3: {e}"))?;
                return Ok(Vec3(x, y, z));
            }
        }
        Err(format!("cannot parse {value} as Vec3"))
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

// I'm overthinking this, aren't I?
enum Solutions {
    None,
    One(i64),
    Two(i64, i64),
    Any,
}

impl Solutions {
    fn merge(self, other: Solutions) -> Solutions {
        match self {
            Solutions::None => Solutions::None,
            Solutions::One(i) => match other {
                Solutions::None => Solutions::None,
                Solutions::One(i1) => {
                    if i == i1 {
                        self
                    } else {
                        Solutions::None
                    }
                }
                Solutions::Two(i1, i2) => {
                    if i == i1 || i == i2 {
                        self
                    } else {
                        Solutions::None
                    }
                }
                Solutions::Any => self,
            },
            Solutions::Two(i1, i2) => match other {
                Solutions::None => other,
                Solutions::One(i) => {
                    if i == i1 || i == i2 {
                        other
                    } else {
                        Solutions::None
                    }
                }
                Solutions::Two(o1, o2) => {
                    let mut sols = vec![];

                    if i1 == o1 || i1 == o2 {
                        sols.push(i1);
                    }

                    if i2 == o1 || i2 == o2 {
                        sols.push(i2);
                    }

                    match sols.len() {
                        0 => Solutions::None,
                        1 => Solutions::One(sols[0]),
                        _ => Solutions::Two(sols[0], sols[1]),
                    }
                }
                Solutions::Any => self,
            },
            Solutions::Any => other,
        }
    }
}

fn solve_quad(a: i64, b: i64, c: i64) -> Solutions {
    if a == 0 {
        if b == 0 {
            if c == 0 {
                Solutions::Any
            } else {
                Solutions::None
            }
        } else if c % b == 0 {
            Solutions::One(-c / b)
        } else {
            Solutions::None
        }
    } else {
        let delta = b * b - 4 * a * c;
        if delta >= 0 && delta == delta.isqrt().pow(2) {
            let r = delta.isqrt();
            let mut sols = vec![];

            if r == 0 {
                if b % (2 * a) == 0 {
                    sols.push(-b / (2 * a));
                }
            } else {
                let x1 = -b - r;
                if x1 % (2 * a) == 0 {
                    sols.push(x1 / (2 * a));
                }
                let x2 = -b + r;
                if x2 % (2 * a) == 0 {
                    sols.push(x2 / (2 * a));
                }
            }

            sols.sort();
            sols.retain(|i| *i >= 0);

            match sols.len() {
                0 => Solutions::None,
                1 => Solutions::One(sols[0]),
                _ => Solutions::Two(sols[0], sols[1]),
            }
        } else {
            Solutions::None
        }
    }
}

impl Particle {
    fn position_at_time(&self, t: i64) -> Vec3 {
        self.position + self.velocity * t + self.acceleration * ((t * (t + 1)) / 2)
    }

    fn at_time(&self, t: i64) -> Self {
        let velocity = self.velocity * t + self.acceleration * ((t * (t + 1)) / 2);
        Self {
            position: self.position + velocity,
            velocity,
            acceleration: self.acceleration,
        }
    }

    fn moving_away_from_origin(&self) -> bool {
        (self.velocity.magnitude() == 0 || self.position.cos(&self.velocity) > 0f64)
            && (self.acceleration.magnitude() == 0 || self.position.cos(&self.acceleration) > 0f64)
    }

    fn intersect_time(&self, other: &Particle) -> Vec<i64> {
        let a = self.acceleration - other.acceleration;
        let b = self.velocity * 2 + self.acceleration - other.velocity * 2 - other.acceleration;
        let c = self.position * 2 - other.position * 2;

        let solutions = solve_quad(a.0, b.0, c.0)
            .merge(solve_quad(a.1, b.1, c.1))
            .merge(solve_quad(a.2, b.2, c.2));

        match solutions {
            Solutions::None => vec![],
            Solutions::One(i) => vec![i],
            Solutions::Two(i1, i2) => vec![i1, i2],
            Solutions::Any => vec![0],
        }
    }
}

impl TryFrom<&str> for Particle {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<_> = value.split(", ").collect();
        let position = if let Some(v) = parts[0].strip_prefix("p=") {
            Vec3::try_from(v).map_err(|e| format!("cannot parse {value} as Particle: {e}"))?
        } else {
            return Err(format!("cannot parse {value} as Particle"));
        };
        let velocity = if let Some(v) = parts[1].strip_prefix("v=") {
            Vec3::try_from(v).map_err(|e| format!("cannot parse {value} as Particle: {e}"))?
        } else {
            return Err(format!("cannot parse {value} as Particle"));
        };
        let acceleration = if let Some(v) = parts[2].strip_prefix("a=") {
            Vec3::try_from(v).map_err(|e| format!("cannot parse {value} as Particle: {e}"))?
        } else {
            return Err(format!("cannot parse {value} as Particle"));
        };
        Ok(Particle {
            position,
            velocity,
            acceleration,
        })
    }
}

fn part1(input: &str) -> usize {
    const STEP: i64 = 1000000;
    let mut particles: Vec<Particle> = input
        .lines()
        .map(|l| Particle::try_from(l).unwrap())
        .collect();

    for _ in 0.. {
        if particles.iter().all(Particle::moving_away_from_origin) {
            return particles
                .iter()
                .enumerate()
                .min_by_key(|(_, p)| p.position.magnitude())
                .unwrap()
                .0;
        }
        particles = particles.into_iter().map(|p| p.at_time(STEP)).collect();
    }
    unreachable!()
}

fn part2(input: &str) -> usize {
    let mut particles: Vec<Particle> = input
        .lines()
        .map(|l| Particle::try_from(l).unwrap())
        .collect();

    // question is: what test can we have that proves particles can no longer hit each other?
    // particle position is a 2nd degree polynomial of time: x = x0 + t * v0 + (t*(t+1)) * a / 2
    // so any pair of particles has at most 2 possible intersections
    let mut intersections = BTreeSet::new();
    for x1 in 0..particles.len() {
        for x2 in x1 + 1..particles.len() {
            intersections.extend(particles[x1].intersect_time(&particles[x2]));
        }
    }

    for i in intersections {
        let mut hits: HashMap<Vec3, usize> = HashMap::new();
        for p in &particles {
            *hits.entry(p.position_at_time(i)).or_default() += 1;
        }
        particles.retain(|p| hits.get(&p.position_at_time(i)).cloned().unwrap() == 1);
    }

    particles.len()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT_1: &str = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>
";

    #[test_case(TEST_INPUT_1, 0)]
    #[test_case(INPUT, 157)]
    fn test_part1(input: &str, idx: usize) {
        assert_eq!(idx, part1(input));
    }

    const TEST_INPUT_2: &str = "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
";

    #[test_case(TEST_INPUT_2, 1)]
    #[test_case(INPUT, 499)]
    fn test_part2(input: &str, count: usize) {
        assert_eq!(count, part2(input));
    }
}
