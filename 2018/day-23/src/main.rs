use std::{collections::BinaryHeap, cmp::Reverse};

use itertools::{Itertools, MinMaxResult};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INPUT_REGEX: Regex = Regex::new(r"pos=<(.*)>, r=(\d+)").unwrap();
}

static INPUT: &str = include_str!("input.txt");

type Position = (i64, i64, i64);

fn distance(p1: Position, p2: Position) -> i64 {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) + p1.2.abs_diff(p2.2)) as i64
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Nanobot {
    position: Position,
    range: i64,
}

impl Nanobot {
    fn from_str(input: &str) -> Self {
        let caps = INPUT_REGEX.captures(input).unwrap();

        let position: Vec<_> = caps[1].split(',').collect();
        let position = (
            position[0].parse().unwrap(),
            position[1].parse().unwrap(),
            position[2].parse().unwrap(),
        );
        let range = caps[2].parse().unwrap();

        Self { position, range }
    }

    fn in_range(&self, pos: Position) -> bool {
        distance(self.position, pos) <= self.range
    }
}

fn part_01(input: &str) -> usize {
    let nanobots: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Nanobot::from_str)
        .collect();
    let largest_radius = nanobots.iter().max_by_key(|n| n.range).unwrap();
    nanobots
        .iter()
        .filter(|n| largest_radius.in_range(n.position))
        .count()
}

fn _part_02_z3(input: &str) -> i64 {
    use z3::ast;

    let nanobots: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Nanobot::from_str)
        .collect();

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let opt = z3::Optimize::new(&ctx);
    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");
    let z = ast::Int::new_const(&ctx, "z");

    let zero = ast::Int::from_i64(&ctx, 0);
    let one = ast::Int::from_i64(&ctx, 1);

    // ideally, a closure would be nice as I would not have to pass the ctx around

    fn abs<'ctx, 'a>(ctx: &'ctx z3::Context, x: &'a ast::Int<'ctx>) -> ast::Int<'ctx> {
        x.lt(&ast::Int::from_i64(ctx, 0))
            .ite(&(ast::Int::from_i64(ctx, -1) * x), x)
    }

    let mut in_range = ast::Int::from_i64(&ctx, 0);

    for n in nanobots {
        let bot_x = ast::Int::from_i64(&ctx, n.position.0);
        let bot_y = ast::Int::from_i64(&ctx, n.position.1);
        let bot_z = ast::Int::from_i64(&ctx, n.position.2);

        let bot_radius = ast::Int::from_i64(&ctx, n.range) + &one;

        let dist_x = abs(&ctx, &(bot_x - &x));
        let dist_y = abs(&ctx, &(bot_y - &y));
        let dist_z = abs(&ctx, &(bot_z - &z));
        let distance_to_bot = &dist_x + &dist_y + &dist_z;

        in_range = &in_range + distance_to_bot.lt(&bot_radius).ite(&one, &zero);
    }
    opt.maximize(&in_range);

    let dist_x = abs(&ctx, &x);
    let dist_y = abs(&ctx, &y);
    let dist_z = abs(&ctx, &z);
    let distance_to_origin = dist_x + dist_y + dist_z;

    opt.minimize(&distance_to_origin);

    opt.check(&[]);
    let m = opt.get_model().unwrap();
    let res = m.eval(&distance_to_origin, true).unwrap().as_i64().unwrap();
    res
}

// previous algo was greedy, so there would always be configuration where the best
// solution cannot be found because it is in a subspace that was discarded too early.

// Correct solution: branch and bound on the subspaces. The "branch" will include
// - number of nanobots in range (maximizing)
// - distance to origin (minimizing)
// - coordinate of subspace
// - range of subspace
//
// - first space: use bot pos +- range to make sure all the points range of any bot
//   are in the search
//
// outline:
// - pop one "branch" from the priority queue
//  - if range == 1, this is the solution. We can stop now
//  - otherwise, divide range by 2, then create subspaces and push
//    them in the queue
//  - to create subspace, evaluate the number of nanobots in range by using the trick
//    below to include range in the distance
//  - for bounding: the only bound is 1... because the count of bots in range is an 
//  overestimate,
//    we just stop expanding subspaces with no bot in range
fn part_02(input: &str) -> i64 {
    let nanobots: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Nanobot::from_str)
        .collect();

    fn dim_range<F: Fn(&Nanobot) -> i64>(n: &Nanobot, f: &F) -> impl Iterator<Item = i64> {
        let coord = f(n);
        [coord - n.range, coord + n.range].into_iter()
    }

    fn min_max<F: Fn(&Nanobot) -> i64>(n: &[Nanobot], f: &F) -> (i64, i64) {
        let MinMaxResult::MinMax(min, max) = 
        n.iter().flat_map(|n| dim_range(n, f))
            .minmax() else { panic!("no bots?")};
        (min, max)
    }

    let (min_xs, max_xs) = min_max(&nanobots, &|n| n.position.0);
    let (min_ys, max_ys) = min_max(&nanobots, &|n| n.position.1);
    let (min_zs, max_zs) = min_max(&nanobots, &|n| n.position.2);

    let range = (max_xs - min_xs + 1).max((max_ys - min_ys + 1).max(max_zs - min_zs + 1));

    fn estimate_nanobots_in_range(bots: &[Nanobot], pos: Position, range: i64) -> usize {
        bots.iter().filter(|n| (distance(pos, n.position) - n.range) / range <= 0).count()
    }

    fn make_search_state(bots: &[Nanobot], pos: Position, range: i64) -> (usize, Reverse<i64>, (i64, i64, i64), i64) {
        let bots_in_range = estimate_nanobots_in_range(bots, pos, range);
        let dist_to_origin = pos.0.abs() + pos.1.abs() + pos.2.abs();

        (bots_in_range, Reverse(dist_to_origin), pos, range)
    }

    let mut queue = BinaryHeap::new();

    queue.push(make_search_state(&nanobots, (min_xs, min_ys, min_zs), range));

    while let Some((_, _, pos, range)) = queue.pop() {
        if range == 1 {
            return pos.0.abs() + pos.1.abs() + pos.2.abs();
        }

        let new_range = range/2;

        // step_by will skip the first element; use -new_range to put it back
        for x in (pos.0-new_range..=pos.0+range).step_by(new_range as usize) {
            for y in (pos.1-new_range..=pos.1+range).step_by(new_range as usize) {
                for z in (pos.2-new_range..=pos.2+range).step_by(new_range as usize) {
                    let state = make_search_state(&nanobots, (x, y, z), new_range);
                    if state.0 > 0 {
                        queue.push(state);
                    }
                }
            }
        }
    }

    panic!("no solution found")
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_01, part_02, INPUT};
    use test_case::test_case;
    static TEST_INPUT: &str = r"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    static TEST_INPUT_2: &str = r"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

    #[test_case(TEST_INPUT, 7)]
    #[test_case(INPUT, 253)]
    fn test_part_01(input: &str, count: usize) {
        assert_eq!(count, part_01(input));
    }

    #[test_case(TEST_INPUT_2, 36)]
    #[test_case(INPUT, 108618801)]
    fn test_part_02(input: &str, dist: i64) {
        assert_eq!(dist, part_02(input));
    }
}
