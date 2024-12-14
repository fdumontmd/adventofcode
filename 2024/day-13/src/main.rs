use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{preceded, separated},
    token::literal,
    PResult, Parser,
};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    x: i64,
    y: i64,
}

impl Machine {
    fn solve(&self) -> Option<(i64, i64)> {
        let num = self.x * self.by - self.y * self.bx;
        let denom = self.ax * self.by - self.ay * self.bx;

        if denom != 0 && num % denom == 0 {
            let a = num / denom;
            if (self.x - a * self.ax) % self.bx == 0 {
                let b = (self.x - a * self.ax) / self.bx;
                Some((a, b))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn cost(&self) -> i64 {
        if let Some((a, b)) = self.solve() {
            3 * a + b
        } else {
            0
        }
    }
}

fn parse_machine(input: &mut &str) -> PResult<Machine> {
    let (ax, ay) = parse_button("A").parse_next(input)?;
    line_ending.parse_next(input)?;
    let (bx, by) = parse_button("B").parse_next(input)?;
    line_ending.parse_next(input)?;
    let (x, y) = parse_prize(input)?;
    line_ending.parse_next(input)?;

    Ok(Machine {
        ax,
        ay,
        bx,
        by,
        x,
        y,
    })
}

fn parse_prize(input: &mut &str) -> PResult<(i64, i64)> {
    preceded(
        literal("Prize: "),
        (
            preceded(literal("X="), dec_int),
            preceded(literal(", Y="), dec_int),
        ),
    )
    .parse_next(input)
}

fn parse_button(button: &'static str) -> impl FnMut(&mut &str) -> PResult<(i64, i64)> {
    move |input: &mut &str| {
        preceded(
            (literal("Button "), literal(button), literal(": ")),
            (
                preceded(literal("X"), dec_int),
                preceded(literal(", Y"), dec_int),
            ),
        )
        .parse_next(input)
    }
}

fn parse(input: &str) -> Vec<Machine> {
    separated(0.., parse_machine, "\n").parse(input).unwrap()
}

fn part1(input: &str) -> i64 {
    parse(input).into_iter().map(|m| m.cost()).sum()
}

fn part2(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|mut m| {
            m.x += 10000000000000;
            m.y += 10000000000000;
            m.cost()
        })
        .sum()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test_case(TEST_INPUT, 480; "test input")]
    #[test_case(INPUT, 36954; "input")]
    fn test_part1(input: &str, cost: i64) {
        assert_eq!(cost, part1(input));
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_button("A").parse("Button A: X+94, Y+34"),
            Ok((94, 34))
        );
        assert_eq!(
            parse_button("B").parse("Button B: X+22, Y+67"),
            Ok((22, 67))
        );
        assert_eq!(parse_prize.parse("Prize: X=8400, Y=5400"), Ok((8400, 5400)));
    }

    #[test_case(INPUT, 79352015273424; "input")]
    fn test_part2(input: &str, cost: i64) {
        assert_eq!(cost, part2(input));
    }
}
