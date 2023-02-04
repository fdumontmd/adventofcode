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

struct Deck {
    cards: Vec<usize>,
}

impl Deck {
    fn new(size: usize) -> Self {
        Self {
            cards: Vec::from_iter(0..size),
        }
    }

    fn apply(&mut self, sort: Sorting) {
        match sort {
            Sorting::DealIntoNewStack => self.deal_into_new_stack(),
            Sorting::Cut(c) => self.cut(c),
            Sorting::DealWithIncrement(i) => self.deal_with_increment(i),
        }
    }

    fn apply_all(&mut self, sort: &Vec<Sorting>) {
        for s in sort {
            self.apply(*s);
        }
    }

    fn deal_into_new_stack(&mut self) {
        self.cards.reverse()
    }

    fn cut(&mut self, n: isize) {
        if n > 0 {
            self.cards.rotate_left(n as usize);
        } else if n < 0 {
            self.cards.rotate_right(n.abs() as usize);
        }
    }

    fn deal_with_increment(&mut self, inc: usize) {
        let mut new_cards = vec![0; self.cards.len()];
        let mut pointer = 0;
        for card in &self.cards {
            new_cards[pointer] = *card;
            pointer = (pointer + inc) % new_cards.len();
        }
        self.cards = new_cards;
    }
}
fn part_1() -> usize {
    const DECK_SIZE: i128 = 10007;
    let mut group = Group::new(DECK_SIZE);
    for line in INPUT.lines() {
        let sort = Sorting::from_str(line);
        group = group.then(&Group::from_sorting(DECK_SIZE, sort));
    }

    group.apply(2019) as usize
}

// fn part_1() -> usize {
//     let mut deck = Deck::new(10007);
//     for line in INPUT.lines() {
//         let sort = Sorting::from_str(line);
//         deck.apply(sort);
//     }
//     dbg!(deck.cards.iter().position(|c| *c == 2019).unwrap());
//
//     let mut group = Group::new(10007);
//     for line in INPUT.lines() {
//         let sort = Sorting::from_str(line);
//         let g = Group::from_sorting(10007, sort);
//         group = group.then(&g);
//     }
//     group.apply(2019) as usize
// }

#[derive(Debug)]
struct Group {
    a: i128,
    b: i128,
    l: i128,
}

impl Group {
    fn new(l: i128) -> Self {
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

    fn then(&self, other: &Self) -> Self {
        Group {
            a: (self.a * other.a) % self.l,
            b: ((other.a * self.b) + other.b).rem_euclid(self.l),
            l: self.l,
        }
    }

    fn apply(&self, x: i128) -> i128 {
        let axb = self.a * x + self.b;
        axb.rem_euclid(self.l)
    }
}

fn main() {
    println!("part 1: {}", part_1());
}

#[cfg(test)]
mod tests {
    use crate::{Deck, Group, Sorting};
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

    #[test]
    fn test_deal_new_stack() {
        let mut deck = Deck::new(10);
        deck.deal_into_new_stack();
        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], deck.cards);
    }

    #[test]
    fn test_cut_3() {
        let mut deck = Deck::new(10);
        deck.cut(3);
        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], deck.cards);
    }

    #[test]
    fn test_cut_minus_4() {
        let mut deck = Deck::new(10);
        deck.cut(-4);
        assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], deck.cards);
    }

    #[test]
    fn test_deal_with_increment_3() {
        let mut deck = Deck::new(10);
        deck.deal_with_increment(3);
        assert_eq!(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3], deck.cards);
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
    fn test_sorting(input: &str) {
        let (sorting, result) = parse_test_case(input);
        let mut deck = Deck::new(10);
        deck.apply_all(&sorting);
        assert_eq!(result, deck.cards);
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
        let mut group = Group::new(10);
        for s in sorting {
            let g = Group::from_sorting(10, s);
            group = group.then(&g);
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
}
