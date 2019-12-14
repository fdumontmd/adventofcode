use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");
const ORE_TOTAL: usize = 1000000000000;

struct Ingredient<'a>(usize, &'a str);

struct Recipe<'a> {
    target: Ingredient<'a>,
    source: Vec<Ingredient<'a>>,
}

struct Reactions<'a> {
    formulae: HashMap<&'a str, Recipe<'a>>,
    ingredients: Vec<&'a str>,
}

impl<'a> Reactions<'a> {
    fn new(desc: &'a str) -> Self {
        parse_reactions(desc)
    }

    fn with_capacity(&self, capacity: usize) -> usize {
        let mut low = 1;
        let mut high = 5000;

        while self.produce_total("FUEL", high) < capacity {
            high *= 2;
        }

        loop {
            if low + 1 >= high {
                return low;
            }

            let mid = low + (high - low) / 2;

            let cost = self.produce_total("FUEL", mid);
            if cost == capacity {
                return mid;
            }

            if cost < capacity {
                low = mid;
            } else {
                high = mid;
            }
        }
    }

    fn produce(&self, target: &str) -> usize {
        self.produce_total(target, 1)
    }

    fn produce_total(&self, target: &str, total: usize) -> usize {
        let mut ingredients = self.ingredients.clone();
        let mut counts = HashMap::new();
        counts.insert(target, total);

        let mut count;

        while let Some(ingredient) = ingredients.pop() {
            if let Some(c) = counts.get(ingredient) {
                count = *c;

                if let Some(recipe) = self.formulae.get(ingredient) {
                    let div = recipe.target.0;
                    let mut chunk = count / div;
                    if count % div != 0 {
                        chunk += 1;
                    }

                    for i in &recipe.source {
                        *counts.entry(i.1).or_default() += chunk * i.0;
                    }
                } else {
                    return count
                }
            }

        }

        unreachable!()
    }
}

fn parse_ingredient<'a>(line: &'a str) -> Ingredient<'a> {
    let line = line.trim();
    if let Some(pos) = line.find(' ') {
        Ingredient(line[0..pos].trim().parse().expect(&format!("not a number {}", &line[0..pos])), &line[pos + 1..].trim())
    } else {
        panic!(format!("cannot parse {} as ingredient", line));
    }
}

fn parse_recipe<'a>(line: &'a str) -> Recipe<'a> {
    if let Some(pos) = line.find("=>") {
        let target = &line[pos + 2..];
        let target = parse_ingredient(target);

        let mut source = Vec::new();
        let mut ingredients = &line[0..pos];

        while let Some(pos) = ingredients.find(',') {
            source.push(parse_ingredient(&ingredients[0..pos]));
            ingredients = &ingredients[pos + 1..];
        }

        source.push(parse_ingredient(ingredients));

        Recipe { target, source }
    } else {
        panic!(format!("cannot find => in {}", line));
    }
}

fn parse_reactions<'a>(desc: &'a str) -> Reactions<'a> {
    let mut formulae = HashMap::new();

    for line in desc.lines() {
        let recipe = parse_recipe(line);
        formulae.insert(recipe.target.1, recipe);
    }

    let ingredients = order_targets(&formulae);
    Reactions { formulae, ingredients, }
}

struct TopologicalSort<'a, 'b> {
    targets: Vec<&'a str>,
    map: &'b HashMap<&'a str, Recipe<'a>>,
    permanent: HashSet<&'a str>,
    temporary: HashSet<&'a str>,
}

impl<'a, 'b> TopologicalSort<'a, 'b> {
    fn sort(map: &'b HashMap<&'a str, Recipe<'a>>) -> Vec<&'a str> {
        let mut ts = TopologicalSort {
            targets: Vec::new(),
            map,
            permanent: HashSet::new(),
            temporary: HashSet::new(),
        };

        for n in ts.map.keys() {
            ts.visit(n);
        }

        ts.targets
    }

    fn visit(&mut self, n: &'a str) {
        if self.permanent.contains(n) {
            return;
        }
        if self.temporary.contains(n) {
            panic!(format!("loop detected in formulae: {}", n));
        }
        self.temporary.insert(n);

        self.map.get(n).map(|r| {
            r.source.iter().for_each(|i| {
                self.visit(i.1);
            });
        });

        self.temporary.remove(n);
        self.permanent.insert(n);
        self.targets.push(n);
    }
}

fn order_targets<'a>(map: &HashMap<&'a str, Recipe<'a>>) -> Vec<&'a str> {
    TopologicalSort::sort(map)
}

fn part_1() -> usize {
    Reactions::new(INPUT).produce("FUEL")
}

fn part_2() -> usize {
    Reactions::new(INPUT).with_capacity(ORE_TOTAL)
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_1: &str = r#"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"#;

    #[test]
    fn test_1() {
        assert_eq!(Reactions::new(TEST_1).produce("FUEL"), 31);
    }

    static TEST_2: &str = r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#;

    #[test]
    fn test_2() {
        assert_eq!(Reactions::new(TEST_2).produce("FUEL"), 165);
    }

    static TEST_3: &str = r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#;

    #[test]
    fn test_3() {
        assert_eq!(Reactions::new(TEST_3).produce("FUEL"), 13312);
    }

    static TEST_4: &str = r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"#;

    #[test]
    fn test_4() {
        assert_eq!(Reactions::new(TEST_4).produce("FUEL"), 180697);
    }

    static TEST_5: &str = r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"#;

    #[test]
    fn test_5() {
        assert_eq!(Reactions::new(TEST_5).produce("FUEL"), 2210736);
    }

    #[test]
    fn total_fuel_3() {
        assert_eq!(Reactions::new(TEST_3).with_capacity(ORE_TOTAL), 82892753);
    }

    #[test]
    fn total_fuel_4() {
        assert_eq!(Reactions::new(TEST_4).with_capacity(ORE_TOTAL), 5586022);
    }

    #[test]
    fn total_fuel_5() {
        assert_eq!(Reactions::new(TEST_5).with_capacity(ORE_TOTAL), 460664);
    }
}
