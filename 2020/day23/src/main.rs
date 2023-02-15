use aoc_utils::ring::Ring;
use itertools::Itertools;
static INPUT: &str = "952438716";

// slow implementation for part 1
fn make_ring<T: From<u8>>(input: &str) -> Ring<T> {
    let mut ring = Ring::new();
    input.bytes().for_each(|b| {
        ring.insert((b - b'0').into());
        ring.move_right(1);
    });

    ring
}

fn one_move(ring: &mut Ring<u64>) {
    let cur = *ring.get(0).unwrap();

    let mut tmp = vec![];
    ring.move_right(1);
    tmp.push(ring.remove().unwrap());
    tmp.push(ring.remove().unwrap());
    tmp.push(ring.remove().unwrap());

    let mut next = if cur == 1 {
        ring.len() as u64 + 3
    } else {
        cur - 1
    };
    while tmp.contains(&next) {
        next = if next == 1 {
            ring.len() as u64 + 3
        } else {
            next - 1
        };
    }

    let mut steps = 0;

    while *ring.get(0).unwrap() != next {
        ring.move_right(1);
        steps += 1;
    }

    // move right of target
    ring.move_right(1);
    steps += 1;

    ring.insert(tmp.pop().unwrap());
    ring.insert(tmp.pop().unwrap());
    ring.insert(tmp.pop().unwrap());

    ring.move_left(steps);
}

fn part_1(input: &str) -> u64 {
    let mut ring = make_ring(input);
    for _ in 0..100 {
        one_move(&mut ring);
    }

    while *ring.get(0).unwrap() != 1 {
        ring.move_right(1);
    }

    ring.iter().skip(1).fold(0u64, |s, b| s * 10 + *b)
}

fn part_2(input: &str) -> usize {
    // reddit solution: next[cup] = next_cup
    // the one thing to reach for when you have a need
    // for circular buffers and loads of link tweaking
    let input: Vec<_> = input.bytes().map(|b| (b - b'0') as usize - 1).collect();
    let first = input[0];
    let next_max = input.iter().cloned().max().unwrap() + 1;

    let mut next = vec![0; 1_000_000];

    for (p, n) in input
        .iter()
        .cloned()
        .chain((next_max..1_000_000).into_iter())
        .tuple_windows()
    {
        next[p] = n;
    }
    let last = next.len() - 1;
    next[last] = first;

    const ROUNDS: usize = 10_000_000;
    let mut cur = first;

    for _ in 0..ROUNDS {
        let tmp1 = next[cur];
        let tmp2 = next[tmp1];
        let tmp3 = next[tmp2];
        next[cur] = next[tmp3];

        let mut prev = if cur == 0 { 1_000_000 - 1 } else { cur - 1 };
        while tmp1 == prev || tmp2 == prev || tmp3 == prev {
            prev = if prev == 0 { 1_000_000 - 1 } else { prev - 1 };
        }
        next[tmp3] = next[prev];
        next[prev] = tmp1;
        cur = next[cur];
    }

    (next[0] + 1) * (next[next[0]] + 1)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = "389125467";

    #[test]
    fn test_part_1() {
        assert_eq!(67384529, part_1(TEST_INPUT));
        assert_eq!(97342568, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(149245887792, part_2(TEST_INPUT));
        assert_eq!(902208073192, part_2(INPUT));
    }
}
