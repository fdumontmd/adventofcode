const INPUT: &str = include_str!("input");

fn parse(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse::<isize>().unwrap()).collect()
}

fn day01(data: &Vec<isize>) -> usize {
    data.windows(2).filter_map(|m| if m[1] > m[0] { Some(true) } else { None } ).count()
}

fn day02(data: &Vec<isize>) -> usize {
    day01(&data.windows(3).map(|m| m.into_iter().sum()).collect())
}

fn main() {
    println!("day 1: {}", day01(&parse(INPUT)));
    println!("day 2: {}", day02(&parse(INPUT)));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_01: &str = r#"199
200
208
210
200
207
240
269
260
263"#;

    #[test]
    fn test_day_01() {
        assert_eq!(7, day01(&parse(TEST_01)));
    }

    #[test]
    fn test_day_02() {
        assert_eq!(5, day02(&parse(TEST_01)));
    }
}


