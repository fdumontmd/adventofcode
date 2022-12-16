use std::cmp::Ordering;

static INPUT: &str = include_str!("input.txt");

#[derive(Clone, Debug, Ord, Eq)]
pub enum Item {
    List(Vec<Item>),
    Int(i32),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Int(si), Item::Int(oi)) => si.partial_cmp(oi),
            (Item::Int(si), other) => Item::List(vec![Item::Int(*si)]).partial_cmp(other),
            (_, Item::Int(oi)) => self.partial_cmp(&Item::List(vec![Item::Int(*oi)])),
            (Item::List(sv), Item::List(ov)) => cmp_list(sv, ov),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn cmp_list(sv: &[Item], ov: &[Item]) -> Option<std::cmp::Ordering> {
    let len = sv.len().min(ov.len());

    for (si, oi) in sv[..len].iter().zip(ov[..len].iter()) {
        if let Some(cmp) = si.partial_cmp(oi) {
            if cmp == Ordering::Equal {
                continue;
            } else {
                return Some(cmp);
            }
        }
    }

    sv.len().partial_cmp(&ov.len())
}

mod parser {
    use nom::{
        branch::alt,
        character::complete::{char, i32, newline, space0},
        multi::{many0, many1, separated_list0},
        sequence::{delimited, separated_pair, terminated},
        IResult,
    };

    use crate::Item;

    // TODO: read fasterthanlime article about nice error report
    pub fn parse_input(input: &str) -> Vec<(Item, Item)> {
        parse_input_err(input).unwrap().1
    }

    pub fn parse_input_err(input: &str) -> IResult<&str, Vec<(Item, Item)>> {
        terminated(separated_list0(many1(newline), parse_pair), many0(newline))(input)
    }

    fn parse_pair(input: &str) -> IResult<&str, (Item, Item)> {
        let (left, right) = separated_pair(parse_line, newline, parse_line)(input)?;
        Ok((left, right))
    }

    fn parse_line(input: &str) -> IResult<&str, Item> {
        parse_list(input)
    }

    fn parse_item(input: &str) -> IResult<&str, Item> {
        alt((parse_list, parse_int))(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Item> {
        let (input, items) =
            delimited(char('['), separated_list0(char(','), parse_item), char(']'))(input)?;

        Ok((input, Item::List(items)))
    }

    fn parse_int(input: &str) -> IResult<&str, Item> {
        let (input, i) = delimited(space0, i32, space0)(input)?;
        Ok((input, Item::Int(i)))
    }
}

fn part_01(input: &str) -> usize {
    parser::parse_input(input)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, (f, s))| {
            if f.cmp(&s) == Ordering::Less {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_02(input: &str) -> usize {
    let mut items: Vec<_> = parser::parse_input(input)
        .into_iter()
        .flat_map(|(l, r)| vec![l, r].into_iter())
        .collect();

    let div1 = Item::List(vec![Item::List(vec![Item::Int(2)])]);
    let div2 = Item::List(vec![Item::List(vec![Item::Int(6)])]);

    items.push(div1.clone());
    items.push(div2.clone());

    items.sort();

    let pos1 = items.iter().position(|i| i == &div1).unwrap();
    let pos2 = items.iter().position(|i| i == &div2).unwrap();

    (pos1 + 1) * (pos2 + 1)
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02};

    static TEST_INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_part_01() {
        assert_eq!(13, part_01(TEST_INPUT));
    }

    #[test]
    fn test_part_02() {
        assert_eq!(140, part_02(TEST_INPUT));
    }
}
