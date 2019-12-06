use std::collections::{HashMap, HashSet, VecDeque};

fn build_orbits(desc: &'static str) -> HashMap<&'static str, HashSet<&'static str>> {
    let mut orbits = HashMap::new();
    for line in desc.lines() {
        if let Some(pos) = line.find(')') {
            let planet = &line[..pos];
            let moon = &line[pos+1..];
            orbits.entry(planet).or_insert(HashSet::new()).insert(moon);
        } else {
            panic!("cannot parse {}", line);
        }
    }
    orbits
}

fn compute_depth(orbits: &HashMap<&'static str, HashSet<&'static str>>) -> HashMap<&'static str, usize> {
    let mut level = 0;
    let mut levels = HashMap::new();
    let mut moons = HashSet::new();

    levels.insert(COM, level);

    orbits.get(COM).map(|m| moons.extend(m.iter()));

    while !moons.is_empty() {
        level += 1;
        let mut next_moons = HashSet::new();
        for moon in moons  {
            if levels.insert(moon, level).is_some() {
                panic!("{} orbits multiple planets", moon);
            }
            orbits.get(moon).map(|m| next_moons.extend(m.iter()));
        }
        moons = next_moons;
    }

    levels
}

fn sum_orbits(levels: &HashMap<&'static str, usize>) -> usize {
    levels.iter().map(|(_, v)| v).sum()
}

fn part_1(desc: &'static str) -> usize {
    let orbits = build_orbits(desc);
    let levels = compute_depth(&orbits);
    sum_orbits(&levels)
}

fn build_paths(desc: &'static str) -> HashMap<&'static str, Vec<&'static str>> {
    let orbits = build_orbits(desc);
    let mut paths = HashMap::new();

    paths.insert(COM, Vec::new());
    let mut planets = VecDeque::new();
    planets.push_back(COM);

    while let Some(planet) = planets.pop_front() {
        orbits.get(planet).map(|moons| {
            for moon in moons {
                let mut path = paths.get(planet).expect("planet not found").clone();
                path.push(planet);
                paths.insert(moon, path);
                planets.push_back(moon);
            }
        });
    }
    paths
}

fn compute_transers(from: &'static str, to: &'static str, paths: &HashMap<&'static str, Vec<&'static str>>) -> usize {
    let from_path = paths.get(from).expect("from path");
    let to_path = paths.get(to).expect("to path");

    let common_prefix = from_path.iter().zip(to_path.iter()).filter(|(f, t)| f == t).count();
    from_path.len() + to_path.len() - 2 * common_prefix
}

fn part_2(desc: &'static str) -> usize {
    let paths = build_paths(desc);
    compute_transers("YOU", "SAN", &paths)
}

static INPUT: &str = include_str!("input.txt");
static COM: &str = "COM";

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;

    static TEST_2: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 42);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_2), 4);
    }
}
