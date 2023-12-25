pub mod custom_error;

use std::cmp::Reverse;

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hand {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn rank(hand: &[u8]) -> Self {
        let mut rank = Vec::new();
        for (_, group) in &hand.iter().cloned().group_by(|a| *a) {
            rank.push(group.count());
        }

        rank.sort_by_key(|c| Reverse(*c));
        match &rank[..] {
            [5] => Hand::FiveOfAKind,
            [4, 1] => Hand::FourOfAKind,
            [3, 2] => Hand::FullHouse,
            [3, ..] => Hand::ThreeOfAKind,
            [2, 2, 1] => Hand::TwoPairs,
            [2, ..] => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }

    fn rank_with_joker(hand: &[u8]) -> Self {
        // just add jokers to the highest count card
        let jokers = hand.iter().filter(|c| **c == b'J').count();
        let mut rank = Vec::new();
        for (_, group) in &hand.iter().cloned().filter(|c| *c != b'J').group_by(|a| *a) {
            rank.push(group.count());
        }
        rank.sort_by_key(|c| Reverse(*c));
        if rank.is_empty() {
            rank.push(jokers);
        } else {
            rank[0] += jokers;
        }
        match &rank[..] {
            [5] => Hand::FiveOfAKind,
            [4, 1] => Hand::FourOfAKind,
            [3, 2] => Hand::FullHouse,
            [3, ..] => Hand::ThreeOfAKind,
            [2, 2, 1] => Hand::TwoPairs,
            [2, ..] => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    pub fn from_card(c: u8) -> Self {
        match c {
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::T,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => panic!("Unknown card {}", c),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CardWithJoker {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl CardWithJoker {
    pub fn from_card(c: u8) -> Self {
        match c {
            b'J' => CardWithJoker::Joker,
            b'2' => CardWithJoker::Two,
            b'3' => CardWithJoker::Three,
            b'4' => CardWithJoker::Four,
            b'5' => CardWithJoker::Five,
            b'6' => CardWithJoker::Six,
            b'7' => CardWithJoker::Seven,
            b'8' => CardWithJoker::Eight,
            b'9' => CardWithJoker::Nine,
            b'T' => CardWithJoker::T,
            b'Q' => CardWithJoker::Q,
            b'K' => CardWithJoker::K,
            b'A' => CardWithJoker::A,
            _ => panic!("Unknown card {}", c),
        }
    }
}
pub mod part1;
pub mod part2;
