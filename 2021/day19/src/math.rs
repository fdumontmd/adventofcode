use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub struct Vec3D([i64; 3]);

impl Vec3D {
    pub fn origin() -> Self {
        Self([0, 0, 0])
    }
    pub fn parse(input: &str) -> Self {
        let coords: Vec<_> = input
            .split(',')
            .map(|d| d.parse::<i64>().unwrap())
            .collect();
        Self([coords[0], coords[1], coords[2]])
    }

    pub fn len(&self) -> i64 {
        self.0[0].abs() + self.0[1].abs() + self.0[2].abs()
    }
}

impl Add<&Vec3D> for &Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: &Vec3D) -> Self::Output {
        Vec3D([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl Add<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Vec3D) -> Self::Output {
        &self + &rhs
    }
}

impl Sub<&Vec3D> for &Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: &Vec3D) -> Self::Output {
        Vec3D([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Sub<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Vec3D) -> Self::Output {
        &self - &rhs
    }
}

impl Mul<&Vec3D> for &Matrix3D {
    type Output = Vec3D;

    fn mul(self, rhs: &Vec3D) -> Self::Output {
        Vec3D([
            self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[0] + self.0[4] * rhs.0[1] + self.0[5] * rhs.0[2],
            self.0[6] * rhs.0[0] + self.0[7] * rhs.0[1] + self.0[8] * rhs.0[2],
        ])
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Matrix3D([i64; 9]);

impl Matrix3D {
    pub fn determinant(&self) -> i64 {
        self.0[0] * self.0[4] * self.0[8]
            + self.0[1] * self.0[5] * self.0[6]
            + self.0[2] * self.0[3] * self.0[7]
            - self.0[2] * self.0[4] * self.0[6]
            - self.0[1] * self.0[3] * self.0[8]
            - self.0[0] * self.0[5] * self.0[7]
    }
}

impl Display for Matrix3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub const ROTATIONS: &[Matrix3D] = &[
    Matrix3D([1, 0, 0, 0, 1, 0, 0, 0, 1]),
    Matrix3D([-1, 0, 0, 0, -1, 0, 0, 0, 1]),
    Matrix3D([-1, 0, 0, 0, 1, 0, 0, 0, -1]),
    Matrix3D([1, 0, 0, 0, -1, 0, 0, 0, -1]),
    Matrix3D([-1, 0, 0, 0, 0, -1, 0, -1, 0]),
    Matrix3D([-1, 0, 0, 0, 0, 1, 0, 1, 0]),
    Matrix3D([1, 0, 0, 0, 0, -1, 0, 1, 0]),
    Matrix3D([1, 0, 0, 0, 0, 1, 0, -1, 0]),
    Matrix3D([0, -1, 0, -1, 0, 0, 0, 0, -1]),
    Matrix3D([0, -1, 0, 1, 0, 0, 0, 0, 1]),
    Matrix3D([0, 1, 0, -1, 0, 0, 0, 0, 1]),
    Matrix3D([0, 1, 0, 1, 0, 0, 0, 0, -1]),
    Matrix3D([0, 0, -1, 0, -1, 0, -1, 0, 0]),
    Matrix3D([0, 0, 1, 0, 1, 0, -1, 0, 0]),
    Matrix3D([0, 0, 1, 0, -1, 0, 1, 0, 0]),
    Matrix3D([0, 0, -1, 0, 1, 0, 1, 0, 0]),
    Matrix3D([0, 0, 1, 1, 0, 0, 0, 1, 0]),
    Matrix3D([0, 0, -1, -1, 0, 0, 0, 1, 0]),
    Matrix3D([0, 0, -1, 1, 0, 0, 0, -1, 0]),
    Matrix3D([0, 0, 1, -1, 0, 0, 0, -1, 0]),
    Matrix3D([0, 1, 0, 0, 0, 1, 1, 0, 0]),
    Matrix3D([0, -1, 0, 0, 0, -1, 1, 0, 0]),
    Matrix3D([0, -1, 0, 0, 0, 1, -1, 0, 0]),
    Matrix3D([0, 1, 0, 0, 0, -1, -1, 0, 0]),
];

#[cfg(test)]
mod tests {
    use super::{Vec3D, ROTATIONS};

    #[test]
    fn matrix_sanity_checks() {
        assert!(ROTATIONS.iter().all(|m| dbg!(dbg!(m).determinant()) == 1));
    }

    static TEST_VECTOR_ROTATIONS_DATA: &str = r"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";

    #[test]
    fn test_vector_rotations() {
        let samples: Vec<_> = TEST_VECTOR_ROTATIONS_DATA.split("\n\n").collect();
        let samples: Vec<Vec<Vec3D>> = samples
            .into_iter()
            .map(|s| {
                s.lines()
                    .filter_map(|l| {
                        if l.starts_with("--") {
                            None
                        } else {
                            Some(Vec3D::parse(l))
                        }
                    })
                    .collect()
            })
            .collect();

        let reference = &samples[0];
        for s in &samples[1..] {
            if !ROTATIONS
                .iter()
                .any(|r| s.iter().enumerate().all(|(idx, s)| r * s == reference[idx]))
            {
                panic!("No suitable rotation found")
            }
        }
    }
}
