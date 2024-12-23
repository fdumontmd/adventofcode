use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");
type U = i64;

const PRUNE: U = 16777216;
/*
* evolve_secret is quite slow; the solution is to convert all operatations to
* shift as all constants are powers of 2
*
* * 64 is << 6
* / 32 is >> 5
* * 2028 is << 11
* and 16777216 = 2**20, so % 16777216 is same as & (2**24 - 1) == & 0xFFFFFF;
*
* moreover, secrets start small enough to be u32, and will stay there because of
* prune operation
*/

fn evolve_secret(s: u32) -> u32 {
    let a = (s ^ (s << 6)) & 0xFFFFFF;
    let b = a ^ (a >> 5);
    (b ^ (b << 11)) & 0xFFFFFF
}

struct Secret(u32);

impl Iterator for Secret {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.0;
        self.0 = evolve_secret(s);
        Some(s)
    }
}

fn part1(input: &str) -> U {
    input
        .lines()
        .map(|l| Secret(l.parse().unwrap()))
        .map(|s| s.into_iter().nth(2000).unwrap() as i64)
        .sum()
}

impl Secret {
    fn prices(self) -> impl Iterator<Item = U> {
        self.into_iter().map(|s| (s % 10) as i64)
    }
}

fn index_prices(secret: u32, map: &mut HashMap<[i64; 4], i64>) {
    let prices: Vec<i64> = Secret(secret).prices().take(2001).collect();
    let delta: Vec<i64> = prices.windows(2).map(|w| w[1] - w[0]).collect();
    let mut seen = HashSet::new();
    for (ds, p) in delta.windows(4).zip(prices.into_iter().skip(4)) {
        if seen.contains(ds) {
            continue;
        }
        seen.insert(ds);
        let ds = [ds[0], ds[1], ds[2], ds[3]];
        *map.entry(ds).or_insert(0) += p;
    }
}

// approach: brute force it
// all the numbers used in the evolution of the secret
// are powers of 2, so there must be some cycle that should cut the
// processing down, but brute force it still kind of fast enough
//
// seems that brute force is the way, but a smarter brute could check
// https://github.com/ndunnett/aoc/blob/main/rust/2024/src/bin/day22.rs
//
// ideas: the bit shifting and masking is faster for evolve_secret
//        use the fact that (-9..=9).len() is 19, so can be encoded on 5 bits
//        so we can use a 20 bit number to represent a sequence of differences
//        with  << 5 & 0xFFFFF to shift the differences left and drop the oldest
//        one
//        a 20 digit number can be used as index in an array with length 0xFFFFF
//        finally using the seen[deltas] = secret is a clever way to reuse seen
fn part2(input: &str) -> U {
    let secrets: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut map = HashMap::new();

    for secret in secrets {
        index_prices(secret, &mut map);
    }

    *map.values().max().unwrap()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "1
10
100
2024";

    #[test_case(TEST_INPUT, 37327623; "test input")]
    #[test_case(INPUT, 14726157693; "input")]
    fn test_part1(input: &str, secret_sum: U) {
        assert_eq!(secret_sum, part1(input));
    }

    const TEST_INPUT_2: &str = "1
2
3
2024";

    #[test_case(TEST_INPUT_2, 23; "test input")]
    #[test_case(INPUT, 1614; "input")]
    fn test_part2(input: &str, bananas: U) {
        assert_eq!(bananas, part2(input));
    }
}
