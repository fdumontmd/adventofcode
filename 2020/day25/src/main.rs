static INPUT: &str = include_str!("input.txt");

const MODULUS: u64 = 20201227;

struct Problem {
    card_public_key: u64,
    door_public_key: u64,
}

impl Problem {
    fn from_str(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        Self {
            card_public_key: lines[0].parse().unwrap(),
            door_public_key: lines[1].parse().unwrap(),
        }
    }
}

fn exp_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut val = 1;

    while exp != 0 {
        if exp % 2 == 1 {
            val = (val * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    val
}

fn find_secret_loop(public_key: u64) -> u64 {
    let Some(l) = power_mod(7, MODULUS).position(|e| e == public_key) else { panic!("ran out of numbers?")};
    let l = l as u64;

    assert_eq!(public_key, exp_mod(7, l, MODULUS));
    l
}

fn power_mod(base: u64, modulus: u64) -> impl Iterator<Item = u64> {
    [1].into_iter().chain((0..).scan(1, move |state, _| {
        *state = (*state * base) % modulus;
        Some(*state)
    }))
}

fn part(input: &str) -> u64 {
    let p = Problem::from_str(input);
    let l1 = find_secret_loop(p.card_public_key);
    let l2 = find_secret_loop(p.door_public_key);

    let e1 = exp_mod(p.card_public_key, l2, MODULUS);
    let e2 = exp_mod(p.door_public_key, l1, MODULUS);

    assert_eq!(e1, e2);

    e1
}

fn main() {
    println!("Part: {}", part(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{find_secret_loop, part, INPUT};

    #[test]
    fn test_find_secret_loop() {
        assert_eq!(8, find_secret_loop(5764801));
        assert_eq!(11, find_secret_loop(17807724));
    }

    #[test]
    fn test_part() {
        assert_eq!(
            14897079,
            part(
                r"5764801
17807724"
            )
        );
        assert_eq!(448851, part(INPUT));
    }
}
