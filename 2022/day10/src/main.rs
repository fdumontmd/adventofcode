use std::fmt::Write;

use color_eyre::Report;
static INPUT: &str = include_str!("input.txt");

enum Instruction {
    AddX(i64),
    Noop,
}

impl Instruction {
    fn cycle_count(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }

    fn add_to_x(&self) -> i64 {
        match self {
            Instruction::Noop => 0,
            Instruction::AddX(v) => *v,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let line = line.trim();
        if line == "noop" {
            Ok(Instruction::Noop)
        } else if let Some(n) = line.strip_prefix("addx ") {
            if let Ok(v) = n.parse() {
                Ok(Instruction::AddX(v))
            } else {
                Err("cannot parse number")
            }
        } else {
            Err("cannot parse instruction")
        }
    }
}

fn execute_instructions(input: &str) -> impl Iterator<Item = color_eyre::Result<i64>> + '_ {
    let mut x = 1;
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .flat_map(move |l| match Instruction::try_from(l) {
            Ok(i) => {
                let current_x = x;
                x += i.add_to_x();
                let mut v = vec![];
                v.resize_with(i.cycle_count(), || Ok(current_x));
                v.into_iter()
            }
            Err(e) => vec![Err(Report::msg(e))].into_iter(),
        })
}

fn part_01(input: &str) -> color_eyre::Result<i64> {
    execute_instructions(input)
        .enumerate()
        .try_fold(0, |total, (idx, x)| {
            let cycle = (idx + 1) as i64;
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                Ok(total + cycle * x?)
            } else {
                Ok(total)
            }
        })
}

fn part_02(input: &str) -> color_eyre::Result<String> {
    execute_instructions(input)
        .enumerate()
        .try_fold(String::new(), |mut buffer, (cycle, x)| {
            let x = x?;
            let cursor = (cycle % 40) as i64;
            if x - 1 <= cursor && cursor <= x + 1 {
                write!(buffer, "#")?;
            } else {
                write!(buffer, ".")?;
            }
            if (cycle + 1) % 40 == 0 {
                writeln!(buffer)?;
            }
            Ok(buffer)
        })
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    println!("Part 1: {}", part_01(INPUT)?);
    println!("Part 2: \n{}", part_02(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::*;

    static TEST_INPUT: &str = r"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    static TEST_OUTPUT_2: &str = r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn test_part_1() {
        assert_eq!(13140, part_01(TEST_INPUT).unwrap());
    }

    #[test]
    fn real_part_1() {
        assert_eq!(15120, part_01(INPUT).unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(TEST_OUTPUT_2, part_02(TEST_INPUT).unwrap());
    }

    static OUTPUT_2: &str = r"###..#..#.###....##.###..###..#.....##..
#..#.#.#..#..#....#.#..#.#..#.#....#..#.
#..#.##...#..#....#.###..#..#.#....#..#.
###..#.#..###.....#.#..#.###..#....####.
#.#..#.#..#....#..#.#..#.#....#....#..#.
#..#.#..#.#.....##..###..#....####.#..#.
";

    #[test]
    fn real_part_2() {
        assert_eq!(OUTPUT_2, part_02(INPUT).unwrap());
    }
}
