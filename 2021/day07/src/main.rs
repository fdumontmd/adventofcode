const INPUT: &str = include_str!("input");

fn part01(input: &str) -> isize {
    let mut pos: Vec<isize> = input.split(',').map(|n| n.trim().parse().unwrap()).collect();
    pos.sort();

    let median = pos[pos.len() / 2];
    dbg!(median);

    let cur: isize = pos.iter().map(|p| (p - median).abs()).sum();
    let prev: isize = pos.iter().map(|p| (p - median + 1).abs()).sum();
    let next: isize = pos.iter().map(|p| (p - median - 1).abs()).sum();

    dbg!(cur);
    dbg!(prev);
    dbg!(next);

    pos.into_iter().map(|p| (p - median).abs()).sum()
}

fn cost(d: isize) -> isize {
    d * (d + 1) / 2
}

fn part02(input: &str) -> isize {
    let mut pos: Vec<isize> = input.split(',').map(|n| n.trim().parse().unwrap()).collect();
    pos.sort();

    let total: isize = pos.iter().sum();

    // got lucky with this one...
    // really what we need is an idx that minimize the square diff with the others
    // so least square?
    // damn, reddit does not help. soo many brute force solutions. in python...
    let mut mean = (total as f32 / pos.len() as f32).round() as isize;

    // the value is a minima, but in test and input it's also a minimum... so this works
    loop {
        let cur: isize = pos.iter().map(|p| cost((p - mean).abs())).sum();
        let prev: isize = pos.iter().map(|p| cost((p - mean + 1).abs())).sum();
        let next: isize = pos.iter().map(|p| cost((p - mean - 1).abs())).sum();

        dbg!(cur);
        dbg!(prev);
        dbg!(next);

        if cur < prev && cur < prev {
            return cur;
        } else if prev < cur {
            mean -= 1;
        } else {
            mean += 1;
        }
    }
}

fn main() {
    println!("part 01: {}", part01(INPUT));
    println!("part 02: {}", part02(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part01() {
        assert_eq!(37, part01(TEST));
    }

    #[test]
    fn test_part02() {
        assert_eq!(168, part02(TEST));
    }
}
