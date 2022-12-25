static INPUT: &str = include_str!("input.txt");

fn snafu_digit(d: char) -> isize {
    match d {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn snafu_to_decimal(input: &str) -> isize {
    input.chars().fold(0, |n, d| n * 5 + snafu_digit(d))
}

fn decimal_to_snafu(mut input: isize) -> String {
    let mut digits: Vec<char> = Vec::new();

    while input != 0 {
        let mut digit = input % 5;
        input /= 5;
        if digit > 2 {
            digit -= 5;
            input += 1;
        }
        digits.push(match digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        });
    }

    digits.reverse();

    digits.iter().collect()
}

fn part_01(input: &str) -> String {
    let sum = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(snafu_to_decimal)
        .sum();
    dbg!(sum);
    decimal_to_snafu(sum)
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::{decimal_to_snafu, part_01, snafu_to_decimal, INPUT};
    static TEST_INPUT: &str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test_case(1, "1")]
    #[test_case(2, "2")]
    #[test_case(3, "1=")]
    #[test_case(4, "1-")]
    #[test_case(5, "10")]
    #[test_case(6, "11")]
    #[test_case(7, "12")]
    #[test_case(8, "2=")]
    #[test_case(9, "2-")]
    #[test_case(10, "20")]
    #[test_case(15, "1=0")]
    #[test_case(20, "1-0")]
    #[test_case(2022, "1=11-2")]
    #[test_case(12345, "1-0---0")]
    #[test_case(314159265, "1121-1110-1=0")]
    fn test_snafu_to_decimal(decimal: isize, snafu: &str) {
        assert_eq!(snafu_to_decimal(snafu), decimal);
    }

    #[test_case(1, "1")]
    #[test_case(2, "2")]
    #[test_case(3, "1=")]
    #[test_case(4, "1-")]
    #[test_case(5, "10")]
    #[test_case(6, "11")]
    #[test_case(7, "12")]
    #[test_case(8, "2=")]
    #[test_case(9, "2-")]
    #[test_case(10, "20")]
    #[test_case(15, "1=0")]
    #[test_case(20, "1-0")]
    #[test_case(2022, "1=11-2")]
    #[test_case(12345, "1-0---0")]
    #[test_case(314159265, "1121-1110-1=0")]
    fn test_decimal_to_snafu(decimal: isize, snafu: &str) {
        assert_eq!(&decimal_to_snafu(decimal), snafu);
    }

    #[test]
    fn test_part_01() {
        assert_eq!("2=-1=0", &part_01(TEST_INPUT));
    }

    #[test]
    fn real_part_01() {
        assert_eq!("122-0==-=211==-2-200", &part_01(INPUT));
    }
}
