const INPUT: &str = include_str!("input");

fn day01(input: &str) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;

    for line in input.lines() {
        let commands: Vec<&str> = line.split(' ').collect();
        let command = commands[0];
        let param: usize = commands[1].parse().unwrap();

        match command {
            "forward" => x += param,
            "down" => y += param,
            "up" => y -= param,
            _ => unreachable!("unknown command {}", line),
        }
    }

    x * y
}

fn day02(input: &str) -> usize {
    let mut x: usize = 0;
    let mut d: usize = 0;
    let mut a: usize = 0;


    for line in input.lines() {
        let commands: Vec<&str> = line.split(' ').collect();
        let command = commands[0];
        let param: usize = commands[1].parse().unwrap();

        match command {
            "forward" => { 
                x += param;
                d += param * a;
            }
            "down" => a += param,
            "up" => a -= param,
            _ => unreachable!("unknown command {}", line),
        }
    }

    x * d
}

fn main() {
    println!("day 1: {}", day01(INPUT));
    println!("day 2: {}", day02(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    #[test]
    fn test_day_01() {
        assert_eq!(150, day01(TEST));
    }

    #[test]
    fn test_day_02() {
        assert_eq!(900, day02(TEST));
    }
}
