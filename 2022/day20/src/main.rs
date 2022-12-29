use aoc_utils::ring::Ring;

static INPUT: &str = include_str!("input.txt");
static KEY: isize = 811589153;

fn parse(input: &str, key: isize) -> Ring<(isize, usize)> {
    let mut r = Ring::new();
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|n| n.parse::<isize>().unwrap())
        .enumerate()
        .for_each(|(idx, n)| {
            r.insert((n * key, idx));
            r.move_right(1);
        });

    r
}

fn mix(ring: &mut Ring<(isize, usize)>) {
    for idx in 0..ring.len() {
        while ring.get(0).map(|&(_, pos)| pos).unwrap() != idx {
            ring.move_right(1);
        }
        if let Some((n, pos)) = ring.remove() {
            if idx != pos {
                panic!(
                    "Out of order: looking for {} but found {} instead",
                    idx, pos
                );
            }
            ring.move_signed(n);
            ring.insert((n, pos));
            ring.move_signed(-n);
        }
    }
}

fn decrypt(input: &str, key: isize, rounds: usize) -> isize {
    let mut ring = parse(input, key);
    for _ in 0..rounds {
        mix(&mut ring);
    }
    let v = ring.into_inner();
    if let Some(zero) = v.iter().position(|&(n, _)| n == 0) {
        v[(zero + 1000) % v.len()].0 + v[(zero + 2000) % v.len()].0 + v[(zero + 3000) % v.len()].0
    } else {
        panic!("0 not found in data");
    }
}

fn part_01(input: &str) -> isize {
    decrypt(input, 1, 1)
}

fn part_02(input: &str) -> isize {
    decrypt(input, KEY, 10)
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02};

    static TEST_INPUT: &str = r"1
2
-3
3
-2
0
4";

    #[test]
    fn test_part_01() {
        assert_eq!(3, part_01(TEST_INPUT));
    }

    #[test]
    fn test_part_02() {
        assert_eq!(1623178306, part_02(TEST_INPUT));
    }
}
