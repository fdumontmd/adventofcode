use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Rugsack {
    left: HashSet<u8>,
    right: HashSet<u8>,
}

fn priority(item: u8) -> usize {
    (match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("Unknown item: {}", item),
    }) as usize
}

impl Rugsack {
    fn from_str(input: &str) -> Self {
        let input = input.trim();
        assert!(input.len() % 2 == 0);
        Rugsack {
            left: HashSet::from_iter(input[..input.len() / 2].bytes()),
            right: HashSet::from_iter(input[input.len() / 2..].bytes()),
        }
    }

    fn shared(&self) -> impl Iterator<Item = u8> + '_ {
        self.left.intersection(&self.right).cloned()
    }

    fn shared_priority(&self) -> usize {
        self.shared().map(priority).sum()
    }

    fn items(&self) -> HashSet<u8> {
        self.left.union(&self.right).cloned().collect()
    }
}

fn parse(input: &str) -> Vec<Rugsack> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Rugsack::from_str)
        .collect()
}

fn total_shared_priority(input: &[Rugsack]) -> usize {
    input.iter().map(|r| r.shared_priority()).sum()
}

fn badges(rugsacks: &[Rugsack]) -> impl Iterator<Item = u8> + '_ {
    rugsacks.chunks(3).map(|group| {
        if let [r0, r1, r2] = group {
            let mut shared: HashSet<_> = r0.items().intersection(&r1.items()).cloned().collect();
            shared = shared.intersection(&r2.items()).cloned().collect();
            shared.into_iter().next().unwrap()
        } else {
            panic!("incomplete group {:?}", group);
        }
    })
}

fn total_badge_priority(rugsacks: &[Rugsack]) -> usize {
    badges(rugsacks).map(priority).sum()
}

fn main() {
    let rugsacks = parse(INPUT);
    println!(
        "Part 1: sum of priorities of all shared items: {}",
        total_shared_priority(&rugsacks)
    );
    println!(
        "Part 2: sum of priorities of badges: {}",
        total_badge_priority(&rugsacks)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_part_1() {
        let rugsacks = parse(TEST_INPUT);
        assert_eq!(157, total_shared_priority(&rugsacks));
    }

    #[test]
    fn test_part_2() {
        let rugsacks = parse(TEST_INPUT);
        assert_eq!(70, total_badge_priority(&rugsacks));
    }

    #[test]
    fn real_part_1() {
        let rugsacks = parse(INPUT);
        assert_eq!(7826, total_shared_priority(&rugsacks));
    }

    #[test]
    fn real_part_2() {
        let rugsacks = parse(INPUT);
        assert_eq!(2577, total_badge_priority(&rugsacks));
    }
}
