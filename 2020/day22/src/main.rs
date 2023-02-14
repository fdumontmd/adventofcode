use std::collections::{HashSet, VecDeque};

static INPUT: &str = include_str!("input.txt");

struct Game {
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
}

enum Winner {
    Player1,
    Player2,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let players: Vec<_> = input.split("\n\n").collect();
        assert_eq!(2, players.len());

        Self {
            player1: Game::parse_one_player(players[0]),
            player2: Game::parse_one_player(players[1]),
        }
    }

    fn parse_one_player(input: &str) -> VecDeque<u8> {
        input.lines().filter_map(|l| l.parse::<u8>().ok()).collect()
    }

    fn round(&mut self) -> bool {
        if self.player1.is_empty() || self.player2.is_empty() {
            false
        } else {
            let Some(top1) = self.player1.pop_front() else { unreachable!()};
            let Some(top2) = self.player2.pop_front() else { unreachable!()};

            if top1 > top2 {
                self.player1.push_back(top1);
                self.player1.push_back(top2);
            } else {
                self.player2.push_back(top2);
                self.player2.push_back(top1);
            }

            !self.player2.is_empty() && !self.player2.is_empty()
        }
    }

    fn play(&mut self) {
        while self.round() {}
    }

    fn play_recursive(&mut self) -> Winner {
        let mut seen = HashSet::new();

        while !self.player1.is_empty() && !self.player2.is_empty() {
            let config = (self.player1.clone(), self.player2.clone());
            let player1_wins = seen.contains(&config);

            if !player1_wins {
                seen.insert(config);
            }

            let Some(top1) = self.player1.pop_front() else { unreachable!()};
            let Some(top2) = self.player2.pop_front() else { unreachable!()};

            if player1_wins {
                // instant win!
                return Winner::Player1;
            } else if self.player1.len() < top1 as usize || self.player2.len() < top2 as usize {
                if top1 > top2 {
                    self.player1.push_back(top1);
                    self.player1.push_back(top2);
                } else {
                    self.player2.push_back(top2);
                    self.player2.push_back(top1);
                }
            } else {
                // recursive game
                let mut rec_game = Game {
                    player1: self.player1.iter().cloned().take(top1 as usize).collect(),
                    player2: self.player2.iter().cloned().take(top2 as usize).collect(),
                };

                match rec_game.play_recursive() {
                    Winner::Player1 => {
                        self.player1.push_back(top1);
                        self.player1.push_back(top2);
                    }
                    Winner::Player2 => {
                        self.player2.push_back(top2);
                        self.player2.push_back(top1);
                    }
                }
            }
        }

        if self.player2.is_empty() {
            Winner::Player1
        } else {
            Winner::Player2
        }
    }

    fn score(&self) -> usize {
        let deck = if self.player1.is_empty() {
            &self.player2
        } else {
            &self.player1
        };

        deck.iter()
            .rev()
            .enumerate()
            .map(|(idx, c)| (idx + 1) * (*c as usize))
            .sum()
    }
}

fn part_1(input: &str) -> usize {
    let mut game = Game::from_str(input);
    game.play();
    game.score()
}

fn part_2(input: &str) -> usize {
    let mut game = Game::from_str(input);
    game.play_recursive();
    game.score()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part_1() {
        assert_eq!(306, part_1(TEST_INPUT));
        assert_eq!(32824, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(291, part_2(TEST_INPUT));
        // slow in debug mode
        assert_eq!(36515, part_2(INPUT));
    }
}
