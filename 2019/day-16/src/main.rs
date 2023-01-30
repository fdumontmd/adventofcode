use std::iter::repeat;

static INPUT: &str = include_str!("input.txt");

fn pattern(pos: usize) -> impl Iterator<Item = i64> {
    repeat(0)
        .take(pos + 1)
        .chain(repeat(1).take(pos + 1))
        .chain(repeat(0).take(pos + 1))
        .chain(repeat(-1).take(pos + 1))
        .cycle()
        .skip(1)
}

fn parse_signal(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u8)
        .collect()
}

// !!half-open on the left!!
fn positive_ranges(pos: usize) -> impl Iterator<Item = (usize, usize)> {
    // indexing start at 1
    let len = pos + 1;
    (0..).map(move |idx| {
        let start = (idx * len * 4) + pos - 1;
        (start, start + len)
    })
}

fn negative_ranges(pos: usize) -> impl Iterator<Item = (usize, usize)> {
    // indexing start at 1
    let len = pos + 1;
    (0..).map(move |idx| {
        let start = (idx * len * 4) + 2 * len + pos - 1;
        (start, start + len)
    })
}

// idea: it's like integration because factors are 0, 1 or -1
// compute "integral" of signal, and
fn phase_integral(signal: Vec<u8>) -> Vec<u8> {
    let sums: Vec<i64> = signal
        .iter()
        .cloned()
        .scan(0, |s, d| {
            *s += d as i64;
            Some(*s)
        })
        .collect();

    let mut res: Vec<u8> = vec![0; signal.len()];

    res[0] = (signal
        .chunks(4)
        .map(|c| *c.first().unwrap_or(&0) as i64 - *c.get(2).unwrap_or(&0) as i64)
        .sum::<i64>()
        .abs()
        % 10) as u8;

    // optimising for second half of array does not bring enough
    // benefits
    for idx in 1..res.len() {
        res[idx] = ((positive_ranges(idx)
            .take_while(|(s, _)| *s < res.len())
            .map(|(s, e)| sums[e.min(res.len() - 1)] - sums[s])
            .sum::<i64>()
            - negative_ranges(idx)
                .take_while(|(s, _)| *s < res.len())
                .map(|(s, e)| sums[e.min(res.len() - 1)] - sums[s])
                .sum::<i64>())
        .abs()
            % 10) as u8;
    }

    res
}

fn phase(signal: Vec<u8>) -> Vec<u8> {
    (0..signal.len())
        .map(|pos| {
            (pattern(pos)
                .zip(signal.iter())
                .map(|(a, &b)| a * b as i64)
                .sum::<i64>()
                .abs()
                % 10) as u8
        })
        .collect()
}

fn part_01(input: &str) -> String {
    let mut signal = parse_signal(input);
    for _ in 0..100 {
        let tmp = phase(signal.clone());
        assert_eq!(tmp, phase_integral(signal));
        signal = tmp;
    }
    signal
        .into_iter()
        .take(8)
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect()
}

// not "fast", but fast enough
fn part_02(input: &str) -> String {
    let mut signal = parse_signal(input);
    let len = signal.len();
    let offset = signal
        .iter()
        .take(7)
        .fold(0usize, |o, d| o * 10 + *d as usize);

    signal = signal.into_iter().cycle().take(len * 10000).collect();

    for idx in 0..100 {
        dbg!(idx);
        signal = phase_integral(signal);
    }

    dbg!(offset);

    signal
        .into_iter()
        .skip(offset)
        .take(8)
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect()
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{part_01, part_02};

    #[test_case("80871224585914546619083218645595", "24176176")]
    #[test_case("19617804207202209144916044189917", "73745418")]
    #[test_case("69317163492948606335995924319873", "52432133")]
    fn test_part_01(signal: &str, output: &str) {
        assert_eq!(output, &part_01(signal));
    }

    #[test_case("03036732577212944063491565474664", "84462026")]
    #[test_case("02935109699940807407585447034323", "78725270")]
    #[test_case("03081770884921959731165446850517", "53553731")]
    fn test_part_02(signal: &str, output: &str) {
        assert_eq!(output, &part_02(signal));
    }
}
