static INPUT: &str = include_str!("input.txt");

struct Range(usize, usize);

impl Range {
    fn from_str(input: &str) -> Self {
        let mut input = input.split('-');
        Range(
            input.next().unwrap().parse().unwrap(),
            input.next().unwrap().parse().unwrap(),
        )
    }

    fn cover(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlap(&self, other: &Range) -> bool {
        (self.0 <= other.0 && other.0 <= self.1)
            || (other.0 <= self.0 && self.0 <= other.1)
            || (self.0 <= other.1 && other.1 <= self.1)
            || (other.0 <= self.1 && self.1 <= other.1)
    }
}

fn parse(input: &str) -> (Range, Range) {
    let mut input = input.split(',');
    (
        Range::from_str(input.next().unwrap()),
        Range::from_str(input.next().unwrap()),
    )
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse)
        .filter(|&(ref r1, ref r2)| r1.cover(r2) || r2.cover(r1))
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse)
        .filter(|&(ref r1, ref r2)| r1.overlap(r2))
        .count()
}
fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(4, part_2(TEST_INPUT));
    }

    #[test]
    fn actual_part_1() {
        assert_eq!(507, part_1(INPUT));
    }

    #[test]
    fn actual_part_2() {
        assert_eq!(897, part_2(INPUT));
    }
}
