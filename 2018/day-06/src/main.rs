use std::error::Error;

use aoc_utils::get_input;

fn input() -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    use std::io::BufRead;
    let mut v = Vec::new();
    for line in get_input().lines() {
        let line = line?;
        let mut iter = line.split(", ");
        let x: i32 = iter.next().unwrap().parse()?;
        let y: i32 = iter.next().unwrap().parse()?;
        v.push((x, y));
    }
    Ok(v)
}

fn distance(from: &(i32, i32), to: &(i32, i32)) -> usize {
    ((from.0 - to.0).abs() + (from.1 - to.1).abs()) as usize
}

// any point that is the closest on the boundary
// has infinite neighbourhood:
// if closest on the boundary, then any point on
// the perpendicular to the boundary from closest
// will be also in same neighbourhood as distance
// to all points inside the boundary will increase
// by the same amount (and thus original closest
// will remain closest)

// therefore any point that is closest to a point on
// the boundary is disqualified

fn boundary(input: &Vec<(i32, i32)>) -> Option<((i32, i32), (i32, i32))> {
    if input.is_empty() {
        None
    } else {
        Some((
            (
                input.iter().map(|&(x, _)| x).min().unwrap().clone(),
                input.iter().map(|&(_, y)| y).min().unwrap().clone(),
            ),
            (
                input.iter().map(|&(x, _)| x).max().unwrap().clone(),
                input.iter().map(|&(_, y)| y).max().unwrap().clone(),
            ),
        ))
    }
}

use std::collections::HashMap;
use std::collections::HashSet;
fn compute_closests(input: &Vec<(i32, i32)>) -> (HashMap<(i32, i32), usize>, HashSet<usize>) {
    let mut infinite: HashSet<usize> = HashSet::new();

    let ((min_x, min_y), (max_x, max_y)) = boundary(input).unwrap();
    let mut map = HashMap::new();

    for x in 0..=max_x {
        'y_loop: for y in 0..=max_y {
            let distances: Vec<_> = input.iter().map(|p| distance(&(x, y), p)).collect();
            let min_dist = distances.iter().min().unwrap();
            let candidates: Vec<_> = distances.iter().enumerate().filter(|(_, d)| *d == min_dist).collect();
            if candidates.len() == 1 {
                let best = candidates[0];
                map.insert((x, y), best.0);
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    infinite.insert(best.0);
                }
            }
        }
    }
    (map, infinite)
}

fn part_one(input: &Vec<(i32, i32)>) -> usize {
    let (map, infinite) = compute_closests(input);

    let mut areas: HashMap<usize, usize> = HashMap::new();

    for p in map.values() {
        if !infinite.contains(&p) {
            *areas.entry(*p).or_default() += 1;
        }
    }

    areas.values().max().unwrap().clone()
}

fn part_two(input: &Vec<(i32, i32)>, limit: usize) -> usize {
    let ((min_x, min_y), (max_x, max_y)) = boundary(input).unwrap();

    let delta = (limit / input.len() + 1) as i32;

    let min_x = min_x - delta;
    let min_y = min_y - delta;
    let max_x = max_x + delta;
    let max_y = max_y + delta;

    let mut count = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let total_dist: usize = input.iter().map(|p| distance(&(x, y), p)).sum();
            if total_dist < limit {
                count += 1;
            }
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input()?;
    println!("largest safe area: {}", part_one(&input));
    println!("largest area within 10000 distance: {}", part_two(&input, 10000));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    fn input() -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for line in INPUT.lines() {
            let mut iter = line.split(", ");
            let x: i32 = iter.next().unwrap().parse().unwrap();
            let y: i32 = iter.next().unwrap().parse().unwrap();
            v.push((x, y));
        }
        v
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&input()), 17);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&input(), 32), 16);
    }
}
