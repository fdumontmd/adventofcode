use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn evaluate(&self, left: bool, right: bool) -> bool {
        match self {
            Operator::And => left && right,
            Operator::Or => left || right,
            Operator::Xor => left ^ right,
        }
    }

    fn from_str(op: &str) -> Self {
        match op {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("unknown operator {op}"),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::And => "AND",
                Operator::Or => "OR",
                Operator::Xor => "XOR",
            }
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Expression<'a> {
    left: &'a str,
    operator: Operator,
    right: &'a str,
    output: &'a str,
    evaluated: bool,
}

impl<'a> Expression<'a> {
    fn new(left: &'a str, operator: Operator, right: &'a str, output: &'a str) -> Self {
        let (left, right) = if left < right {
            (left, right)
        } else {
            (right, left)
        };
        Expression {
            left,
            operator,
            right,
            output,
            evaluated: false,
        }
    }

    fn evaluate(&mut self, store: &mut HashMap<&'a str, Option<bool>>) {
        if !self.evaluated {
            if let Some(left) = store[self.left] {
                if let Some(right) = store[self.right] {
                    if store[self.output].is_none() {
                        store.insert(self.output, Some(self.operator.evaluate(left, right)));
                    }
                    self.evaluated = true;
                }
            }
        }
    }

    fn is_input(&self, input: &str) -> bool {
        self.left == input || self.right == input
    }

    fn is_output(&self, output: &str) -> bool {
        self.output == output
    }
}

impl<'a> TryFrom<&'a str> for Expression<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let tokens: Vec<&str> = value.split_whitespace().collect();
        if tokens.len() == 5 {
            let op = Operator::from_str(tokens[1]);
            Ok(Self::new(tokens[0], op, tokens[2], tokens[4]))
        } else {
            Err(format!("cannot parse {value}"))
        }
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.left, self.operator, self.right, self.output
        )
    }
}

// dumb works well enough
fn part1(input: &str) -> u64 {
    let mut store: HashMap<&str, Option<bool>> = HashMap::new();
    let parts: Vec<&str> = input.split("\n\n").collect();

    for line in parts[0].lines() {
        let init: Vec<&str> = line.split(": ").collect();
        store.insert(init[0], Some(init[1] == "1"));
    }

    let mut expressions: Vec<Expression> = parts[1]
        .lines()
        .map(|line| {
            let e = Expression::try_from(line).unwrap();
            store.entry(e.left).or_insert(None);
            store.entry(e.right).or_insert(None);
            store.entry(e.output).or_insert(None);

            e
        })
        .collect();

    while !expressions.is_empty() {
        for e in &mut expressions {
            e.evaluate(&mut store);
        }
        // should check instead whether all z stores are evaluated
        expressions.retain(|e| !e.evaluated);

        if store
            .iter()
            .filter(|(k, _)| k.starts_with('z'))
            .all(|(_, v)| v.is_some())
        {
            break;
        }
    }

    let mut zs: Vec<(&str, Option<bool>)> = store
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect();
    zs.sort();
    zs.reverse();

    zs.into_iter()
        .fold(0, |acc, (_, v)| acc * 2 + if v.unwrap() { 1 } else { 0 })
}

