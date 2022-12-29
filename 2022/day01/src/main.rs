const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Elf {
    load: Vec<usize>,
}

impl Elf {
    fn new(load: Vec<usize>) -> Self {
        Elf { load }
    }
    fn total_load(&self) -> usize {
        self.load.iter().sum()
    }
}

fn parse(data: &str) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut load = Vec::new();
    for line in data.lines() {
        if line.trim().is_empty() {
            if !load.is_empty() {
                elves.push(Elf::new(load));
                load = Vec::new();
            }
        } else if let Ok(l) = line.trim().parse() {
            load.push(l);
        } else {
            eprintln!("Cannot parse line {}", line);
        }
    }

    if !load.is_empty() {
        elves.push(Elf::new(load));
    }

    elves
}

fn max_load(data: &[Elf]) -> usize {
    data.iter()
        .map(|e| e.total_load())
        .max()
        .unwrap_or_default()
}

fn top_three(data: &[Elf]) -> usize {
    let mut sorted: Vec<_> = data.iter().map(|e| e.total_load()).collect();
    sorted.sort();
    sorted.reverse();
    sorted.into_iter().take(3).sum()
}

fn main() {
    let data = parse(INPUT);
    println!("Max load: {}", max_load(&data));
    println!("Top three total load {}", top_three(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &str = &r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn test_part_1() {
        let elves = parse(TEST_DATA);
        assert_eq!(24000, max_load(&elves));
    }

    #[test]
    fn test_part_2() {
        let elves = parse(TEST_DATA);
        assert_eq!(45000, top_three(&elves));
    }

    #[test]
    fn real_part_1() {
        let elves = parse(INPUT);
        assert_eq!(68292, max_load(&elves));
    }
    #[test]
    fn real_part_2() {
        let elves = parse(INPUT);
        assert_eq!(203203, top_three(&elves));
    }
}
