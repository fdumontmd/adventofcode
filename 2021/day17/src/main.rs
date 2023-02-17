use std::ops::RangeInclusive;

const X_RANGE: RangeInclusive<i64> = 150..=171;
const Y_RANGE: RangeInclusive<i64> = -129..=-70;
const TARGET: (RangeInclusive<i64>, RangeInclusive<i64>) = (X_RANGE, Y_RANGE);

fn y_maximum_speed(r: &RangeInclusive<i64>) -> i64 {
    // for a starting dy speed, we'll reach peak at dy(dy+1)/2 then fall
    // we reach the ground at speed -dy
    // now, we'll reach the target if the next pos (-dy - 1) is in range, i.e.
    // is == the lower bound
    r.start().abs() - 1
}

fn maximum_height(y: i64) -> i64 {
    y * (y + 1) / 2
}

fn part_1(target: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> i64 {
    let dy = y_maximum_speed(&target.1);
    maximum_height(dy)
}

fn part_2(target: &(RangeInclusive<i64>, RangeInclusive<i64>)) -> usize {
    // dx must be in the 0..target.0.end() range, or will overshot
    // dy must be less than y_maximum_speed, and greater or equal
    // to target.1.start()

    let mut hit = 0;

    for idx in 1..=*target.0.end() {
        for idy in *target.1.start()..=y_maximum_speed(&target.1) {
            // not the smartest... just compute steps and stop if we are after the x range or below
            // the y range
            let mut x = 0;
            let mut y = 0;
            let mut dx = idx;
            let mut dy = idy;
            while x <= *target.0.end() && y >= *target.1.start() {
                if target.0.contains(&x) && target.1.contains(&y) {
                    hit += 1;
                    break;
                }
                x += dx;
                y += dy;
                dx = 0.max(dx - 1);
                dy -= 1;
            }
        }
    }

    hit
}

fn main() {
    println!("Part 1: {}", part_1(&TARGET));
    println!("Part 2: {}", part_2(&TARGET));
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use crate::{part_1, part_2, TARGET};

    static TEST_INPUT: (RangeInclusive<i64>, RangeInclusive<i64>) = (20..=30, -10..=-5);

    #[test]
    fn test_part_1() {
        assert_eq!(45, part_1(&TEST_INPUT));
        assert_eq!(8256, part_1(&TARGET));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(112, part_2(&TEST_INPUT));
        assert_eq!(2326, part_2(&TARGET));
    }
}
