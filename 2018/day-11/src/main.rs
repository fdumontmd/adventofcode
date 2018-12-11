const INPUT: i64 = 7139;

const SIDE: usize = 300;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position(i64, i64);

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position(x as i64, y as i64)
    }

    #[inline]
    fn rack_id(&self) -> i64 {
        self.0 + 10
    }

    fn power_level(&self, serial: i64) -> i64 {
        let power = self.rack_id() * self.1;
        let power = power + serial;
        let power = power * self.rack_id();
        let power = (power / 100) % 10;
        power - 5
    }

    fn as_tuple(&self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }
}

struct Grid {
    grid: Vec<Vec<i64>>,
}

impl Grid {
    fn new(serial: i64) -> Self {
        let mut grid = vec![vec![0; 300]; 300];
        for y in 0..SIDE {
            for x in 0..SIDE {
                grid[y][x] = Position::new(x + 1, y + 1).power_level(serial);
            }
        }
        Grid {
            grid,
        }
    }

    fn best_square_power(&self, side: usize) -> ((usize, usize), i64) {
        let mut best = None;

        for y in 0..(SIDE-side + 1) {
            for x in 0..(SIDE-side + 1) {
                let mut current = 0;
                for dy in 0..side {
                    for dx in 0..side {
                        current += self.grid[y + dy][x + dx];
                    }
                }

                best = Some(match best.take() {
                    None => (Position::new(x + 1, y + 1), current),
                    Some((p, l)) => {
                        if current > l {
                            (Position::new(x + 1, y + 1), current)
                        } else {
                            (p, l)
                        }
                    }
                });
            }
        }

        let best = best.unwrap();
        (best.0.as_tuple(), best.1)
    }

    fn best_square(&self) -> (usize, usize, usize) {
        // memoization to the rescue
        use std::collections::HashMap;
        let mut memo: HashMap<(usize, usize, usize), i64> = HashMap::new();
        // copy square of size 1:
        for y in 0..SIDE {
            for x in 0..SIDE {
                memo.insert((x + 1, y + 1, 1),self.grid[y][x]);
            }
        }

        for side in 2..=SIDE {
            for y in 0..(SIDE-side+1) {
                for x in 0..(SIDE-side+1) {
                    let mut current = memo[&(x + 1, y + 1, side-1)];
                    for dx in 0..side {
                        current += self.grid[y+side-1][x+dx];
                    }
                    for dy in 0..(side-1) {
                        current += self.grid[y+dy][x+side-1];
                    }
                    memo.insert((x + 1, y + 1, side), current);
                }
            }
        }

        let best = memo.into_iter().max_by(|f,s| f.1.cmp(&s.1)).unwrap();
        best.0
    }
}


fn main() {
    let grid = Grid::new(INPUT);
    println!("part one: {:?}", grid.best_square_power(3));
    println!("part two: {:?}", grid.best_square());
    println!("part two': {:?}", grid.best_square_power(16));

    for side in 0..SIDE {
        println!("part two'': best for side {}: {:?}", side + 1, grid.best_square_power(side+1));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_power_level() {
        assert_eq!(Position::new(3,5).power_level(8), 4);
        assert_eq!(Position::new(122,79).power_level(57), -5);
        assert_eq!(Position::new(217,196).power_level(39), 0);
        assert_eq!(Position::new(101,153).power_level(71), 4);
    }

    #[test]
    fn test_grid() {
        assert_eq!(Grid::new(18).best_square_power(3), ((33, 45), 29));
        assert_eq!(Grid::new(42).best_square_power(3), ((21, 61), 30));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Grid::new(18).best_square(), (90,269,16));
        assert_eq!(Grid::new(42).best_square(), (232,251,12));
    }
}
