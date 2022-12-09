use std::iter::Iterator;
use std::str;

static INPUT: &str = include_str!("input.txt");
struct FloorIterator<'a> {
    c: str::Chars<'a>,
    floor: i64,
}

impl<'a> FloorIterator<'a> {
    fn new(input: &'a str) -> FloorIterator<'a> {
        FloorIterator {
            c: input.chars(),
            floor: 0,
        }
    }
}

impl<'a> Iterator for FloorIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if let Some(c) = self.c.next() {
            if c == '(' {
                self.floor += 1;
            } else if c == ')' {
                self.floor -= 1;
            }
            Some(self.floor)
        } else {
            None
        }
    }
}

fn floor(input: &str) -> i64 {
    // start on floor 0, if no button pushed we stay there
    FloorIterator::new(input).last().unwrap_or(0)
}

fn first_basement(input: &str) -> Option<usize> {
    FloorIterator::new(input).position(|f| f == -1).map(|p| p + 1)
}

fn main() {
    println!("Floor: {}", floor(&INPUT));
    println!("First basement: {}", first_basement(&INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::{floor, first_basement, INPUT};

    #[test]
    fn test_part_1() {
        assert_eq!(floor(INPUT), 138);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(first_basement(INPUT), Some(1771));
    }

    #[test]
    fn no_button() {
        assert_eq!(floor(""), 0);
        assert_eq!(first_basement(""), None);
    }

    #[test]
    fn floor_0() {
        assert_eq!(floor("(())"), 0);
        assert_eq!(floor("()()"), 0);
    }

    #[test]
    fn floor_3() {
        assert_eq!(floor("((("), 3);
        assert_eq!(floor("(()(()("), 3);
        assert_eq!(floor("))((((("), 3);
    }

    #[test]
    fn floor_minus_1() {
        assert_eq!(floor("())"), -1);
        assert_eq!(floor("))("), -1);
    }

    #[test]
    fn floor_minus_3() {
        assert_eq!(floor(")))"), -3);
        assert_eq!(floor(")())())"), -3);
    }

    #[test]
    fn first_basement_test() {
        assert_eq!(first_basement(")"), Some(1));
        assert_eq!(first_basement("()())"), Some(5));
    }

    #[test]
    fn never_basement() {
        assert_eq!(first_basement("("), None);
    }
}
