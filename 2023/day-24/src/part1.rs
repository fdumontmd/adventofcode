use std::ops::RangeInclusive;

use crate::custom_error::AocError;

pub type Vec3d = (f64, f64, f64);
pub type Vec2d = (f64, f64);

#[derive(Debug)]
pub struct Trajectory {
    pub position: Vec3d,
    pub velocity: Vec3d,
}

impl Trajectory {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split('@');
        let pos = parts
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.trim().parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        let vec = parts
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.trim().parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        Trajectory {
            position: (pos[0], pos[1], pos[2]),
            velocity: (vec[0], vec[1], vec[2]),
        }
    }

    pub fn to_standard_2d_form(&self) -> Vec3d {
        let a = self.velocity.1;
        let b = -self.velocity.0;
        let c = self.position.0 * self.velocity.1 - self.position.1 * self.velocity.0;
        //let c = if c < 0.0000001 { 0.0 } else { c };
        (a, b, c)
    }

    pub fn is_in_future_2d(&self, pos: Vec2d) -> bool {
        // pos is guaranteed to be on the path; compute t and check >= 0
        let t1 = (pos.0 - self.position.0) / self.velocity.0;
        let t2 = (pos.1 - self.position.1) / self.velocity.1;
        assert!((t1 - t2).abs() / t1.abs() < 0.0000001);
        t1 >= 0.0
    }

    pub fn solve_for_range_2d(&self, range: &RangeInclusive<f64>) -> Option<RangeInclusive<f64>> {
        let xt0 = (range.start() - self.position.0) / self.velocity.0;
        let xt1 = (range.end() - self.position.0) / self.velocity.0;
        let yt0 = (range.start() - self.position.1) / self.velocity.1;
        let yt1 = (range.end() - self.position.1) / self.velocity.1;

        let t0 = xt0.max(yt0);
        let t1 = xt1.min(yt1);

        if t0 < t1 {
            Some(t0..=t1)
        } else {
            None
        }
    }
}

pub enum Intersection {
    Point(Vec2d),
    Line,
    None,
}

pub fn intersection_2d(l1: Vec3d, l2: Vec3d) -> Intersection {
    if l1.1 != 0.0 && l2.1 != 0.0 {
        // B is not zero
        // rewrite as mx=b
        let m = l1.0 / l1.1 - l2.0 / l2.1;
        let b = l1.2 / l1.1 - l2.2 / l2.1;
        if m == 0.0 && b == 0.0 {
            Intersection::Line
        } else if m == 0.0 || b == 0.0 {
            Intersection::None
        } else {
            let x = b / m;
            let y1 = (l1.2 - l1.0 * x) / l1.1;
            let y2 = (l2.2 - l2.0 * x) / l2.1;
            assert!((y1 - y2).abs() / y1.abs() < 0.0000001);
            Intersection::Point((x, y1))
        }
    } else if l1.0 != 0.0 && l2.0 != 0.0 {
        // A is not zero
        // rewrite as my=b
        let m = l1.1 / l1.0 - l2.1 / l2.0;
        let b = l1.2 / l1.0 - l2.2 / l2.0;
        if m == 0.0 && b == 0.0 {
            Intersection::Line
        } else if m == 0.0 || b == 0.0 {
            Intersection::None
        } else {
            let y = b / m;
            let x1 = (l1.2 - l1.1 * y) / l1.0;
            let x2 = (l2.2 - l2.1 * y) / l2.0;
            assert!((x1 - x2).abs() / x1.abs() < 0.0000001);
            Intersection::Point((x1, y))
        }
    } else {
        Intersection::None
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let count = process_in_range(input, 200000000000000f64..=400000000000000f64);
    Ok(format!("{count}"))
}

pub fn process_in_range(input: &str, range: RangeInclusive<f64>) -> usize {
    let trajectories: Vec<Trajectory> = input.lines().map(Trajectory::parse).collect();
    let mut count = 0;
    for (idx, h1) in trajectories.iter().enumerate() {
        for h2 in &trajectories[idx + 1..] {
            match intersection_2d(h1.to_standard_2d_form(), h2.to_standard_2d_form()) {
                Intersection::Point((x, y)) => {
                    if range.contains(&x)
                        && range.contains(&y)
                        && h1.is_in_future_2d((x, y))
                        && h2.is_in_future_2d((x, y))
                    {
                        count += 1;
                    }
                }
                Intersection::Line => {
                    let Some(r1) = h1.solve_for_range_2d(&range) else {
                        continue;
                    };
                    let Some(r2) = h2.solve_for_range_2d(&range) else {
                        continue;
                    };
                    let rmin = r1.start().max(*r2.start());
                    let rmax = r2.end().min(*r2.end());
                    if rmin < rmax && rmin >= 0.0 {
                        count += 1;
                    }
                }
                Intersection::None => {}
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    /// .
    #[rstest]
    #[case(INPUT, 2, 7f64, 27f64)]
    #[case(include_str!("../input.txt"), 17906, 200000000000000f64, 400000000000000f64)]
    fn test_process(#[case] input: &str, #[case] count: usize, #[case] from: f64, #[case] to: f64) {
        assert_eq!(count, process_in_range(input, from..=to));
    }
}
