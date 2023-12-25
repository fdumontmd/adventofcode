use std::{cmp::Reverse, collections::BinaryHeap};

use crate::custom_error::AocError;

type Position = (usize, usize, usize);

type Brick = (Position, Position);

pub fn overlap(b: &Brick, o: &Brick) -> bool {
    let xmin = b.0 .0.max(o.0 .0);
    let ymin = b.0 .1.max(o.0 .1);
    let xmax = b.1 .0.min(o.1 .0);
    let ymax = b.1 .1.min(o.1 .1);

    xmin <= xmax && ymin <= ymax
}

pub fn settle(bricks: Vec<Brick>) -> Vec<Brick> {
    // try a new approach:
    // split bricks into two sets: settled, and in the air
    // find the one in the air with the lowest bottom z (maybe use a priority queue)
    // no matter what, it must be below anything else (or same level, which is same)
    // take it out, and find where to add it: compute the list of overlapping settled
    // bricks, and take the one with the highest top z

    let mut queue = BinaryHeap::from_iter(bricks.into_iter().map(|b| (Reverse(b.0 .2), b)));

    let mut bricks = Vec::new();

    while let Some((_, mut b)) = queue.pop() {
        let z = bricks
            .iter()
            .filter_map(|s| if overlap(s, &b) { Some(s.1 .2) } else { None })
            .max()
            .unwrap_or(0)
            + 1;
        let h = b.1 .2 - b.0 .2;
        b.0 .2 = z;
        b.1 .2 = z + h;
        bricks.push(b);
    }

    bricks.sort_by_key(|b| b.0 .2);

    bricks
}

pub fn parse_bricks(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split('~');
        let part_1: Vec<usize> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|d| d.parse().unwrap())
            .collect();
        let part_2: Vec<usize> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|d| d.parse().unwrap())
            .collect();

        // normalize so that first part is always smaller than second part
        bricks.push((
            (
                part_1[0].min(part_2[0]),
                part_1[1].min(part_2[1]),
                part_1[2].min(part_2[2]),
            ),
            (
                part_1[0].max(part_2[0]),
                part_1[1].max(part_2[1]),
                part_1[2].max(part_2[2]),
            ),
        ));
    }
    bricks
}

pub fn compute_support(input: &str) -> (Vec<Vec<usize>>, Vec<usize>) {
    let bricks = parse_bricks(input);
    let bricks = settle(bricks);
    // now, a bricks supports another one iff they overlap and bottom one top z is
    // top one bottom z - 1

    // ids of bricks supported
    let mut supports: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];
    // number of bricks supporting
    let mut supported_by: Vec<usize> = vec![0; bricks.len()];

    for (idx, b) in bricks.iter().enumerate() {
        for (o_idx, o) in bricks[idx + 1..].iter().enumerate() {
            let o_idx = idx + 1 + o_idx;

            if b.1 .2 + 1 == o.0 .2 && overlap(b, o) {
                supports[idx].push(o_idx);
                supported_by[o_idx] += 1;
            } else if o.0 .2 > b.1 .2 + 1 {
                continue;
            }
        }
    }

    (supports, supported_by)
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let (supports, supported_by) = compute_support(input);

    let count = supports
        .into_iter()
        .enumerate()
        .filter(|(_, v)| v.iter().all(|s| supported_by[*s] > 1))
        .count();

    Ok(format!("{count}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[rstest]
    #[case(INPUT, "5")]
    #[case(include_str!("../input.txt"), "490")]

    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
