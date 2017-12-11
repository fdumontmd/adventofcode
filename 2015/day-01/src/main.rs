use std::io;
use std::io::Read;
use std::iter::Iterator;
use std::str;

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

fn floor(input: &str) -> Option<i64> {
    FloorIterator::new(input).last()
}

fn first_basement(input: &str) -> Option<usize> {
    FloorIterator::new(input).position(|f| f == -1).map(|p| p + 1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Floor: {:?}", floor(&input));
    println!("First basement: {:?}", first_basement(&input));
}

#[cfg(test)]
mod tests {
    use super::{floor, first_basement};

    #[test]
    fn no_floor() {
        assert_eq!(floor(""), None);
        assert_eq!(first_basement(""), None);
    }

    #[test]
    fn floor_0() {
        assert_eq!(floor("(())"), Some(0));
        assert_eq!(floor("()()"), Some(0));
    }

    #[test]
    fn floor_3() {
        assert_eq!(floor("((("), Some(3));
        assert_eq!(floor("(()(()("), Some(3));
        assert_eq!(floor("))((((("), Some(3));
    }

    #[test]
    fn floor_minus_1() {
        assert_eq!(floor("())"), Some(-1));
        assert_eq!(floor("))("), Some(-1));
    }

    #[test]
    fn floor_minus_3() {
        assert_eq!(floor(")))"), Some(-3));
        assert_eq!(floor(")())())"), Some(-3));
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
