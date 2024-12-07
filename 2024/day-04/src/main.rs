const INPUT: &str = include_str!("input.txt");
const PATTERN: &str = "XMAS";

fn count_matches_forward(mut line: &str, pattern: &str) -> usize {
    let mut count = 0;

    while !line.is_empty() {
        if let Some(pos) = line.find(pattern) {
            count += 1;
            line = &line[pos + 1..];
        } else {
            break;
        }
    }

    count
}

fn count_matches(line: &str, pattern: &str) -> usize {
    count_matches_forward(line, pattern)
        + count_matches_forward(&(line.chars().rev().collect::<String>()), pattern)
}

fn flip_diagonal(input: &str) -> Vec<String> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut diags: Vec<Vec<char>> = vec![vec![]; 2 * lines.len() - 1];

    assert_eq!(lines.len(), lines[0].len());

    // first upper diag
    for ud in 0..lines.len() {
        for c in 0..=ud {
            let r = ud - c;
            diags[ud].push(lines[r][c]);
        }
    }

    // lower diag
    for ld in 1..lines.len() {
        for c in ld..lines.len() {
            let r = lines.len() - c + ld - 1;
            diags[lines.len() + ld - 1].push(lines[r][c]);
        }
    }

    diags.into_iter().map(|d| d.into_iter().collect()).collect()
}

fn flip_grid(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut cols = vec![vec![]; lines[0].len()];

    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            cols[idx].push(c);
        }
    }

    cols.into_iter()
        .map(|col| col.into_iter().collect())
        .collect()
}

// brute forcing my way through
fn part1(input: &str) -> usize {
    let horiz: usize = input.lines().map(|line| count_matches(line, PATTERN)).sum();

    let vert: usize = flip_grid(input)
        .iter()
        .map(|col| count_matches(col, PATTERN))
        .sum();

    // diagonal
    let diag1: usize = flip_diagonal(input)
        .into_iter()
        .map(|d| count_matches(&d, PATTERN))
        .sum();

    // I won't try and figure out diagonal indices again... this is fast enough
    let reversed: Vec<String> = input
        .lines()
        .map(|r| r.chars().rev().collect::<String>())
        .collect();

    let reversed = reversed.join("\n");

    let diag2: usize = flip_diagonal(&reversed)
        .into_iter()
        .map(|d| count_matches(&d, PATTERN))
        .sum();

    horiz + vert + diag1 + diag2
}

fn part2(input: &str) -> usize {
    // look for A inside the grid; then check the corners for 2 Ms and 2 Ss,
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut count = 0;

    for r in 1..grid.len() - 1 {
        for c in 1..grid[0].len() - 1 {
            if grid[r][c] == 'A'
                && match (
                    grid[r - 1][c - 1],
                    grid[r - 1][c + 1],
                    grid[r + 1][c - 1],
                    grid[r + 1][c + 1],
                ) {
                    ('M', 'M', 'S', 'S')
                    | ('S', 'S', 'M', 'M')
                    | ('M', 'S', 'M', 'S')
                    | ('S', 'M', 'S', 'M') => true,

                    _ => false,
                }
            {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test_case(TEST_INPUT, 18; "test input")]
    #[test_case(INPUT, 2397; "input")]
    fn test_part_1(input: &str, count: usize) {
        assert_eq!(count, part1(input));
    }

    #[test_case(TEST_INPUT, 9; "test input")]
    #[test_case(INPUT, 1824; "input")]
    fn test_part_2(input: &str, count: usize) {
        assert_eq!(count, part2(input));
    }
}
