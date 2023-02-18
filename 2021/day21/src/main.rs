use std::collections::HashMap;

const INPUT: (usize, usize) = (10, 1);

// dumb but enough for part 1
fn part_1(input: (usize, usize)) -> usize {
    let mut dice = (1..=100).cycle().enumerate();

    let mut p1_pos = input.0;
    let mut p2_pos = input.1;
    let mut p1_score = 0;
    let mut p2_score = 0;

    loop {
        let d1 = dice.next().unwrap();
        let d2 = dice.next().unwrap();
        let d3 = dice.next().unwrap();

        p1_pos = (p1_pos - 1 + d1.1 + d2.1 + d3.1) % 10 + 1;
        p1_score += p1_pos;
        if p1_score >= 1000 {
            return p2_score * (d3.0 + 1);
        }

        let d1 = dice.next().unwrap();
        let d2 = dice.next().unwrap();
        let d3 = dice.next().unwrap();
        p2_pos = (p2_pos - 1 + d1.1 + d2.1 + d3.1) % 10 + 1;
        p2_score += p2_pos;
        if p2_score >= 1000 {
            return p1_score * (d3.0 + 1);
        }
    }
}

fn previous_pos(pos: usize, d: usize) -> usize {
    let mut pos = pos - 1; // 0..10
    if pos < d {
        pos += 10;
    }
    pos - d + 1
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Player {
    Player1,
    Player2,
}

// the number of ways 3 three-sided dice can sum up to n+1:
const DICE_OUTCOME_COUNTS: [u128; 9] = [0, 0, 1, 3, 6, 7, 6, 3, 1];

fn dice_outcome_count(d: usize) -> u128 {
    DICE_OUTCOME_COUNTS[d - 1]
}

// a dynamic programming solution where we build from the start
// for each round, compute the ways to reach a specific state
// that combines the pos+score of each player.
// sum over the dice rolls the ways to get to current state
// given the dices; use dice_outcome_count to just deal with dice sum
// winning is handled a bit differently because player 1 starts and thus
// has some advantage.
// tedious, but not really complicated
// things to keep in mind:
// - for a given score and pos, the previous score was always score - pos
// - therefore, score < pos are not possible states
fn winning_ways((p1, p2): (usize, usize), winner: Player) -> u128 {
    let mut memo = HashMap::new();
    // before game
    memo.insert((0, 0, p1, 0, p2), 1);

    for round in 1.. {
        let mut round_ways = 0;

        for p1_score in 1..=20 {
            for p1_pos in 1..=10 {
                if p1_score < p1_pos {
                    continue;
                }
                for p2_score in 1..=20 {
                    for p2_pos in 1..=10 {
                        if p2_score < p2_pos {
                            continue;
                        }
                        let mut ways = 0;
                        for d1 in 3..=9 {
                            let p1_pre_pos = previous_pos(p1_pos, d1);
                            for d2 in 3..=9 {
                                let p2_pre_pos = previous_pos(p2_pos, d2);
                                ways += dice_outcome_count(d1)
                                    * dice_outcome_count(d2)
                                    * memo
                                        .get(&(
                                            round - 1,
                                            p1_score - p1_pos,
                                            p1_pre_pos,
                                            p2_score - p2_pos,
                                            p2_pre_pos,
                                        ))
                                        .unwrap_or(&0);
                            }
                        }
                        memo.insert((round, p1_score, p1_pos, p2_score, p2_pos), ways);
                        round_ways += ways;
                    }
                }
            }
        }
        if winner == Player::Player1 {
            for p1_score in 21..=30 {
                for p1_pos in 1..=10 {
                    // we had already won the previous round
                    if p1_score - p1_pos > 20 {
                        continue;
                    }
                    for p2_score in 1..=20 {
                        for p2_pos in 1..=10 {
                            let mut ways = 0;
                            for d1 in 3..=9 {
                                let p1_pre_pos = previous_pos(p1_pos, d1);
                                ways += dice_outcome_count(d1)
                                    * memo
                                        .get(&(
                                            round - 1,
                                            p1_score - p1_pos,
                                            p1_pre_pos,
                                            p2_score,
                                            p2_pos,
                                        ))
                                        .unwrap_or(&0);
                            }
                            memo.insert((round, p1_score, p1_pos, p2_pos, p2_score), ways);
                            round_ways += ways;
                        }
                    }
                }
            }
        } else {
            for p2_score in 21..=30 {
                for p2_pos in 1..=10 {
                    if p2_score - p2_pos > 20 {
                        continue;
                    }
                    for p1_score in 1..=20 {
                        for p1_pos in 1..=10 {
                            if p1_score < p1_pos {
                                continue;
                            }
                            let mut ways = 0;
                            for d1 in 3..=9 {
                                for d2 in 3..=9 {
                                    let p2_pre_pos = previous_pos(p2_pos, d1);
                                    let p1_pre_pos = previous_pos(p1_pos, d2);
                                    ways += dice_outcome_count(d1)
                                        * dice_outcome_count(d2)
                                        * memo
                                            .get(&(
                                                round - 1,
                                                p1_score - p1_pos,
                                                p1_pre_pos,
                                                p2_score - p2_pos,
                                                p2_pre_pos,
                                            ))
                                            .unwrap_or(&0);
                                }
                            }
                            round_ways += ways;
                            memo.insert((round, p1_score, p1_pos, p2_score, p2_pos), ways);
                        }
                    }
                }
            }
        }

        if round_ways == 0 {
            break;
        }
    }

    memo.into_iter()
        .filter_map(|((_, p1_score, _, p2_score, _), ways)| {
            let score = if winner == Player::Player1 {
                p1_score
            } else {
                p2_score
            };
            if score > 20 {
                Some(ways)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: (usize, usize)) -> u128 {
    winning_ways(input, Player::Player1).max(winning_ways(input, Player::Player2))
}
fn main() {
    let mut dices = vec![0; 9];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                dices[i + j + k + 2] += 1;
            }
        }
    }

    println!("{dices:?}");
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    const TEST_INPUT: (usize, usize) = (4, 8);

    #[test]
    fn test_part_1() {
        assert_eq!(739785, part_1(TEST_INPUT));
        assert_eq!(920079, part_1(INPUT));
    }

    // those tests should be release only
    #[test]
    fn test_part_2() {
        assert_eq!(444356092776315, part_2(TEST_INPUT));
        assert_eq!(56852759190649, part_2(INPUT));
    }
}
