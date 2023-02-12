use std::{
    collections::{BTreeSet, HashMap},
    ops::RangeInclusive,
};

static INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Field<'a> {
    name: &'a str,
    ranges: Vec<RangeInclusive<u64>>,
}

impl<'a> Field<'a> {
    fn parse_str(line: &'a str) -> Self {
        let parts: Vec<_> = line.split(": ").collect();
        let name = parts[0];
        let ranges = parts[1]
            .split(" or ")
            .map(|r| {
                let bounds: Vec<_> = r.split('-').collect();
                RangeInclusive::new(bounds[0].parse().unwrap(), bounds[1].parse().unwrap())
            })
            .collect();
        Self { name, ranges }
    }

    fn is_compatible(&self, v: &u64) -> bool {
        self.ranges.iter().any(|r| r.contains(v))
    }
}

#[derive(Debug)]
struct Input<'a> {
    fields: Vec<Field<'a>>,
    my_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

impl<'a> Input<'a> {
    fn parse_str(input: &'a str) -> Self {
        let mut lines = input.lines();
        let mut fields = Vec::new();
        for line in &mut lines {
            if line.trim().is_empty() {
                break;
            }
            fields.push(Field::parse_str(line));
        }

        let mut my_ticket = Vec::new();

        for line in &mut lines {
            if line.trim().is_empty() {
                break;
            }
            if line == "your ticket:" {
                continue;
            }

            my_ticket = line.split(',').map(|n| n.parse().unwrap()).collect();
        }

        let mut nearby_tickets = Vec::new();

        for line in lines {
            if line.trim().is_empty() {
                break;
            }
            if line == "nearby tickets:" {
                continue;
            }

            nearby_tickets.push(line.split(',').map(|n| n.parse().unwrap()).collect());
        }

        Self {
            fields,
            my_ticket,
            nearby_tickets,
        }
    }

    fn cleanup(self) -> Self {
        let nearby_tickets = self
            .nearby_tickets
            .into_iter()
            .filter(|t| {
                t.iter()
                    .all(|v| self.fields.iter().any(|f| f.is_compatible(v)))
            })
            .collect();
        Self {
            nearby_tickets,
            ..self
        }
    }
}

fn part_1(input: &str) -> u64 {
    let input = Input::parse_str(input);

    let mut sum = 0;

    for ticket in &input.nearby_tickets {
        for v in ticket {
            if input.fields.iter().all(|f| !f.is_compatible(v)) {
                sum += v;
            }
        }
    }

    sum
}

fn part_2(input: &str) -> u64 {
    let input = Input::parse_str(input).cleanup();

    let mut mappings = HashMap::new();

    for idx in 0..input.my_ticket.len() {
        let entry = mappings.entry(idx).or_insert(BTreeSet::new());
        for f in &input.fields {
            entry.insert(f.name);
        }
    }

    for (idx, v) in input.my_ticket.iter().enumerate() {
        for f in &input.fields {
            if !f.is_compatible(v) {
                mappings.get_mut(&idx).unwrap().remove(f.name);
            }
        }
    }

    for t in &input.nearby_tickets {
        for (idx, v) in t.iter().enumerate() {
            for f in &input.fields {
                if !f.is_compatible(v) {
                    mappings.get_mut(&idx).unwrap().remove(f.name);
                }
            }
        }
    }

    loop {
        if mappings.values().all(|v| v.len() == 1) {
            break;
        }

        // look for singles, and remove them from other mappings

        let singles: Vec<_> = mappings
            .values()
            .filter_map(|v| {
                if v.len() == 1 {
                    Some(*v.iter().next().unwrap())
                } else {
                    None
                }
            })
            .collect();

        let mut removed = false;

        for s in singles {
            for v in mappings.values_mut() {
                if v.len() > 1 {
                    v.remove(s);
                    removed = true;
                }
            }
        }

        if !removed {
            panic!("will need search: {mappings:?}");
        }
    }

    let mappings: HashMap<&str, usize> =
        HashMap::from_iter(mappings.into_iter().map(|(k, v)| (*v.first().unwrap(), k)));

    mappings
        .into_iter()
        .filter_map(|(k, v)| {
            if k.starts_with("departure ") {
                Some(v)
            } else {
                None
            }
        })
        .map(|p| input.my_ticket[p])
        .product()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_part_1() {
        assert_eq!(71, part_1(TEST_INPUT));
        assert_eq!(21071, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3429967441937, part_2(INPUT));
    }
}
