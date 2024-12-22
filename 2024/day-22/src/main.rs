use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");
type U = i64;

const PRUNE: U = 16777216;

fn mix(n: U, s: U) -> U {
    n ^ s
}

fn prune(s: U) -> U {
    s.rem_euclid(PRUNE)
}

fn evolve_secret(mut s: U) -> U {
    s = prune(mix(s * 64, s));
    s = prune(mix(s / 32, s));
    prune(mix(s * 2048, s))
}

struct Secret(U);

impl Iterator for Secret {
    type Item = U;

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
        .map(|s| s.into_iter().nth(2000).unwrap())
        .sum()
}

impl Secret {
    fn prices(self) -> impl Iterator<Item = U> {
        self.into_iter().map(|s| s % 10)
    }
}

fn index_prices(secret: i64, map: &mut HashMap<[i64; 4], i64>) {
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
fn part2(input: &str) -> U {
    let secrets: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();

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
