const INPUT: &str = include_str!("input.txt");

fn parse_seat_id(id: &str) -> usize {
    let mut s = 0;
    for c in id.chars() {
        match c {
            'F'|'L' => s *= 2,
            'B'|'R' => s = s * 2  + 1,
            _ => unreachable!(),
        }
    }
    s
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().filter(|l| !l.is_empty())
        .map(|l| parse_seat_id(l))
        .collect()
}

fn part1(data: &Vec<usize>) -> usize {
    *data.iter().max().unwrap()
}

fn part2(data: &mut Vec<usize>) -> usize {
    data.sort();
    for p in data.windows(2) {
        if p[0] + 2 == p[1] {
            return p[0] + 1;
        }
    }

    unreachable!()
}

fn main() {
    let mut data = parse_input(INPUT);
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2(&mut data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_seat_parser() {
        assert_eq!(parse_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(parse_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(parse_seat_id("BBFFBBFRLL"), 820);
    }
}
