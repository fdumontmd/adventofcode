use color_eyre::{eyre::bail, Report, Result};
use itertools::Itertools;
use std::{cmp::Reverse, collections::VecDeque, num::ParseIntError};

static INPUT: &str = include_str!("input.txt");
type Item = u64;

#[derive(Debug)]
enum Operation {
    Add(Item),
    MultBy(Item),
    Square,
}

impl Operation {
    fn inspect(&self, item: Item) -> Item {
        match self {
            Operation::Add(v) => item + v,
            Operation::MultBy(v) => item * v,
            Operation::Square => item * item,
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "old * old" {
            Ok(Operation::Square)
        } else if let Some(number) = value.strip_prefix("old * ") {
            number.parse().map(Operation::MultBy).map_err(From::from)
        } else if let Some(number) = value.strip_prefix("old + ") {
            number.parse().map(Operation::Add).map_err(From::from)
        } else {
            bail!("Cannot parse Operation {}", value);
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<Item>,
    operation: Operation,
    divisibility_test: Item,
    next_if_true: usize,
    next_if_false: usize,
    inspections: usize,
    relief: Item,
}

// not learning Nom for this
impl TryFrom<&str> for Monkey {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = value.lines().collect();
        if lines.len() != 6 {
            bail!(
                "Cannot parse Monkey - missing lines: {} instead of 6 - {}",
                lines.len(),
                value
            );
        }

        let lines = &lines[0..6];
        let id: usize = match lines[0].strip_prefix("Monkey ") {
            Some(id) => {
                if id.ends_with(':') {
                    id[0..id.len() - 1].parse()?
                } else {
                    bail!("Cannot parse Monkey ident: {}", lines[0]);
                }
            }
            None => bail!("Cannot parse Monkey ident: {}", lines[0]),
        };

        let items = match lines[1].trim().strip_prefix("Starting items: ") {
            Some(items) => items
                .split(", ")
                .map(|i| i.parse().map_err(From::from))
                .collect::<Result<_>>()?,
            None => bail!("Cannot parse starting items: {}", lines[1]),
        };

        let operation = match lines[2].trim().strip_prefix("Operation: new = ") {
            Some(operation) => Operation::try_from(operation)?,
            None => bail!("Cannot parse operation: {}", lines[2]),
        };

        let divisibility_test: Item = match lines[3].trim().strip_prefix("Test: divisible by ") {
            Some(div_test) => div_test.parse()?,
            None => bail!("Cannot parse divisibility test: {}", lines[3]),
        };

        let next_if_true: usize = match lines[4].trim().strip_prefix("If true: throw to monkey ") {
            Some(monkey) => monkey.parse()?,
            None => bail!("Cannot parse throw to monkey if true: {}", lines[4]),
        };

        let next_if_false: usize = match lines[5].trim().strip_prefix("If false: throw to monkey ")
        {
            Some(monkey) => monkey.parse()?,
            None => bail!("Cannot parse throw to monkey if false: {}", lines[5]),
        };

        Ok(Monkey {
            id,
            items,
            operation,
            divisibility_test,
            next_if_true,
            next_if_false,
            inspections: 0,
            relief: 3,
        })
    }
}

impl Monkey {
    fn next_in_round(&mut self) -> Option<(usize, Item)> {
        if !self.items.is_empty() {
            self.inspections += 1;
        }
        self.items
            .pop_front()
            .map(|i| self.operation.inspect(i))
            .map(|i| i / self.relief)
            .map(|i| (self.throw_to(i), i))
    }

    fn throw_to(&self, item: Item) -> usize {
        if item % self.divisibility_test == 0 {
            self.next_if_true
        } else {
            self.next_if_false
        }
    }

    fn catch(&mut self, item: Item) {
        self.items.push_back(item)
    }

    fn set_relief(&mut self, relief: Item) {
        self.relief = relief;
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    cm: Item,
}

impl Monkeys {
    fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            while let Some((next, mut item)) = self.monkeys[idx].next_in_round() {
                // why does it work only when no relief? because division introduces
                // rounding, which breaks the relation between common multiples
                // and the item worry level
                if self.monkeys[idx].relief == 1 {
                    item %= self.cm;
                }
                self.monkeys[next].catch(item);
            }
        }
    }

    fn rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.round();
        }
    }

    fn inspections(&self) -> impl Iterator<Item = usize> + '_ {
        self.monkeys.iter().map(|m| m.inspections)
    }

    fn worry(&mut self) {
        self.monkeys.iter_mut().for_each(|m| m.set_relief(1));
    }

    fn monkey_business(&self) -> usize {
        self.inspections()
            .map(Reverse)
            .k_smallest(2)
            .map(|i| i.0)
            .product()
    }
}

impl TryFrom<&str> for Monkeys {
    type Error = Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.replace("\r\n", "\n");
        let mut monkeys: Vec<Monkey> = value
            .split("\n\n")
            .map(Monkey::try_from)
            .collect::<Result<_>>()?;
        monkeys.sort_by_key(|m| m.id);
        let cm = monkeys.iter().map(|m| m.divisibility_test).product();
        Ok(Monkeys { monkeys, cm })
    }
}

fn part_01(input: &str) -> Result<usize> {
    let mut monkeys = Monkeys::try_from(input)?;
    monkeys.rounds(20);

    Ok(monkeys.monkey_business())
}

fn part_02(input: &str) -> Result<usize> {
    let mut monkeys = Monkeys::try_from(input)?;
    monkeys.worry();
    monkeys.rounds(10000);

    Ok(monkeys.monkey_business())
}

fn main() -> Result<()> {
    println!("Part 1: {}", part_01(INPUT)?);
    println!("Part 2: {}", part_02(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02, INPUT};

    static TEST_INPUT: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_01() {
        assert_eq!(10605, part_01(TEST_INPUT).unwrap());
    }

    #[test]
    fn real_part_01() {
        assert_eq!(50616, part_01(INPUT).unwrap());
    }

    #[test]
    fn test_part_02() {
        assert_eq!(2713310158, part_02(TEST_INPUT).unwrap());
    }

    #[test]
    fn real_part_02() {
        assert_eq!(11309046332, part_02(INPUT).unwrap());
    }
}
