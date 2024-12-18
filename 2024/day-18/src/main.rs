use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");
const INPUT_SIZE: Size = (71, 71);

type Size = (usize, usize);
type Pos = (usize, usize);

fn parse_locations(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let coords: Vec<usize> = l.split(',').map(|n| n.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect()
}

// let's try to reuse the search bit...
fn try_find_path(bytes_pos: &[Pos], size: Size, bytes: usize) -> Option<usize> {
    let mut grid = vec![vec![false; size.0]; size.1];
    for (idx, pos) in bytes_pos.iter().enumerate() {
        if idx >= bytes {
            break;
        }
        grid[pos.1][pos.0] = true;
    }

    // strict BFS, so lowest number of steps will come first
    let mut queue = VecDeque::new();
    queue.push_front(((0, 0), 0));
    let mut seen = vec![vec![false; size.0]; size.1];

    while let Some((pos, steps)) = queue.pop_back() {
        if pos == (size.0 - 1, size.1 - 1) {
            return Some(steps);
        }
        for target in [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|d| {
                Some((
                    pos.0.checked_add_signed(d.0)?,
                    pos.1.checked_add_signed(d.1)?,
                ))
            })
        {
            if target.0 >= size.0 || target.1 >= size.1 {
                continue;
            }
            if !grid[target.1][target.0] && !seen[target.1][target.0] {
                seen[target.1][target.0] = true;
                queue.push_front((target, steps + 1))
            }
        }
    }

    None
}

fn part1(input: &str, size: Size, bytes: usize) -> usize {
    let bytes_pos = parse_locations(input);
    try_find_path(&bytes_pos, size, bytes).unwrap()
}

fn part2(input: &str, size: Size) -> Pos {
    let bytes_pos = parse_locations(input);
    let mut top = input.lines().count();
    let mut bottom = 0;

    // let's try for a binary search...
    loop {
        let mid = bottom + (top - bottom) / 2;
        if try_find_path(&bytes_pos, size, mid).is_some() {
            bottom = mid;
        } else {
            top = mid;
        }
        if top <= bottom + 1 {
            return bytes_pos[bottom];
        }
    }
}

fn main() {
    println!("part 1: {}", part1(INPUT, INPUT_SIZE, 1024));
    println!("part 2: {:?}", part2(INPUT, INPUT_SIZE));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    const TEST_INPUT_SIZE: Size = (7, 7);

    #[test_case(TEST_INPUT, TEST_INPUT_SIZE, 12, 22; "test input")]
    #[test_case(INPUT, INPUT_SIZE, 1024, 360; "input")]
    fn test_part1(input: &str, size: Size, bytes: usize, steps: usize) {
        assert_eq!(steps, part1(input, size, bytes));
    }

    #[test_case(TEST_INPUT, TEST_INPUT_SIZE, (6,1); "test input")]
    #[test_case(INPUT, INPUT_SIZE, (58,62); "input")]
    fn test_part2(input: &str, size: Size, byte: Pos) {
        assert_eq!(byte, part2(input, size));
    }
}
