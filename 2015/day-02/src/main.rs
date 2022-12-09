use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");

type Size = u32;

struct Box {
    l: Size,
    w: Size,
    h: Size,
}

impl Box {
    fn new(l: Size, w: Size, h: Size) -> Box {
        Box { l: l, w: w, h: h }
    }

    fn area(&self) -> Size {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn two_smallest_sides(&self) -> (Size, Size) {
        let mut v = vec![self.l, self.w, self.h];
        v.sort();
        (v[0], v[1])
    }

    fn slack(&self) -> Size {
        let (x, y) = self.two_smallest_sides();

        x * y
    }

    fn total_area(&self) -> Size {
        self.area() + self.slack()
    }

    fn ribbon(&self) -> Size {
        let (x, y) = self.two_smallest_sides();
        x + x + y + y
    }

    fn bow(&self) -> Size {
        self.l * self.w * self.h
    }

    fn total_ribbon(&self) -> Size {
        self.ribbon() + self.bow()
    }
}

#[derive(Debug)]
enum ParseBoxError {
    ParameterCountError,
    NumberFormatError(ParseIntError),
}

impl fmt::Display for ParseBoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseBoxError::ParameterCountError => "Invalid number of parameter".fmt(f),
            ParseBoxError::NumberFormatError(ref e) => e.fmt(f),
        }
    }
}

impl From<ParseIntError> for ParseBoxError {
    fn from(e: ParseIntError) -> ParseBoxError {
        ParseBoxError::NumberFormatError(e)
    }
}

impl FromStr for Box {
    type Err = ParseBoxError;

    fn from_str(s: &str) -> Result<Box, ParseBoxError> {
        let v: Vec<&str> = s.trim().split('x').collect();
        if v.len() != 3 {
            return Err(ParseBoxError::ParameterCountError);
        }

        let l = v[0].parse()?;
        let w = v[1].parse()?;
        let h = v[2].parse()?;

        Ok(Box::new(l, w, h))
    }
}

struct Total {
    area: u32,
    ribbon: u32,
}

impl FromStr for Total {
    type Err = ParseBoxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut area = 0;
        let mut ribbon = 0;
        for line in s.lines() {
            let b: Box = line.parse()?;
            area += b.total_area();
            ribbon += b.total_ribbon();
        }
        Ok(Total {
            area,
            ribbon,
        })
    }
}

fn main() -> Result<(), ParseBoxError> {
    let t: Total = INPUT.parse()?;

    println!("Total area: {}", t.area);
    println!("Total ribbon: {}", t.ribbon);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Box, INPUT, Total};

    #[test]
    fn test_solutions() {
        let t: Total = INPUT.parse().unwrap();

        assert_eq!(t.area, 1588178);
        assert_eq!(t.ribbon, 3783758);
    }

    #[test]
    fn two_by_three_by_four() {
        assert_eq!(Box::new(2, 3, 4).total_area(), 58);
        assert_eq!(Box::new(1, 1, 10).total_area(), 43);
    }

    #[test]
    fn parse_and_evaluate() {
        assert_eq!("2x3x4".parse::<Box>().unwrap().total_area(), 58)
    }

    #[test]
    fn ribbon() {
        assert_eq!(Box::new(2, 3, 4).total_ribbon(), 34);
        assert_eq!(Box::new(1, 1, 10).total_ribbon(), 14);
    }
}