// it's an addition between any two 45 bit numbers, so the expressions can be predicted
// for each wire, check whether they can be assigned a role based on the overall adder schema
// presumably, at the end there will be a search?
//
// from x_i and y_i, we should be able to forward everything and check on the fly
// for x00 and y00, we should have x00 XOR y00 -> z00 and x00 AND y01 -> c01
// the for x_i and y_i:
//
// x_i XOR y_i -> SimpleAdd(i)
// c_i XOR SimpleAdd(i) -> z_i
// x_i AND y_i -> SimpleCarry(i)
// c_i OR SimpleCarry(i) -> c_(i+1)
//
// process the rules in the order of index, starting with x00 and y00 as input, identifying
// likely c01 and so on
// if a rule involves one incorrect input, that input's formula is swapped. If both inputs
// match but the output is wrong, the output is swapped
//
//
// annoyingly, the test input has a different formula between inputs and outputs
// cannot really use without introducing some concept of gate schema, which I really don't
// want to do
fn part2(input: &str, pairs: usize) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();

    // each x_i and y_i, and nothing else
    let bitlen = parts[0].lines().count() as u8 / 2;

    let formulae: Vec<Expression> = parts[1]
        .lines()
        .map(|l| Expression::try_from(l).unwrap())
        .collect();

    let mut swapped = BTreeSet::new();
    let z00 = formulae
        .iter()
        .find(|e| e.left == "x00" && e.right == "y00" && e.operator == Operator::Xor)
        .unwrap();

    if z00.output != "z00" {
        swapped.insert(z00.output.to_string());
    }

    let mut carry: &str = formulae
        .iter()
        .find_map(|e| {
            if e.left == "x00" && e.right == "y00" && e.operator == Operator::And {
                Some(e.output)
            } else {
                None
            }
        })
        .unwrap();

    for bit in 1..bitlen {
        // find basic add x_bit XOR y_bit -> ??
        let x = format!("x{bit:02}");
        let y = format!("y{bit:02}");
        let z = format!("z{bit:02}");
        let basic_add = formulae
            .iter()
            .find(|e| e.left == x && e.right == y && e.operator == Operator::Xor)
            .unwrap()
            .output;
        // check Add (either previous carry, basic add or output can be wrong)

        let add = formulae
            .iter()
            .find(|e| e.operator == Operator::Xor && (e.is_input(carry) || e.is_input(basic_add)))
            .unwrap();
        if !add.is_output(&z) {
            swapped.insert(z);
            swapped.insert(add.output.to_string());
        }

        if !add.is_input(basic_add) {
            swapped.insert(basic_add.to_string());
        }

        if !add.is_input(carry) {
            swapped.insert(carry.to_string());
        }
        // check basic carry - only output can be wrong
        let basic_carry = formulae
            .iter()
            .find(|e| e.left == x && e.right == y && e.operator == Operator::And)
            .unwrap()
            .output;
        // check cascade carry (if either previous carry or basic add were wrong, ignore that)
        // if carry was wrong, basic_add could also be wrong... let's ignore that for now
        let cascade_carry = formulae
            .iter()
            .find(|e| e.operator == Operator::And && (e.is_input(basic_add) || e.is_input(carry)))
            .unwrap();

        if !cascade_carry.is_input(basic_add) {
            swapped.insert(basic_add.to_string());
        }

        if !cascade_carry.is_input(carry) {
            swapped.contains(carry);
        }
        // check carry (basic carry or cascade carry can be wrong)
        let carry_gate = formulae
            .iter()
            .find(|e| {
                e.operator == Operator::Or
                    && (e.is_input(cascade_carry.output) || e.is_input(basic_carry))
            })
            .unwrap();

        if !carry_gate.is_input(cascade_carry.output) {
            swapped.insert(cascade_carry.output.to_string());
        }

        if !carry_gate.is_input(basic_carry) {
            swapped.insert(basic_carry.to_string());
        }

        carry = carry_gate.output;
    }

    dbg!(&swapped);

    assert_eq!(pairs * 2, swapped.len());

    let swapped: Vec<_> = swapped.into_iter().collect();
    swapped.join(",")
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT, 4));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT_SMALL: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const TEST_INPUT_LARGE: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test_case(TEST_INPUT_SMALL, 4; "small test input")]
    #[test_case(TEST_INPUT_LARGE, 2024; "large test input")]
    fn test_part1(input: &str, output: u64) {
        assert_eq!(output, part1(input));
    }

    const TEST_INPUT_PART_2: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

    /* not sure how to use the test input... rules are not the same
    #[test_case(TEST_INPUT_PART_2, 2, "z00,z01,z02,z05"; "test input")]
    fn test_part2(input: &str, pairs: usize, swapped: &str) {
        assert_eq!(swapped, part2(input, pairs));
    }
    */

    #[test_case(INPUT, 4, "fgt,fpq,nqk,pcp,srn,z07,z24,z32"; "input")]
    fn test_part2(input: &str, pairs: usize, swapped: &str) {
        assert_eq!(swapped, part2(input, pairs));
    }
}
