use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");

fn parse_mask(input: &str) -> (u64, u64, Vec<u32>) {
    let (o, a, mut v) =
        input
            .bytes()
            .enumerate()
            .fold((0, 0, vec![]), |(o, a, mut v), (idx, b)| {
                if b == b'X' {
                    v.push(35 - idx as u32);
                }
                (
                    o << 1 | (b == b'1') as u64,
                    (a << 1 | (b != b'0') as u64),
                    v,
                )
            });
    v.reverse();
    (o, a, v)
}

enum Instruction {
    Mask(u64, u64, Vec<u32>),
    Mem(usize, u64),
}

fn parse_line(input: &str) -> Instruction {
    if let Some(mask) = input.strip_prefix("mask = ") {
        let (o, a, v) = parse_mask(mask);
        Instruction::Mask(o, a, v)
    } else {
        let Some(input) = input.strip_prefix("mem[") else { panic!("cannot parse {input}")};
        let input: Vec<_> = input.split("] = ").collect();
        Instruction::Mem(input[0].parse().unwrap(), input[1].parse().unwrap())
    }
}

fn part_1(input: &str) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = (0, 0);

    for line in input.lines() {
        match parse_line(line) {
            Instruction::Mask(o, a, _) => mask = (o, a),
            Instruction::Mem(m, v) => {
                memory.insert(m, (v | mask.0) & mask.1);
            }
        }
    }

    memory.values().sum()
}

fn generate_addresses(
    address: usize,
    mask: &(u64, u64, Vec<u32>),
) -> impl Iterator<Item = usize> + '_ {
    let address = address | mask.0 as usize;

    (0..(1 << mask.2.len())).map(move |m| {
        let mut o = 0;
        let mut a = (1 << 36) - 1;
        for (idx, shift) in mask.2.iter().enumerate() {
            if (m & (1 << idx)) != 0 {
                o |= 1 << shift;
            } else {
                a &= !(1 << shift);
            }
        }
        (address | o) & a
    })
}

fn part_2(input: &str) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = (0, 0, vec![]);

    for line in input.lines() {
        match parse_line(line) {
            Instruction::Mask(o, a, v) => mask = (o, a, v),
            Instruction::Mem(m, v) => {
                for address in generate_addresses(m, &mask) {
                    memory.insert(address, v);
                }
            }
        }
    }

    memory.values().sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{generate_addresses, parse_mask, part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_parse_mask() {
        assert_eq!(
            (
                0b1000000,
                0b111111111111111111111111111111111101,
                vec![
                    0, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35
                ]
            ),
            parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(165, part_1(TEST_INPUT));
        assert_eq!(8471403462063, part_1(INPUT));
    }

    #[test]
    fn test_generate_address() {
        let mask = parse_mask("000000000000000000000000000000X1001X");
        dbg!(&mask);
        assert_eq!(
            vec![26, 27, 58, 59],
            generate_addresses(42, &mask).collect::<Vec<_>>()
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            208,
            part_2(
                r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            )
        );
        assert_eq!(2667858637669, part_2(INPUT));
    }
}
