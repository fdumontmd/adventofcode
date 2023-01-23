use aoc_utils::union_find::UnionFind;

static INPUT: &str = include_str!("input.txt");
// combine n^2 dist comparison with union-find

type Point = (i64, i64, i64, i64);

fn distance(p1: &Point, p2: &Point) -> u64 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) + p1.2.abs_diff(p2.2) + p1.3.abs_diff(p2.3)
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let l = l.trim();
            let dims: Vec<_> = l.split(',').collect();
            (
                dims[0].parse().unwrap(),
                dims[1].parse().unwrap(),
                dims[2].parse().unwrap(),
                dims[3].parse().unwrap(),
            )
        })
        .collect()
}

fn part_01(input: &str) -> usize {
    let data = parse(input);
    dbg!(data.len());
    let mut uf = UnionFind::new();
    // ensure_capacity is badly named; means make sure
    // argument can be used as index, so always 1 less than len
    // TODO: find a more meaningful name or API
    uf.ensure_capacity(data.len() - 1);

    for i in 0..data.len() {
        for j in i + 1..data.len() {
            if distance(&data[i], &data[j]) <= 3 {
                uf.join(i, j);
            }
        }
    }
    dbg!(uf.groups());
    uf.groups().len()
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::part_01;
    use test_case::test_case;

    #[test_case(
        r"0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0",
        2
    )]
    #[test_case(
        r"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0",
        4
    )]
    #[test_case(
        r"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2",
        3
    )]
    #[test_case(
        r"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2",
        8
    )]
    fn test_part_1(input: &str, count: usize) {
        assert_eq!(count, part_01(input));
    }
}
