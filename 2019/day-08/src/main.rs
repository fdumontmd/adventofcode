static INPUT: &str = include_str!("input.txt");
static WIDTH: usize = 25;
static HEIGHT: usize = 6;

fn part_1(desc: &str, width: usize, height: usize) -> usize {
    desc.trim().as_bytes().chunks(width * height).map(|slice| {
        (
            slice.iter().filter(|b| **b == b'0').count(),
            slice.iter().filter(|b| **b == b'1').count() *
            slice.iter().filter(|b| **b == b'2').count()
        )
    }).min().unwrap().1
}

fn part_2(desc: &str, width: usize, height: usize) -> Vec<Vec<u8>> {
    let layers: Vec<_> = desc.trim().as_bytes()
        .chunks(width * height).collect();

    let combined: Vec<_> =
    (0..(width * height)).into_iter().map(|idx| {
        for l in &layers {
            if l[idx] != b'2' {
                return l[idx] - b'0';
            }
        }
        layers.last().unwrap()[idx] -b'0'
    }).collect();

    combined.chunks(width).map(|line| line.iter().cloned().collect()).collect()
}

fn display(layer: Vec<Vec<u8>>) {
    for line in layer {
        for b in line {
            if b == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    println!("part 1: {}", part_1(INPUT, WIDTH, HEIGHT));
    println!("part 2:");
    display(part_2(INPUT, WIDTH, HEIGHT));
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST: &str = "0222112222120000";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST, 2, 2), vec![vec![0,1],vec![1,0]]);
    }
}
