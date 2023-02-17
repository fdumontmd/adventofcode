use std::fmt::Display;

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SFNumComp {
    Open,
    Close,
    Regular(u64),
}

impl SFNumComp {
    fn parse_u8(byte: u8) -> Option<Self> {
        if byte.is_ascii_digit() {
            Some(SFNumComp::Regular((byte - b'0') as u64))
        } else if byte == b'[' {
            Some(SFNumComp::Open)
        } else if byte == b']' {
            Some(SFNumComp::Close)
        } else {
            None
        }
    }

    fn is_regular(&self) -> bool {
        match self {
            SFNumComp::Open => false,
            SFNumComp::Close => false,
            SFNumComp::Regular(_) => true,
        }
    }

    fn increase(&mut self, amount: u64) {
        if let SFNumComp::Regular(r) = *self {
            *self = SFNumComp::Regular(r + amount);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SFNum(Vec<SFNumComp>);

// just store the parsed version in a Vec
impl SFNum {
    fn parse(input: &str) -> Self {
        Self(input.bytes().filter_map(SFNumComp::parse_u8).collect())
    }

    fn add(&mut self, other: Self) {
        self.0.insert(0, SFNumComp::Open);
        self.0.extend(other.0);
        self.0.push(SFNumComp::Close);
        self.reduce();
    }

    fn reduce(&mut self) {
        let mut depth = 0;
        let mut pos = 0;
        while pos < self.0.len() {
            match self.0[pos] {
                SFNumComp::Open => depth += 1,
                SFNumComp::Close => depth -= 1,
                SFNumComp::Regular(_) => {}
            }

            if depth == 5 {
                self.explode(pos);
                depth -= 1;
            }
            pos += 1;
        }

        assert_eq!(0, depth);

        let mut pos = 0;
        while pos < self.0.len() {
            match self.0[pos] {
                SFNumComp::Open => depth += 1,
                SFNumComp::Close => depth -= 1,
                SFNumComp::Regular(r) => {
                    if r >= 10 {
                        self.split(pos, depth);
                        // restart
                        pos = 0;
                        depth = 0;
                        continue;
                    }
                }
            }
            pos += 1;
        }
    }

    // explode the pair starting at pos; expect two regular numbers
    // in the following pos
    fn explode(&mut self, pos: usize) {
        assert_eq!(SFNumComp::Open, self.0[pos]);
        assert!(self.0.len() > pos + 3);
        let SFNumComp::Regular(left) = self.0[pos + 1] else { panic!("expected a regular number")};
        let SFNumComp::Regular(right) = self.0[pos + 2] else { panic!("expected a regular number")};
        assert_eq!(SFNumComp::Close, self.0[pos + 3]);
        self.0.remove(pos + 1);
        self.0.remove(pos + 1);
        self.0.remove(pos + 1);
        self.0[pos] = SFNumComp::Regular(0);

        if let Some(lp) = self.0[0..pos].iter().rposition(|c| c.is_regular()) {
            self.0[lp].increase(left);
        }

        if let Some(rp) = self.0[pos + 1..].iter().position(|c| c.is_regular()) {
            self.0[rp + pos + 1].increase(right);
        }
    }

    fn split(&mut self, pos: usize, depth: usize) {
        let SFNumComp::Regular(r) = self.0[pos] else { panic!("expected a regular number")};
        assert!(r >= 10);
        let left = r / 2;
        let right = r - left;
        self.0[pos] = SFNumComp::Close;
        self.0.insert(pos, SFNumComp::Regular(right));
        self.0.insert(pos, SFNumComp::Regular(left));
        self.0.insert(pos, SFNumComp::Open);
        if depth == 4 {
            self.explode(pos);
        }
    }
}

enum Expr {
    Regular(u64),
    Pair(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn from_sf_num(sf_num: &SFNum) -> Self {
        let mut i = sf_num.0.iter().cloned();
        Expr::from_iter(&mut i)
    }
    fn from_iter<I: Iterator<Item = SFNumComp>>(i: &mut I) -> Self {
        let Some(sfnc) = i.next() else { panic!("no more token")};
        match sfnc {
            SFNumComp::Open => {
                let left = Expr::from_iter(i);
                let right = Expr::from_iter(i);
                let expr = Expr::Pair(Box::new(left), Box::new(right));
                let Some(SFNumComp::Close) = i.next() else { panic!("missing close token")};
                expr
            }
            SFNumComp::Close => panic!("unexpected close"),
            SFNumComp::Regular(r) => Expr::Regular(r),
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Expr::Regular(v) => *v,
            Expr::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Regular(r) => write!(f, "{r}"),
            Expr::Pair(l, r) => write!(f, "[{l},{r}]"),
        }
    }
}

impl Display for SFNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expr = Expr::from_sf_num(self);
        write!(f, "{expr}")
    }
}

fn add_all(input: &str) -> SFNum {
    let lines: Vec<_> = input.lines().collect();
    let mut acc = SFNum::parse(lines[0]);
    for n in &lines[1..] {
        let n = SFNum::parse(n);
        acc.add(n);
    }
    acc
}

fn part_1(input: &str) -> u64 {
    Expr::from_sf_num(&add_all(input)).magnitude()
}

fn part_2(input: &str) -> u64 {
    let mut magnitudes = vec![];
    let numbers: Vec<_> = input.lines().map(SFNum::parse).collect();

    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            let mut first = numbers[i].clone();
            let second = numbers[j].clone();

            first.add(second);
            magnitudes.push(Expr::from_sf_num(&first).magnitude());

            let mut first = numbers[j].clone();
            let second = numbers[i].clone();

            first.add(second);
            magnitudes.push(Expr::from_sf_num(&first).magnitude());
        }
    }

    magnitudes.into_iter().max().unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{add_all, part_1, part_2, Expr, SFNum, INPUT};

    #[test_case("[[[[[9,8],1],2],3],4]", 4, "[[[[0,9],2],3],4]")]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]", 8, "[7,[6,[5,[7,0]]]]")]
    #[test_case("[[6,[5,[4,[3,2]]]],1]", 7, "[[6,[5,[7,0]]],3]")]
    #[test_case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        7,
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    )]
    #[test_case(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        17,
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
    )]
    fn test_explode_left_only(from: &str, at: usize, to: &str) {
        let mut from = SFNum::parse(from);
        let to = SFNum::parse(to);
        from.explode(at);
        assert_eq!(to, from);
    }

    #[test]
    fn test_add() {
        let mut acc = SFNum::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let other = SFNum::parse("[1,1]");
        acc.add(other);
        let to = SFNum::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(to, acc);
    }

    #[test]
    fn test_add_all() {
        let input = r"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        let sum = add_all(input);
        let to = SFNum::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(to, sum);
    }

    #[test_case("[[1,2],[[3,4],5]]", 143)]
    #[test_case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[test_case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[test_case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[test_case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    #[test_case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    fn test_magniture(input: &str, magnitude: u64) {
        assert_eq!(
            magnitude,
            Expr::from_sf_num(&SFNum::parse(input)).magnitude()
        );
    }

    static TEST_INPUT: &str = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_part_1() {
        assert_eq!(4140, part_1(TEST_INPUT));
        assert_eq!(4435, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3993, part_2(TEST_INPUT));
        assert_eq!(4802, part_2(INPUT));
    }
}
