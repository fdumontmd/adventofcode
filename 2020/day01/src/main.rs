use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
const TARGET: i64 = 2020;

fn parse() -> Vec<i64> {
    INPUT.split("\n").filter(|s| !s.is_empty()).map(|s| i64::from_str(s).unwrap()).collect()
}

fn part1(data: &[i64], target: i64) -> Option<i64> {
    use std::collections::HashMap;

    // target - item -> item
    let mut map = HashMap::new();
    for item in data {
        map.insert(target - item, item);
    }

    for item in data {
        if let Some(other) = map.get(item) {
            return Some(*other * *item);
        }
    }
    None
}

fn part2(data: &[i64], target: i64) -> Option<i64> {
    for i in data {
        if let Some(mult) = part1(data, target - *i) {
            return Some(mult * *i);
        }
    }

    None
}

fn main() {
    let data = parse();
    println!("part 1: {}", part1(&data, TARGET).unwrap());
        
    println!("part 2: {}", part2(&data, TARGET).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &[i64] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn check_part1() {
        assert_eq!(part1(DATA, TARGET), Some(514579));
    }

    #[test]
    fn check_part2() {
        assert_eq!(part2(DATA, TARGET), Some(241861950));
    }
}
