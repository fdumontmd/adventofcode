use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> (&str, HashMap<&[u8], u8>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let template = parts[0];

    let mut insertions = HashMap::new();
    for line in parts[1].lines() {
        let rule: Vec<_> = line.split(" -> ").collect();
        let pair = rule[0].as_bytes();
        let inserted = rule[1].as_bytes()[0];
        insertions.insert(pair, inserted);
    }

    (template, insertions)
}

fn step(template: &str, rules: &HashMap<&[u8], u8>) -> String {
    let result: Vec<u8> = template
        .as_bytes()
        .windows(2)
        .flat_map(|pair| {
            let mut v = vec![pair[0]];
            if let Some(inserted) = rules.get(&pair) {
                v.push(*inserted);
            }
            v.into_iter()
        })
        .chain(template.bytes().last())
        .collect();

    String::from_utf8(result).unwrap()
}

fn part_1(input: &str) -> usize {
    let (template, rules) = parse_input(input);
    let mut template = template.to_string();

    for _s in 0..10 {
        template = step(&template, &rules);
    }

    let mut counts: HashMap<u8, usize> = HashMap::new();

    for byte in template.bytes() {
        *counts.entry(byte).or_default() += 1;
    }

    let (min, max) = match counts.values().minmax() {
        MinMaxResult::NoElements => panic!("empty template?"),
        MinMaxResult::OneElement(x) => (x, x),
        MinMaxResult::MinMax(min, max) => (min, max),
    };

    max - min
}

fn part_2(input: &str, rounds: usize) -> usize {
    let (template, rules) = parse_input(input);

    let mut pair_counts: HashMap<(u8, u8), usize> = HashMap::new();
    template.as_bytes().windows(2).for_each(|p| {
        *pair_counts.entry((p[0], p[1])).or_default() += 1;
    });

    let last = template.bytes().last().unwrap();

    for _s in 0..rounds {
        let mut tmp = HashMap::new();
        for (pair, count) in pair_counts {
            if let Some(c) = rules.get(&[pair.0, pair.1][..]) {
                *tmp.entry((pair.0, *c)).or_default() += count;
                *tmp.entry((*c, pair.1)).or_default() += count;
            } else {
                *tmp.entry(pair).or_default() += count;
            }
        }
        pair_counts = tmp;
        let total: usize = pair_counts.values().sum();
    }

    let mut component_counts: HashMap<u8, usize> = HashMap::new();

    let total: usize = pair_counts.values().sum();
    for (pair, count) in pair_counts {
        *component_counts.entry(pair.0).or_default() += count;
    }

    *component_counts.entry(last).or_default() += 1;

    let (min, max) = match component_counts.values().minmax() {
        MinMaxResult::NoElements => panic!("empty template?"),
        MinMaxResult::OneElement(x) => (x, x),
        MinMaxResult::MinMax(min, max) => (min, max),
    };
    max - min
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT, 40));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part_1() {
        assert_eq!(1588, part_1(TEST_INPUT));
        assert_eq!(2010, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1588, part_2(TEST_INPUT, 10));
        assert_eq!(2188189693529, part_2(TEST_INPUT, 40));
        assert_eq!(2437698971143, part_2(INPUT, 40));
    }
}
