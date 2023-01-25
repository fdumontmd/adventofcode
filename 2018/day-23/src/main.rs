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

// this gives the correct answer for my input, but looks like correct "by accident"
// the selection of a block of space with the most bots could fail if those bots
// are disjoint are smaller range
// z3 solution is correct by construction, AFAICT
fn part_02(input: &str) -> i64 {
    let nanobots: Vec<_> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(Nanobot::from_str)
        .collect();

    macro_rules! min_max {
        ($x:expr) => {
            (
                nanobots.iter().map($x).min().unwrap(),
                nanobots.iter().map($x).max().unwrap(),
            )
        };
    }

    let mut xs = min_max!(|a| a.position.0);
    let mut ys = min_max!(|a| a.position.1);
    let mut zs = min_max!(|a| a.position.2);

    let mut range = 1;
    while range < xs.1 - xs.0 || range < ys.1 - ys.0 || range < zs.1 - zs.0 {
        range *= 2;
    }

    loop {
        let mut target_count = 0;
        let mut best = (0, 0, 0);
        let mut best_val = 0;

        for x in (xs.0..=xs.1).step_by(range as usize) {
            for y in (ys.0..=ys.1).step_by(range as usize) {
                for z in (zs.0..=zs.1).step_by(range as usize) {
                    let count = nanobots
                        .iter()
                        .filter(|b| (distance((x, y, z), b.position) - b.range) / range <= 0)
                        .count();
                    if count > target_count {
                        // square with higher count
                        target_count = count;
                        best_val = x.abs() + y.abs() + z.abs();
                        best = (x, y, z);
                    } else if count == target_count {
                        // tie breaks, pick closest to origin
                        if x.abs() + y.abs() + z.abs() < best_val {
                            best_val = x.abs() + y.abs() + z.abs();
                            best = (x, y, z);
                        }
                    }
                }
            }
        }

        if range == 1 {
            return best_val;
        }

        xs = (best.0 - range, best.0 + range);
        ys = (best.1 - range, best.1 + range);
        zs = (best.2 - range, best.2 + range);

        range /= 2;
    }
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_01, part_02_z3, INPUT};
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

    #[test]
    fn test_part_02() {
        assert_eq!(36, part_02_z3(TEST_INPUT_2));
    }
}
