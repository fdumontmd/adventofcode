use aoc_utils::num::chinese_remainders;

static INPUT: &str = include_str!("input.txt");

fn part_1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let start_time: usize = lines[0].parse().unwrap();
    let (wait, id) = lines[1]
        .split(',')
        .filter_map(|d| d.parse::<usize>().ok())
        .map(|d| {
            let wait = d - (start_time % d);
            (wait, d)
        })
        .min()
        .unwrap();
    wait * id
}

fn part_2(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    let ids: Vec<_> = lines[lines.len() - 1]
        .split(',')
        .enumerate()
        .filter_map(|(idx, d)| {
            d.parse::<i64>()
                .ok()
                .map(|d| ((d - idx as i64).rem_euclid(d), d))
        })
        .collect();

    let x = chinese_remainders(&ids);
    let x = (x.0.rem_euclid(x.1), x.1);

    x.0.rem_euclid(x.1)
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};
    use test_case::test_case;

    static TEST_INPUT: &str = r"939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part_1() {
        assert_eq!(295, part_1(TEST_INPUT));
        assert_eq!(2092, part_1(INPUT));
    }

    #[test_case(1068781, TEST_INPUT)]
    #[test_case(3417, "17,x,13,19")]
    #[test_case(754018, "67,7,59,61")]
    #[test_case(779210, "67,x,7,59,61")]
    #[test_case(1261476, "67,7,x,59,61")]
    #[test_case(1202161486, "1789,37,47,1889")]
    #[test_case(702970661767766, INPUT)]
    fn test_part_2(start: i64, input: &str) {
        assert_eq!(start, part_2(input));
    }
}
