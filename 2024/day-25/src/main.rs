const INPUT: &str = include_str!("input.txt");

struct Key([u8; 5]);
struct Lock([u8; 5]);

impl Key {
    fn matches(&self, l: &Lock) -> bool {
        self.0
            .into_iter()
            .zip(l.0)
            .map(|(k, l)| k + l)
            .all(|s| s <= 7)
    }
}

enum Part {
    Key(Key),
    Lock(Lock),
}

impl Part {
    fn parse(input: &str) -> Self {
        if input.as_bytes()[0] == b'#' {
            // lock
            let mut lock = [0; 5];
            for line in input.lines() {
                for (col, b) in line.bytes().enumerate() {
                    if b == b'#' {
                        lock[col] += 1;
                    }
                }
            }
            Part::Lock(Lock(lock))
        } else {
            // key
            let mut key = [7; 5];
            for line in input.lines() {
                for (col, b) in line.bytes().enumerate() {
                    if b == b'.' {
                        key[col] -= 1;
                    }
                }
            }
            Part::Key(Key(key))
        }
    }

    fn into_key(self) -> Key {
        match self {
            Part::Key(key) => key,
            Part::Lock(_lock) => panic!("not a key"),
        }
    }

    fn into_lock(self) -> Lock {
        match self {
            Part::Key(_key) => panic!("not a lock"),
            Part::Lock(lock) => lock,
        }
    }
}

fn part1(input: &str) -> usize {
    let parts: Vec<Part> = input.split("\n\n").map(Part::parse).collect();

    let (keys, locks): (Vec<_>, Vec<_>) =
        parts.into_iter().partition(|p| matches!(p, Part::Key(_)));

    let keys: Vec<Key> = keys.into_iter().map(|k| k.into_key()).collect();
    let locks: Vec<Lock> = locks.into_iter().map(|l| l.into_lock()).collect();

    keys.iter()
        .map(|k| locks.iter().filter(|l| k.matches(l)).count())
        .sum()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test_case(TEST_INPUT, 3; "test input")]
    #[test_case(INPUT, 3663; "input")]
    fn test_part1(input: &str, pairs: usize) {
        assert_eq!(pairs, part1(input));
    }
}
