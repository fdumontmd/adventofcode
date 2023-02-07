const INPUT: &str = include_str!("input");

fn count_bits<'a>(lines: impl IntoIterator<Item = &'a str>) -> (Vec<usize>, usize) {
    let mut total = 0;
    let mut bits: Vec<usize> = Vec::new();
    for line in lines {
        if bits.is_empty() {
            bits = vec![0; line.len()];
        }
        total += 1;
        for (idx, b) in line.bytes().enumerate() {
            if b == b'1' {
                bits[idx] += 1;
            }
        }
    }
    (bits, total)
}

fn part1(input: &str) -> usize {
    let (bits, total) = count_bits(input.lines());

    let mut gamma = 0;
    let mut epsilon = 0;

    for b in bits {
        gamma *= 2;
        epsilon *= 2;
        if 2 * b >= total {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn select_max(input: &Vec<&str>) -> usize {
    let mut idx: usize = 0;
    let mut lines: Vec<&str> = input.to_vec();

    loop {
        if lines.len() == 1 {
            break;
        }
        let (bits, total) = count_bits(lines.iter().cloned());
        let keep = if 2 * bits[idx] >= total {
            b'1'
        } else {
            b'0'
        };

        lines = lines.into_iter().filter(|l| l.as_bytes()[idx] == keep).collect();
        idx += 1;
    }

    usize::from_str_radix(lines[0], 2).unwrap()
}


fn select_min(input: &Vec<&str>) -> usize {
    let mut idx: usize = 0;
    let mut lines: Vec<&str> = input.to_vec();

    loop {
        if lines.len() == 1 {
            break;
        }
        let (bits, total) = count_bits(lines.iter().cloned());
        let keep = if 2 * bits[idx] < total {
            b'1'
        } else {
            b'0'
        };

        lines = lines.into_iter().filter(|l| l.as_bytes()[idx] == keep).collect();
        idx += 1;
    }

    usize::from_str_radix(lines[0], 2).unwrap()
}

fn part2(input: &str) -> usize {
    let lines = input.lines().collect();

    select_max(&lines) * select_min(&lines)
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_01() {
        assert_eq!(198, part1(TEST));
    }

    #[test]
    fn test_02_ogr() {
        assert_eq!(23, select_max(&TEST.lines().collect()));
   }

   #[test]
    fn test_02_csr() {
        assert_eq!(10, select_min(&TEST.lines().collect()));
   }

    #[test]
    fn test_02() {
        assert_eq!(230, part2(TEST));
    }

}
