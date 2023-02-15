use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input");

fn part01(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split('|')
                .nth(1)
                .expect(&format!("cannot parse line {}", l))
                .split_whitespace()
                .filter(|n| n.len() == 2 || n.len() == 3 || n.len() == 4 || n.len() == 7)
                .count()
        })
        .sum()
}

fn shared_segments(s1: &str, s2: &str) -> usize {
    let s1: HashSet<u8> = HashSet::from_iter(s1.bytes());
    let s2 = HashSet::from_iter(s2.bytes());
    s1.intersection(&s2).count()
}

fn sort_segment(s: &str) -> String {
    let mut s: Vec<u8> = s.bytes().collect();
    s.sort();
    String::from_utf8(s).unwrap()
}

fn decode_digits(input: &str) -> u64 {
    // start from len:
    // 2 -> 1
    // 3 -> 7
    // 4 -> 4
    // 6 -> 8
    // next are 5 letter segments:
    // 2, 3, 5
    // 3 intersect 1 == 2
    // 2 intersect 4 == 2
    // otherwise, 5
    // next are 6 letter segments:
    // 0, 6, 9
    // 6 intersect 1 == 1
    // 9 intersect 3 == 5
    // otherwise 0

    // need to sort all the entries
    let parts: Vec<_> = input.split(" | ").collect();
    let mut samples: HashSet<_> = parts[0].split_whitespace().map(sort_segment).collect();
    let digits: Vec<_> = parts[1].split_whitespace().map(sort_segment).collect();

    let mut mapping: HashMap<String, u64> = HashMap::new();
    let one = samples.iter().cloned().find(|s| s.len() == 2).unwrap();
    let seven = samples.iter().cloned().find(|s| s.len() == 3).unwrap();
    let four = samples.iter().cloned().find(|s| s.len() == 4).unwrap();
    let eight = samples.iter().cloned().find(|s| s.len() == 7).unwrap();

    samples.remove(&one);
    samples.remove(&seven);
    samples.remove(&four);
    samples.remove(&eight);

    mapping.insert(one.clone(), 1);
    mapping.insert(seven, 7);
    mapping.insert(four.clone(), 4);
    mapping.insert(eight, 8);

    let mut five_letters: HashSet<_> = samples.iter().cloned().filter(|s| s.len() == 5).collect();
    assert_eq!(3, five_letters.len());

    let three = five_letters
        .iter()
        .cloned()
        .find(|s| shared_segments(s, &one) == 2)
        .unwrap();
    let two = five_letters
        .iter()
        .cloned()
        .find(|s| shared_segments(s, &four) == 2)
        .unwrap();
    assert_ne!(three, two);

    five_letters.remove(&two);
    five_letters.remove(&three);
    let five = five_letters.into_iter().next().unwrap();

    samples.remove(&two);
    samples.remove(&three);
    samples.remove(&five);

    mapping.insert(two, 2);
    mapping.insert(three.clone(), 3);
    mapping.insert(five, 5);

    assert_eq!(3, samples.len());
    assert!(samples.iter().all(|s| s.len() == 6));

    let six = samples
        .iter()
        .cloned()
        .find(|s| shared_segments(s, &one) == 1)
        .unwrap();
    let nine = samples
        .iter()
        .cloned()
        .find(|s| shared_segments(s, &three) == 5)
        .unwrap();
    samples.remove(&six);
    samples.remove(&nine);
    let zero = samples.into_iter().next().unwrap();

    mapping.insert(zero, 0);
    mapping.insert(six, 6);
    mapping.insert(nine, 9);

    digits
        .into_iter()
        .map(|d| mapping[&d])
        .fold(0, |s, d| s * 10 + d)
}

fn part02(input: &str) -> u64 {
    input.lines().map(decode_digits).sum()
}

fn main() {
    println!("part 1: {}", part01(INPUT));
    println!("part 2: {}", part02(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part01() {
        assert_eq!(26, part01(TEST));
        assert_eq!(245, part01(INPUT));
    }

    #[test]
    fn test_part02_partial() {
        assert_eq!(5353, part02("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"));
    }

    #[test]
    fn test_part02() {
        assert_eq!(61229, part02(TEST));
        assert_eq!(983026, part02(INPUT));
    }
}
