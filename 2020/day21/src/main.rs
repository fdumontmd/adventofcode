use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

static INPUT: &str = include_str!("input.txt");

struct Input<'a> {
    all_safe_ingredients: BTreeSet<&'a str>,
    all_ingredients: Vec<BTreeSet<&'a str>>,
    allergens: BTreeMap<&'a str, BTreeSet<&'a str>>,
}

impl<'a> Input<'a> {
    // is it the most beautiful code? No. But is it fast? Probably not. But was it fun to
    // write? Also no
    fn parse_str(input: &'a str) -> Self {
        let mut map = BTreeMap::new();
        let mut allergens = BTreeMap::new();
        let mut all_ingredients = Vec::new();
        input.lines().for_each(|l| {
            let Some(l) = l.strip_suffix(')') else { panic!("line format not valid; no ending ): {l}")};
            let parts: Vec<_> = l.split(" (contains ").collect();
            let ingredients: BTreeSet<_> = parts[0].split_whitespace().collect();
            let idx = all_ingredients.len();
            all_ingredients.push(ingredients);

            parts[1]
                .split(", ")
                .for_each(|a| map.entry(a).or_insert(vec![]).push(idx));
        });

        let mut all_safe_ingredients =
            BTreeSet::from_iter(all_ingredients.iter().flat_map(|l| l.iter().cloned()));

        for (a, indices) in map {
            let s: BTreeSet<&str> = all_ingredients[*indices.first().unwrap()].clone();
            let common_ingredients: BTreeSet<&str> = indices.iter().fold(s, |s, i| {
                s.intersection(&all_ingredients[*i])
                    .cloned()
                    .collect::<BTreeSet<&str>>()
            });

            all_safe_ingredients.retain(|i| !common_ingredients.contains(i));
            allergens.insert(a, common_ingredients);
        }

        Self {
            all_safe_ingredients,
            all_ingredients,
            allergens,
        }
    }
}

fn part_1(input: &str) -> usize {
    let input = Input::parse_str(input);

    input
        .all_ingredients
        .iter()
        .map(|ai| {
            ai.iter()
                .filter(|i| input.all_safe_ingredients.contains(*i))
                .count()
        })
        .sum()
}

fn part_2(input: &str) -> String {
    let mut input = Input::parse_str(input);

    let mut allergens = BTreeMap::new();

    while !input.allergens.is_empty() {
        let mut single = None;

        for (&a, v) in &input.allergens {
            if let Some(&first) = v.first() {
                if v.len() == 1 {
                    allergens.insert(a, first);
                    single = Some(a);
                    break;
                }
            }
        }

        if let Some((a, ai)) = single.and_then(|a| allergens.get(a).map(|ai| (a, ai))) {
            input.allergens.remove(a);
            for v in input.allergens.values_mut() {
                v.retain(|i| i != ai);
            }
        } else {
            panic!("no single! need to search");
        }
    }

    allergens.values().join(",")
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part_1() {
        assert_eq!(5, part_1(TEST_INPUT));
        assert_eq!(2078, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl", part_2(TEST_INPUT));
        assert_eq!(
            "lmcqt,kcddk,npxrdnd,cfb,ldkt,fqpt,jtfmtpd,tsch",
            part_2(INPUT)
        );
    }
}
