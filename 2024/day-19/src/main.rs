use std::collections::BTreeSet;

const INPUT: &str = include_str!("input.txt");

// reuse solution count from part2 as it is actually faster
fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut towels: Vec<&str> = vec![];

    for design in &mut lines {
        if design.is_empty() {
            break;
        }
        towels.extend(design.split(", "));
    }

    lines
        .filter(|pattern| count_solutions(pattern, &towels) > 0)
        .count()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut towels: Vec<&str> = vec![];

    for design in &mut lines {
        if design.is_empty() {
            break;
        }
        towels.extend(design.split(", "));
    }

    lines.map(|pattern| count_solutions(pattern, &towels)).sum()
}

// dynamic programming: compute number of covers for suffixes
// patterns by working from the end, and for each towel
// extending a suffix, we increase the number of solution for
// the new suffix (towel + old suffix) by the number of solutions
// for the old suffix
//
// we start by setting the empty suffix number of solution to 1 (
// as an empty string can be covered in one way by no towel)
//
// then work down by considering each solved suffix only once, ordered
// by len (so we work down to the beginning of the pattern)
//
// pattern_covers accumulate the number of solutions for each suffixes
// of pattern. it is filled from the end
//
// once we cannot add any suffix, we return the number of
// solutions for the first element, which if non zero is the number
// of possible covers for the entire pattern
fn count_solutions(pattern: &str, towels: &[&str]) -> usize {
    let mut pattern_covers = vec![0; pattern.len() + 1];
    pattern_covers[pattern.len()] = 1;
    let mut queue = BTreeSet::new();
    queue.insert(pattern.len());

    while let Some(&end) = queue.last() {
        queue.remove(&end);
        for t in towels {
            if let Some(prefix) = &pattern[0..end].strip_suffix(t) {
                pattern_covers[prefix.len()] += pattern_covers[end];
                if !prefix.is_empty() {
                    queue.insert(prefix.len());
                }
            }
        }
    }

    pattern_covers[0]
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test_case(TEST_INPUT, 6; "test input")]
    #[test_case(INPUT, 369; "input")]
    fn test_part1(input: &str, patterns: usize) {
        assert_eq!(patterns, part1(input));
    }

    #[test_case(TEST_INPUT, 16; "test input")]
    #[test_case(INPUT, 761826581538190; "input")]
    fn test_part2(input: &str, count: usize) {
        assert_eq!(count, part2(input));
    }
}
