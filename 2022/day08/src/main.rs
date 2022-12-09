use std::{cmp::max, collections::HashSet, ops::Index};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Map {
    trees: Vec<i8>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut trees = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            width = line.len();
            height += 1;
            trees.extend(line.bytes().map(|c| (c - b'0') as i8));
        }

        Map {
            trees,
            width,
            height,
        }
    }

    fn rows(&self) -> impl Iterator<Item = usize> {
        0..self.height
    }

    fn cols(&self) -> impl Iterator<Item = usize> {
        0..self.width
    }

    fn row_iter(&self, row: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..self.width).map(move |col| (col, row))
    }

    fn row_iter_rev(&self, row: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..self.width).rev().map(move |col| (col, row))
    }

    fn col_iter(&self, col: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height).map(move |row| (col, row))
    }

    fn col_iter_rev(&self, col: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height).rev().map(move |row| (col, row))
    }

    fn visible_count(&self) -> usize {
        let mut visibles = HashSet::new();

        for row in self.rows() {
            let mut min = -1;

            self.row_iter(row)
                .filter(|&(col, row)| self.is_visible(col, row, &mut min))
                .for_each(|idx| {
                    visibles.insert(idx);
                });

            min = -1;

            self.row_iter_rev(row)
                .filter(|&(col, row)| self.is_visible(col, row, &mut min))
                .for_each(|idx| {
                    visibles.insert(idx);
                });
        }

        for col in self.cols() {
            let mut min = -1;

            self.col_iter(col)
                .filter(|&(col, row)| self.is_visible(col, row, &mut min))
                .for_each(|idx| {
                    visibles.insert(idx);
                });

            min = -1;

            self.col_iter_rev(col)
                .filter(|&(col, row)| self.is_visible(col, row, &mut min))
                .for_each(|idx| {
                    visibles.insert(idx);
                });
        }

        visibles.len()
    }

    fn is_visible(&self, col: usize, row: usize, min: &mut i8) -> bool {
        let visible = *self.index((col, row)) > *min;
        *min = max(*min, *self.index((col, row)));
        visible
    }

    fn scenic_score(&self, idx: (usize, usize)) -> usize {
        let (col, row) = idx;
        let h = *self.index(idx);
        let mut score = 1;

        // there should be a clever way to use take_while, possibly with a function returning a
        // closure
        let mut v = 0;
        for c in (0..col).rev() {
            v += 1;
            if *self.index((c, row)) >= h {
                break;
            }
        }
        score *= v;

        v = 0;

        for c in col + 1..self.width {
            v += 1;
            if *self.index((c, row)) >= h {
                break;
            }
        }

        score *= v;
        v = 0;

        for r in (0..row).rev() {
            v += 1;
            if *self.index((col, r)) >= h {
                break;
            }
        }

        score *= v;
        v = 0;
        for r in row + 1..self.height {
            v += 1;
            if *self.index((col, r)) >= h {
                break;
            }
        }
        score *= v;

        score
    }

    fn best_scenic_score(&self) -> usize {
        let mut best_score = 0;
        for row in self.rows() {
            for idx in self.row_iter(row) {
                best_score = max(best_score, self.scenic_score(idx));
            }
        }
        best_score
    }
}

impl Index<(usize, usize)> for Map {
    type Output = i8;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.trees[index.1 * self.width + index.0]
    }
}

fn main() {
    let m = Map::from_str(INPUT);
    println!("Visible trees: {}", m.visible_count());
    println!("Best scenic score: {}", m.best_scenic_score());
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_INPUT: &str = r"
30373
25512
65332
33549
35390
";

    #[test]
    fn test_part_1() {
        let m = Map::from_str(TEST_INPUT);
        assert_eq!(21, m.visible_count());
    }

    #[test]
    fn real_part_1() {
        let m = Map::from_str(INPUT);
        assert_eq!(1533, m.visible_count());
    }

    #[test]
    fn test_part_2() {
        let m = Map::from_str(TEST_INPUT);

        assert_eq!(8, m.best_scenic_score());
    }

    #[test]
    fn real_part_2() {
        let m = Map::from_str(INPUT);

        assert_eq!(345744, m.best_scenic_score());
    }
}
