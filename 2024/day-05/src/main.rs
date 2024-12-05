use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

// part 1: brute force
// parse the first part to create a map of prior->[successors],
// then scan each of the lists and check for each item if there's
// one of that item's successors in the slice before the item
//
// part 2: dumb topological sort

type Successors = HashMap<usize, Vec<usize>>;

fn parse(input: &str) -> (Successors, Vec<Vec<usize>>) {
    let mut successors = HashMap::new();
    let mut sequences = Vec::new();

    let mut rules = true;

    for line in input.lines() {
        if line.is_empty() {
            rules = false;
            continue;
        }

        if rules {
            let rule: Vec<usize> = line.split('|').map(|n| n.parse().unwrap()).collect();
            successors
                .entry(rule[0])
                .or_insert(Vec::new())
                .push(rule[1]);
        } else {
            sequences.push(line.split(',').map(|n| n.parse().unwrap()).collect());
        }
    }

    (successors, sequences)
}

fn is_sorted(successors: &Successors, sequence: &[usize]) -> bool {
    for (idx, elt) in sequence.iter().enumerate() {
        if successors.contains_key(elt) {
            for succ in &successors[elt] {
                if sequence[0..idx].contains(succ) {
                    return false;
                }
            }
        }
    }
    true
}

fn part1(input: &str) -> usize {
    let (successors, sequences) = parse(input);

    let mut sum = 0;

    for seq in sequences {
        if is_sorted(&successors, &seq) {
            sum += seq[seq.len() / 2];
        }
    }

    sum
}

fn sort(successors: &Successors, mut sequence: Vec<usize>) -> Vec<usize> {
    // dumbest topological sort...
    let mut successors = successors.clone();

    // remove keys not used
    let keys: Vec<usize> = successors.keys().cloned().collect();
    for key in keys {
        if !sequence.contains(&key) {
            successors.remove(&key);
        } else {
            successors
                .get_mut(&key)
                .unwrap()
                .retain(|succ| sequence.contains(succ));
        }
    }

    let mut new_seq = Vec::new();

    while !sequence.is_empty() {
        // assume all items are different; turns out correct
        for elt in &sequence {
            if !successors.values().any(|succs| succs.contains(elt)) {
                new_seq.push(*elt);
                successors.remove(elt);
            }
        }
        sequence.retain(|elt| !new_seq.contains(elt));
    }

    new_seq
}

fn part2(input: &str) -> usize {
    let (successors, sequences) = parse(input);

    let mut sum = 0;

    for mut seq in sequences {
        if !is_sorted(&successors, &seq) {
            seq = sort(&successors, seq);
            sum += seq[seq.len() / 2];
        }
    }

    sum
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test_case(TEST_INPUT, 143; "test input")]
    #[test_case(INPUT, 5275; "input")]
    fn test_part1(input: &str, sum: usize) {
        assert_eq!(sum, part1(input));
    }

    #[test_case(TEST_INPUT, 123; "test input")]
    #[test_case(INPUT, 6191; "input")]
    fn test_part2(input: &str, sum: usize) {
        assert_eq!(sum, part2(input));
    }
}
