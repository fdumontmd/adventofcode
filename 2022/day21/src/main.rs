use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Operator {
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Plus => lhs + rhs,
            Operator::Minus => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
        }
    }

    fn parse(code: &str) -> Operator {
        match code {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => panic!("{} is not a valid operator", code),
        }
    }

    fn invert_left(&self, target: i64, rhs: i64) -> i64 {
        match self {
            Operator::Plus => target - rhs,
            Operator::Minus => target + rhs,
            Operator::Mul => target / rhs,
            Operator::Div => target * rhs,
        }
    }

    fn invert_right(&self, target: i64, lhs: i64) -> i64 {
        match self {
            Operator::Plus => target - lhs,
            Operator::Minus => -target + lhs,
            Operator::Mul => target / lhs,
            Operator::Div => lhs / target,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "div"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Monkey {
    Number(i64),
    Operation(String, Operator, String),
}

impl Monkey {
    fn eval(&self, monkeys: &mut BTreeMap<String, Monkey>) -> i64 {
        match self {
            Monkey::Number(n) => *n,
            Monkey::Operation(lhs_id, op, rhs_id) => {
                let Some(lhs) = monkeys.remove(lhs_id) else { panic!("Monkey {} not found", lhs_id)};
                let lhs_num = lhs.eval(monkeys);
                monkeys.insert(lhs_id.clone(), Monkey::Number(lhs_num));
                let Some(rhs) = monkeys.remove(rhs_id) else { panic!("Monkey {} not found", rhs_id)};
                let rhs_num = rhs.eval(monkeys);
                monkeys.insert(rhs_id.clone(), Monkey::Number(rhs_num));
                op.eval(lhs_num, rhs_num)
            }
        }
    }
}

fn build_monkeys(input: &str) -> BTreeMap<String, Monkey> {
    let mut monkeys = BTreeMap::new();

    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .for_each(|l| {
            let monkey: Vec<&str> = l.split(": ").collect();
            let monkey = &monkey[0..2];
            let id = monkey[0].to_string();

            let code: Vec<&str> = monkey[1].split(' ').collect();
            let m = match code.len() {
                1 => Monkey::Number(code[0].parse().unwrap()),
                3 => Monkey::Operation(
                    code[0].to_string(),
                    Operator::parse(code[1]),
                    code[2].to_string(),
                ),
                _ => panic!("Cannot parse line {}", l),
            };
            monkeys.insert(id, m);
        });

    monkeys
}

fn eval_monkey(key: String, monkeys: &mut BTreeMap<String, Monkey>) -> i64 {
    let monkey = monkeys.remove(&key).unwrap();
    let value = monkey.eval(monkeys);
    monkeys.insert(key, Monkey::Number(value));
    value
}

fn part_01(input: &str) -> i64 {
    let mut monkeys = build_monkeys(input);
    eval_monkey("root".to_string(), &mut monkeys)
}

// constraint programming is a bust; can't even model part 1 correctly (or maybe minizinc can't
// solve it)
// new idea: looks like humn is only used once; and the value it defines is only used once, and so
// on. So maybe the problem is not really hard:
// 1. compute the part that does not lead to humn;
// 2. look up the other, and repeat, solve as you go
// 3. once we bottom out on humn, we emit the value we have
//
// root: aa == bb
// aa: cc + dd
// bb: ee * ff
// ee: 3
// ff: 5
// cc: 6
// dd: humn + gg
// gg: 2
//
// 1: bb <- 15
// 2: cc + dd == 15
// cc <- 6 => dd = 9
// dd = humn + gg => hum + gg = 9
// gg <- 2 => hum = 7 (I think, doing this by hand)

// build a set of nodes on the way to humn
fn find(start: &String, monkeys: &BTreeMap<String, Monkey>, key: &String) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();

    fn found_under(
        node: &String,
        monkeys: &BTreeMap<String, Monkey>,
        key: &String,
        paths: &mut BTreeSet<String>,
    ) -> bool {
        if node == key {
            true
        } else {
            match &monkeys[node] {
                Monkey::Number(_) => false,
                Monkey::Operation(lhs, _, rhs) => {
                    if found_under(lhs, monkeys, key, paths) {
                        paths.insert(lhs.to_string());
                        true
                    } else if found_under(rhs, monkeys, key, paths) {
                        paths.insert(rhs.to_string());
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    found_under(start, monkeys, key, &mut paths);

    paths
}

fn part_02(input: &str) -> i64 {
    let mut monkeys = build_monkeys(input);
    let paths_to_humns = find(&"root".to_string(), &monkeys, &"humn".to_string());

    let root = monkeys.remove(&"root".to_string());

    let Monkey::Operation(lhs,_,rhs) = root.unwrap() else { panic!("root is not an operation")};
    let (path, other) = if paths_to_humns.contains(&lhs) {
        (lhs, rhs)
    } else if paths_to_humns.contains(&rhs) {
        (rhs, lhs)
    } else {
        panic!("path humn not found");
    };

    assert!(!paths_to_humns.contains(&other));
    let mut target = eval_monkey(other, &mut monkeys);

    let mut path = path;

    loop {
        match monkeys.remove(&path).unwrap() {
            Monkey::Number(_) => {
                if &path == "humn" {
                    break;
                }
                unreachable!("should never hit a Number on a path to humn");
            }
            Monkey::Operation(lhs, op, rhs) => {
                target = if paths_to_humns.contains(&lhs) {
                    path = lhs;
                    op.invert_left(target, eval_monkey(rhs, &mut monkeys))
                } else if paths_to_humns.contains(&rhs) {
                    path = rhs;
                    op.invert_right(target, eval_monkey(lhs, &mut monkeys))
                } else {
                    panic!("path humn not found");
                };
            }
        }
    }

    target
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02, INPUT};

    static TEST_INPUT: &str = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_part_01() {
        assert_eq!(152, part_01(TEST_INPUT));
    }

    #[test]
    fn test_part_02() {
        assert_eq!(301, part_02(TEST_INPUT));
    }

    #[test]
    fn real_part_01() {
        assert_eq!(168502451381566, part_01(INPUT));
    }

    #[test]
    fn real_part_02() {
        assert_eq!(3343167719435, part_02(INPUT));
    }
}
