const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<u64> {
    let mut adapters = vec![0];
    adapters.extend(input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u64>().unwrap()));

    adapters.sort();

    adapters.push(adapters[adapters.len() - 1] + 3);

    adapters
}

fn part1(data: &Vec<u64>) -> usize {
    let diffs: Vec<_> = data.windows(2)
        .map(|w| w[1] - w[0])
        .filter(|&d| d == 1 || d == 3)
        .collect();

    diffs.iter().filter(|&d| *d == 1).count()
        * diffs.iter().filter(|&d| *d == 3).count()
}

fn part2(data: &Vec<u64>) -> u64 {
    // start from the end: compute how many solutions from each adapter
    // as sum of reachable adapters
    use std::collections::HashMap;

    let mut combinations = HashMap::new();

    let mut data = data.clone();
    data.reverse();

    // compute first 3 by hand
    // only one way to combine final adapter
    combinations.insert(data[0], 1);
    // but last adapter, if used, must use next adapter
    combinations.insert(data[1], 1);
    if data[0] - data[2] <= 3 {
        combinations.insert(data[2], 2);
    } else {
        combinations.insert(data[2], 1);
    }

    data.windows(4).for_each(|w| {
        let mut combi = 0;
        for i in 0..3 {
            if w[i] - w[3] <= 3 {
                combi += *combinations.get(&w[i]).unwrap();
            }
        }

        combinations.insert(w[3], combi);
    });
    *combinations.get(&0).unwrap()
}

fn main() {
    let data = parse(INPUT);
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2(&data));
}

#[cfg(test)]
mod test {
    use super::*;
    const SMALL_TEST_INPUT: &str = r#"16
10
15
5
1
11
7
19
6
12
4
"#;

    const LARGE_TEST_INPUT: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"#;

    #[test]
    fn test_small_1() {
        assert_eq!(part1(&parse(SMALL_TEST_INPUT)), 35);
    }

    #[test]
    fn test_large_1() {
        assert_eq!(part1(&parse(LARGE_TEST_INPUT)), 220);
    }

    #[test]
    fn test_small_2() {
        assert_eq!(part2(&parse(SMALL_TEST_INPUT)), 8);
    }

    #[test]
    fn test_large_2() {
        assert_eq!(part2(&parse(LARGE_TEST_INPUT)), 19208);
    }
}

