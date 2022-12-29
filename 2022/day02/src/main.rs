static INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn from_str_left(input: &str) -> Self {
        match input {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Unknow left input {}", input),
        }
    }

    fn from_str_right(input: &str) -> Self {
        match input {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("Unknow right input {}", input),
        }
    }

    fn succ(&self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn from_outcome(other: Hand, outcome: Outcome) -> Hand {
        match outcome {
            Outcome::Draw => other,
            Outcome::Win => other.succ(),
            Outcome::Loss => other.prev(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn new(o: Hand, m: Hand) -> Self {
        if o == m {
            Outcome::Draw
        } else if o == m.prev() {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    fn from_str(input: &str) -> Self {
        match input {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknow outcome input {}", input),
        }
    }
}

struct Round(Hand, Hand);

impl Round {
    fn new(o: Hand, m: Hand) -> Self {
        Round(o, m)
    }

    fn score(&self) -> usize {
        Outcome::new(self.0, self.1).score() + self.1.score()
    }
}

fn parse_part_1(input: &str) -> Vec<Round> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let round: Vec<&str> = line.split(' ').collect();
            if round.len() != 2 {
                panic!("unrecognized format for round: {}", line);
            }
            Round::new(
                Hand::from_str_left(round[0]),
                Hand::from_str_right(round[1]),
            )
        })
        .collect()
}

fn parse_part_2(input: &str) -> Vec<Round> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let round: Vec<&str> = line.split(' ').collect();
            if round.len() != 2 {
                panic!("unrecognized format for round: {}", line);
            }
            let other = Hand::from_str_left(round[0]);
            let outcome = Outcome::from_str(round[1]);
            let me = Hand::from_outcome(other, outcome);
            Round::new(other, me)
        })
        .collect()
}

fn total_score(rounds: &[Round]) -> usize {
    rounds.iter().map(|r| r.score()).sum()
}

fn main() {
    let rounds = parse_part_1(INPUT);
    println!("Total score part 1: {}", total_score(&rounds));
    let rounds = parse_part_2(INPUT);
    println!("Total score part 2: {}", total_score(&rounds));
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_INPUT: &str = r#"
A Y
B X
C Z
"#;

    #[test]
    fn test_part_1() {
        let data = parse_part_1(TEST_INPUT);
        assert_eq!(15, total_score(&data))
    }

    #[test]
    fn test_part_2() {
        let rounds = parse_part_2(TEST_INPUT);
        assert_eq!(12, total_score(&rounds));
    }

    #[test]
    fn actual_part_1() {
        let rounds = parse_part_1(INPUT);
        assert_eq!(12458, total_score(&rounds))
    }

    #[test]
    fn actual_part_2() {
        let rounds = parse_part_2(INPUT);
        assert_eq!(12683, total_score(&rounds));
    }
}
