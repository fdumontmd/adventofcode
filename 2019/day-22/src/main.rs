static INPUT: &str = include_str!("input.txt");
// deal to new stack: reverse
// cut N (positive): rotate_left
// cut -N : rotate_right
// deal with increment N: dedicated code

#[derive(Debug, Copy, Clone)]
enum Sorting {
    DealIntoNewStack,
    Cut(isize),
    DealWithIncrement(usize),
}

impl Sorting {
    fn from_str(input: &str) -> Self {
        if input == "deal into new stack" {
            Sorting::DealIntoNewStack
        } else if let Some(cut) = input.strip_prefix("cut ") {
            Sorting::Cut(cut.parse().unwrap())
        } else if let Some(inc) = input.strip_prefix("deal with increment ") {
            Sorting::DealWithIncrement(inc.parse().unwrap())
        } else {
            panic!("cannot parse {input}")
        }
    }
}

// invert the group built from the instructions
// then take the stupidly large power of that group
// -> need to look up: linear group over finite field inverse
//                     linear group over finite fields exponent
fn part_2() -> i128 {
    let g = Group::from_str(119315717514047, INPUT);
    let gi = g.inverse();
    let gmi = gi.exponent(101741582076661);
    gmi.apply(2020)
}

fn part_1() -> usize {
    const DECK_SIZE: i128 = 10007;
    let mut group = Group::unit(DECK_SIZE);
    for line in INPUT.lines() {
        let sort = Sorting::from_str(line);
        group = group.mult_by(&Group::from_sorting(DECK_SIZE, sort));
    }

    group.apply(2019) as usize
}

// worked this out from Reddit thread when I saw a*x+b as a group over modular integers
#[derive(Debug, Copy, Clone)]
struct Group {
    a: i128,
    b: i128,
    l: i128,
}

impl Group {
    fn unit(l: i128) -> Self {
        Self { a: 1, b: 0, l }
    }
    fn from_sorting(l: i128, sort: Sorting) -> Self {
        match sort {
            Sorting::DealIntoNewStack => Group {
                a: -1,
                b: -1 + l,
                l,
            },
            Sorting::Cut(c) => Group {
                a: 1,
                b: -c as i128,
                l,
            },
            Sorting::DealWithIncrement(i) => Group {
                a: i as i128,
                b: 0,
                l,
            },
        }
    }

    fn mult_by(&self, other: &Self) -> Self {
        Group {
            a: (self.a * other.a).rem_euclid(self.l),
            b: ((other.a * self.b) + other.b).rem_euclid(self.l),
            l: self.l,
        }
    }

    fn apply(&self, x: i128) -> i128 {
        let axb = self.a * x + self.b;
        axb.rem_euclid(self.l)
    }

    fn inverse(&self) -> Self {
        let ee = extended_gcd(self.l, self.a);
        let ai = ee.1 .0;

        assert_eq!(1, (self.a * ai).rem_euclid(self.l));

        Self {
            a: ai,
            b: (-self.b * ai).rem_euclid(self.l),
            l: self.l,
        }
    }

    fn from_str(l: i128, input: &str) -> Self {
        input
            .lines()
            .map(Sorting::from_str)
            .fold(Group::unit(l), |s, g| s.mult_by(&Group::from_sorting(l, g)))
    }

    fn exponent(&self, mut exp: i128) -> Self {
        let mut result = Group::unit(self.l);
        let mut base = *self;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mult_by(&base);
            }
            exp >>= 1;
            base = base.mult_by(&base);
        }

        result
    }
}

// copied from my aoc lib...
pub fn extended_gcd(a: i128, b: i128) -> (i128, (i128, i128), (i128, i128)) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_r, (old_t, old_s), (t, s))
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[cfg(test)]
mod tests {
    use crate::{Group, Sorting};
    use test_case::test_case;

    fn parse_test_case(input: &str) -> (Vec<Sorting>, Vec<usize>) {
        let mut sorting = Vec::new();
        let mut result = Vec::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(numbers) = line.strip_prefix("Result: ") {
                result = numbers
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            } else {
                sorting.push(Sorting::from_str(line));
            }
        }

        (sorting, result)
    }

    #[test_case(
        r"deal with increment 7
deal into new stack
deal into new stack
Result: 0 3 6 9 2 5 8 1 4 7"
    )]
    #[test_case(
        r"cut 6
deal with increment 7
deal into new stack
Result: 3 0 7 4 1 8 5 2 9 6"
    )]
    #[test_case(
        r"deal with increment 7
deal with increment 9
cut -2
Result: 6 3 0 7 4 1 8 5 2 9"
    )]
    #[test_case(
        r"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
Result: 9 2 5 8 1 4 7 0 3 6"
    )]
    fn test_group(input: &str) {
        let (sorting, result) = parse_test_case(input);
        let mut group = Group::unit(10);
        for s in sorting {
            let g = Group::from_sorting(10, s);
            group = group.mult_by(&g);
        }
        let mut out: Vec<usize> = vec![0; 10];
        (0..10).for_each(|d| out[group.apply(d) as usize] = d as usize);
        assert_eq!(result, out);
    }

    #[test]
    fn test_group_unit() {
        assert_eq!(
            2,
            Group::from_sorting(10, Sorting::DealIntoNewStack).apply(7),
        );
        assert_eq!(4, Group::from_sorting(10, Sorting::Cut(3)).apply(7),);
        assert_eq!(1, Group::from_sorting(10, Sorting::Cut(-4)).apply(7),);
        assert_eq!(
            1,
            Group::from_sorting(10, Sorting::DealWithIncrement(3)).apply(7)
        );
        assert_eq!(
            9,
            dbg!(Group::from_sorting(10, Sorting::DealWithIncrement(7)).apply(7)),
        );
    }

    #[test]
    fn test_inverse() {
        let g = Group::from_sorting(10, Sorting::DealWithIncrement(7));
        let gi = g.inverse();

        let u = g.mult_by(&gi);
        assert_eq!(u.a, 1);
        assert_eq!(u.b, 0);
        for i in 0..10 {
            assert_eq!(i, u.apply(i));
        }
    }

    #[test_case(
        r"deal with increment 7
deal into new stack
deal into new stack
Result: 0 3 6 9 2 5 8 1 4 7"
    )]
    #[test_case(
        r"cut 6
deal with increment 7
deal into new stack
Result: 3 0 7 4 1 8 5 2 9 6"
    )]
    #[test_case(
        r"deal with increment 7
deal with increment 9
cut -2
Result: 6 3 0 7 4 1 8 5 2 9"
    )]
    #[test_case(
        r"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
Result: 9 2 5 8 1 4 7 0 3 6"
    )]
    fn test_inverse_complexe(input: &str) {
        let (sorting, result) = parse_test_case(input);
        let mut group = Group::unit(10);
        for s in sorting {
            let g = Group::from_sorting(10, s);
            group = group.mult_by(&g);
        }
        let gi = group.inverse();
        let u = group.mult_by(&gi);
        assert_eq!(u.a, 1);
        assert_eq!(u.b, 0);
        // group maps card position to card position
        for i in 0..10 {
            assert_eq!(result[i] as i128, gi.apply(i as i128));
        }
    }
}
