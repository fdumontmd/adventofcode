static INPUT: usize = 909441;

struct Scoreboard {
    recipes: Vec<u8>,
    elf1: usize,
    elf2: usize,
    cutoff: Option<usize>,
    target: Option<Vec<u8>>,
}

impl Scoreboard {
    fn new_with_cutoff(cutoff: usize) -> Self {
        Scoreboard {
            recipes: vec![3, 7],
            elf1: 0,
            elf2: 1,
            cutoff: Some(cutoff + 10),
            target: None,
        }
    }

    fn new_with_target(desc: &str) -> Self {
        Scoreboard {
            recipes: vec![3, 7],
            elf1: 0,
            elf2: 1,
            cutoff: None,
            target: Some(desc.bytes().map(|b| b - b'0').collect()),
        }
    }

    fn ends_with_target(&self) -> bool {
        self.target.as_ref().map(|t| self.recipes.len() >= t.len() && *t == &self.recipes[self.recipes.len() - t.len()..]).unwrap_or(false)
    }

    fn step(&mut self) {
        let current_1 = self.recipes[self.elf1];
        let current_2 = self.recipes[self.elf2];
        let combi = current_1 + current_2;
        if combi > 9 {
            self.recipes.push(1);
            if self.cutoff.map(|c| self.recipes.len() == c).unwrap_or(false) {
                return;
            }

            if self.ends_with_target() {
                return;
            }
        }
        self.recipes.push(combi % 10);

        self.elf1 = (self.elf1 + (current_1 + 1) as usize) % self.recipes.len();
        self.elf2 = (self.elf2 + (current_2 + 1) as usize) % self.recipes.len();
    }

    fn last_10(&mut self) -> String {
        let cutoff = self.cutoff.unwrap();
        while self.recipes.len() < cutoff {
            self.step();
        }
        use std::iter::FromIterator;
        String::from_iter(
            self.recipes[self.recipes.len() - 10..]
                .iter()
                .map(|b| (b + b'0') as char),
        )
    }

    fn recipes_until_target(&mut self) -> usize {
        let tl = self.target.as_ref().unwrap().len();

        while !self.ends_with_target() {
            self.step();
        }

        self.recipes.len() - tl
    }
}

fn main() {
    println!("part 1: {}", Scoreboard::new_with_cutoff(INPUT).last_10());
    println!("part 2: {}", Scoreboard::new_with_target(&format!("{}", INPUT)).recipes_until_target());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_9() {
        assert_eq!(Scoreboard::new_with_cutoff(9).last_10(), "5158916779");
    }

    #[test]
    fn test_5() {
        assert_eq!(Scoreboard::new_with_cutoff(5).last_10(), "0124515891");
    }

    #[test]
    fn test_18() {
        assert_eq!(Scoreboard::new_with_cutoff(18).last_10(), "9251071085");
    }

    #[test]
    fn test_2018() {
        assert_eq!(Scoreboard::new_with_cutoff(2018).last_10(), "5941429882");
    }

    #[test]
    fn test_until_51589() {
        assert_eq!(Scoreboard::new_with_target("51589").recipes_until_target(), 9);
    }

    #[test]
    fn test_until_01245() {
        assert_eq!(Scoreboard::new_with_target("01245").recipes_until_target(), 5);
    }

    #[test]
    fn test_until_92510() {
        assert_eq!(Scoreboard::new_with_target("92510").recipes_until_target(), 18);
    }

    #[test]
    fn test_until_59414() {
        assert_eq!(Scoreboard::new_with_target("59414").recipes_until_target(), 2018);
    }
}
