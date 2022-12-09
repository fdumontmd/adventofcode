use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

static INPUT: &str = include_str!("input.txt");

lazy_static! {
    static ref MOVE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

#[derive(Copy, Clone, Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_str(input: &str) -> Self {
        let cap = MOVE.captures(input).unwrap();
        Move {
            count: cap[1].parse().unwrap(),
            from: cap[2].parse().unwrap(),
            to: cap[3].parse().unwrap(),
        }
    }
}

struct Crane {
    stacks: Vec<Vec<char>>,
    crate_mover_9001: bool,
}

impl Crane {
    fn build_crane(input: &str) -> (Self, Vec<Move>) {
        let mut stacks = Vec::new();

        let mut moves = Vec::new();
        for (a_move, group) in &input.lines().group_by(|l| l.starts_with("move")) {
            if !a_move {
                for line in group {
                    for (stack, mut item) in line.chars().chunks(4).into_iter().enumerate() {
                        let item = item.nth(1).unwrap();
                        if item.is_alphabetic() {
                            while stacks.len() <= stack {
                                stacks.push(Vec::new());
                            }
                            stacks[stack].push(item);
                        }
                    }
                }
            } else {
                for line in group {
                    moves.push(Move::from_str(line));
                }
            }
        }

        for stack in &mut stacks {
            stack.reverse();
        }

        (
            Crane {
                stacks,
                crate_mover_9001: false,
            },
            moves,
        )
    }

    fn perform(&mut self, m: &Move) {
        if !self.crate_mover_9001 {
            for idx in 0..m.count {
                if let Some(top) = self.stacks[m.from - 1].pop() {
                    self.stacks[m.to - 1].push(top);
                } else {
                    panic!(
                        "Cannot execute {:#?}, not enough packages after {} steps",
                        m, idx
                    );
                }
            }
        } else {
            let from = &mut self.stacks[m.from - 1];
            let items: Vec<_> = from.drain(from.len() - m.count..).collect();
            for item in items {
                self.stacks[m.to - 1].push(item);
            }
        }
    }

    fn upgrade_to_9001(&mut self) {
        self.crate_mover_9001 = true;
    }

    fn run(&mut self, moves: &[Move]) {
        for m in moves {
            self.perform(m);
        }
    }

    fn tops(&self) -> String {
        self.stacks.iter().map(|s| s[s.len() - 1]).collect()
    }
}

fn main() {
    let (mut crane, moves) = Crane::build_crane(INPUT);
    crane.run(&moves);
    println!("Part 1: {}", crane.tops());
    let (mut crane, moves) = Crane::build_crane(INPUT);
    crane.upgrade_to_9001();
    crane.run(&moves);
    println!("Part 2: {}", crane.tops());
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part_1() {
        let (mut crane, moves) = Crane::build_crane(TEST_INPUT);
        crane.run(&moves);
        assert_eq!("CMZ", &crane.tops());
    }

    #[test]
    fn test_part_2() {
        let (mut crane, moves) = Crane::build_crane(TEST_INPUT);
        crane.upgrade_to_9001();
        crane.run(&moves);
        assert_eq!("MCD", &crane.tops());
    }

    #[test]
    fn real_part_1() {
        let (mut crane, moves) = Crane::build_crane(INPUT);
        crane.run(&moves);
        assert_eq!("JRVNHHCSJ", &crane.tops());
    }

    #[test]
    fn real_part_2() {
        let (mut crane, moves) = Crane::build_crane(INPUT);
        crane.upgrade_to_9001();
        crane.run(&moves);
        assert_eq!("GNFBSBJLH", &crane.tops());
    }
}
