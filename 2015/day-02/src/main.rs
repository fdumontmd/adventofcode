use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

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

        let l = try!(v[0].parse());
        let w = try!(v[1].parse());
        let h = try!(v[2].parse());

        Ok(Box::new(l, w, h))
    }
}



fn main() {
    let mut total_area = 0;
    let mut total_ribbon = 0;

    let mut input = String::new();

    while io::stdin().read_line(&mut input).is_ok() {
        if let Ok(b) = input.parse::<Box>() {
            total_area += b.total_area();
            total_ribbon += b.total_ribbon();
        } else {
            break;
        }
        input.clear();
    }

    println!("Total area: {}", total_area);
    println!("Total ribbon: {}", total_ribbon);
}

#[cfg(test)]
mod tests {
    use super::Box;

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
