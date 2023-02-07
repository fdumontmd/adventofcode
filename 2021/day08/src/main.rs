const INPUT: &str = include_str!("input");

fn part01(input: &str) -> usize {
    input.lines().map(|l| l.split('|').nth(1).expect(&format!("cannot parse line {}", l)).split_whitespace().filter(|n| n.len() == 2 || n.len() == 3 || n.len() == 4 || n.len() == 7).count()).sum()
}

fn part02(input: &str) -> usize {
    // analysis:
    // 1 -> 2 segments
    // 7 -> 3
    // 4 -> 4
    // 2 -> 5
    // 3 -> 5
    // 5 -> 5
    // 0 -> 6 
    // 6 -> 6
    // 9 -> 6
    // 8 -> 7
    
    // so 1, 4, 7 and 8 are unique segment counts
    // 7 - 1 gives us real a
    // 4 - 1 gives bd
    // 8 - 4 gives aeg
    // 7 - 4 gives a
    // 4 - 7 gives bd
    // 0 and 1 have 2 common segments
    // 2 and 1 have 1
    // 3 and 1 have 2
    // 5 and 1 have 1
    // 6 and 1 have 1
    // 9 have 2
    // -> with 1, can find 3, 6
    //
    // 0 and 4 have 3
    // 2 and 4 have 2
    // 3 and 4 have 3
    // 5 and 5 have 3
    // 6 and 4 have 3
    // 9 and 4 have 4
    // with 4, can find 2, 9
    //
    // 0 and 7 have 3
    // 2 and 7 have 2
    // 3 and 7 have 3
    // 5 and 7 have 2
    // 6 and 7 have 2
    // 9 and 7 have 3
    // with 7, can find 6, 3
    //
    // with 2, 3, can find 5
    // with 6, 9, can find 0
    
    // 8 is useless

    0
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
    }

    #[test]
    fn test_part02() {
        assert_eq!(5353, part02(TEST));
    }
}
