use std::collections::HashSet;

const INPUT: &str = include_str!("input");

// bad format; probably want to index by (usize, usize) instead
struct Board([[usize; 5]; 5]);

impl Board {
    fn new(data: &Vec<usize>) -> Self {
        let mut b = Board([[0; 5]; 5]);
        for i in 0..5 {
            for j in 0..5 {
                b.0[i][j] = data[i*5+j];
            }
        }
        b
    }

    fn sum(&self) -> usize {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                sum += self.0[i][j];
            }
        }
        sum
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut lines = input.lines();

    let picks = lines.next().unwrap();

    let picks = picks.split(',').map(|n| n.parse().unwrap()).collect();

    let mut acc = Vec::new();
    let mut boards = Vec::new();

    while let Some(line) = lines.next() {
        if !line.is_empty() {
            acc.extend(line.split_whitespace().map(|n| n.parse::<usize>().unwrap()));
            if acc.len() == 25 {
                boards.push(Board::new(&acc));
                acc.clear();
            }
        }
    }

    (picks, boards)
}

fn check_bingo(board: &Board, i: usize, j: usize) -> bool {
    let mut nonzero = false;
    for x in 0..5 {
        if board.0[i][x] != 0 {
            nonzero = true;
            break;
        }
    }
    if !nonzero {
        return true;
    }

    for y in 0..5 {
        if board.0[y][j] != 0 {
            return false;
        }
    }

    return true;
} 

fn bingo(board: &mut Board, pick: usize) -> bool {
    for i in 0..5 {
        for j in 0..5 {
            if board.0[i][j] == pick {
                board.0[i][j] = 0;
                return check_bingo(board, i, j);
            }
        }
    }
    false
}

fn part01(input: &str) -> usize {
    let (picks, mut boards) = parse(input);

    for pick in picks {
        for board in &mut boards {
            if bingo(board, pick) {
                return pick * board.sum();
            }
        }
    }

    0
}

// could really compute that in part01 instead...
fn part02(input: &str) -> usize {
    let (picks, mut boards) = parse(input);
    let mut last = 0;
    let mut seen = HashSet::new();


    for pick in picks {
        for (idx, board) in boards.iter_mut().enumerate() {
            if !seen.contains(&idx) && bingo(board, pick) {
                seen.insert(idx);
                last = pick * board.sum();
            }
        }
    }
    last 
}

fn main() {
    println!("part 01: {}", part01(INPUT));
    println!("part 02: {}", part02(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part01() {
        assert_eq!(4512, part01(&TEST));
    }

    #[test]
    fn test_part02() {
        assert_eq!(1924, part02(&TEST));
    }
}
