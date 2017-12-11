extern crate regex;

const ELECTRON: &'static str = "e";

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

use regex::Regex;

type Replacements<'s> = HashMap<&'s str, Vec<&'s str>>;

fn generate_from(molecule: &str, replacements: &Replacements) -> HashSet<String> {
    let mut molecules = HashSet::new();
    let molecule = molecule.trim();
    for (k, l) in replacements {
        for (pos, _) in molecule.match_indices(k) {
            for replacement in l {
                let mut new_molecule = String::new();
                new_molecule.push_str(&molecule[..pos]);
                new_molecule.push_str(replacement);
                new_molecule.push_str(&molecule[pos + k.len()..]);
                molecules.insert(new_molecule);
            }
        }
    }
    molecules
}

fn build_reverse_replacements<'s>(replacements: &Replacements<'s>) -> HashMap<&'s str, &'s str> {
    let mut reverse = HashMap::new();

    for (k, l) in replacements {
        for r in l {
            assert!(!reverse.get(r).is_some());
            reverse.insert(*r, *k);
        }
    }

    reverse
}

fn ancestors(molecule: &str, reverse: &HashMap<&str, &str>) -> Vec<String> {
    let mut ancestors = Vec::new();
    for (r, k) in reverse {
        if *k == ELECTRON && *r != molecule {
            continue;
        }
        for (pos, _) in molecule.match_indices(r) {
            let mut ancestor = String::new();
            ancestor.push_str(&molecule[..pos]);
            ancestor.push_str(*k);
            ancestor.push_str(&molecule[pos+r.len()..]);
            ancestors.push(ancestor);
        }
    }
    ancestors
}

struct Search<'s> {
    reverse: HashMap<&'s str, &'s str>,
    fringe: BinaryHeap<(isize, usize, String)>,
}

impl<'s> Search<'s> {
    fn new(molecule: &'s str, replacements: &Replacements<'s>) -> Self {
        let mut fringe = BinaryHeap::new();
        fringe.push((-(molecule.len() as isize), 0, String::from(molecule)));
        Search{ reverse: build_reverse_replacements(replacements), fringe: fringe }
    }
}

impl<'s> Iterator for Search<'s> {
    type Item = (usize, String);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.fringe.pop() {
            for a in ancestors(&current.2, &self.reverse) {
                self.fringe.push((-(a.len() as isize), current.1 + 1, a));
            }

            Some((current.1, current.2))
        } else {
            None
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(\w+) => (\w+)").unwrap();

    let mut replacements = Replacements::new();

    for line in buffer.lines() {
        if let Some(ref caps) = re.captures(line) {
            replacements.entry(caps.at(1).unwrap()).or_insert(Vec::new()).push(caps.at(2).unwrap());
        } else {
            if !line.is_empty() {
                let molecule = line.trim();
                let molecules = generate_from(molecule, &replacements);

                println!("Total number of possible molecules: {}", molecules.len());

                let search = Search::new(molecule, &replacements);

                let mut min_depth = None;

                for (depth, source) in search {
                    if &source == ELECTRON {
                        if min_depth.is_none() || min_depth.unwrap() > depth {
                            min_depth = Some(depth);

                            println!("Steps: {}; looking for better", depth);
                        }
                    }
                }

                break;
            }
        }
    }
}

fn depth(molecule: &str, replacements: &Replacements) -> usize {
    let search = Search::new(molecule, &replacements);

    for (depth, source) in search {
        if &source == ELECTRON {
            return depth;
        }
    }

    unreachable!()
}

#[test]
fn test() {
    let mut replacements = Replacements::new();

    replacements.entry("e").or_insert(Vec::new()).push("H");
    replacements.entry("e").or_insert(Vec::new()).push("O");
    replacements.entry("H").or_insert(Vec::new()).push("HO");
    replacements.entry("H").or_insert(Vec::new()).push("OH");
    replacements.entry("O").or_insert(Vec::new()).push("HH");

    assert_eq!(3, depth("HOH", &replacements));

    assert_eq!(6, depth("HOHOHO", &replacements));

}
