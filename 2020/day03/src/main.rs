const INPUT: &str = include_str!("input.txt");
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut v = Vec::new();
            v.extend_from_slice(l.as_bytes());
            v
        })
    .collect()
}

fn part1(data: &Vec<Vec<u8>>) -> usize {
    let mut col: usize = 0;
    let mut count: usize = 0;
    
    for row in data {
        if row[col] == b'#' {
            count += 1;
        }
        col = (col + 3) % row.len();
    }
    count

}

fn part2_sub(data: &Vec<Vec<u8>>, right: usize, down: usize) -> usize {
    let mut col: usize = 0;
    let mut count: usize = 0;

    let mut idx: usize = 0;

    while idx < data.len() {
        let row = &data[idx];

        if row[col] == b'#' {
            count += 1;
        }
        col = (col + right) % row.len();

        idx += down;
    }

    count
}

fn part2(data: &Vec<Vec<u8>>) -> usize {
    part2_sub(data, 1, 1)
        * part2_sub(data, 3, 1)
        * part2_sub(data, 5, 1)
        * part2_sub(data, 7, 1)
        * part2_sub(data, 1, 2)
}

fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    const DATA: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    use super::*;

    #[test]
    fn check_part1() {
        let data = parse_input(DATA);
        assert_eq!(part1(&data), 7);
    }

    #[test]
    fn check_part2() {
        let data = parse_input(DATA);
        assert_eq!(part2(&data), 336);
    }
}
