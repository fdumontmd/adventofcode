use crate::{custom_error::AocError, CardWithJoker, Hand};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let hand: Vec<_> = l.split_whitespace().collect();
            let bid: u64 = hand[1].parse().unwrap();
            let mut hand: Vec<u8> = hand[0].as_bytes().to_vec();
            let cards: Vec<CardWithJoker> =
                hand.iter().map(|c| CardWithJoker::from_card(*c)).collect();
            hand.sort();

            (
                Hand::rank_with_joker(&hand),
                cards,
                String::from_utf8(hand).unwrap(),
                bid,
            )
        })
        .collect();
    hands.sort();
    let winnings: u64 = hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.3)
        .sum();
    Ok(format!("{}", winnings))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("5905", process(INPUT)?);
        Ok(())
    }
}
