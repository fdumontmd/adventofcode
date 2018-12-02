use std::collections::HashSet;
use std::error::Error;
use std::io::BufRead;
use aoc_utils::get_input;

fn parse_input() -> Result<Vec<i32>, Box<dyn Error>> {
    let mut v = Vec::new();
    for line in get_input().lines() {
        let line = line?;
        v.push(line.parse()?);
    }
    Ok(v)
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = parse_input()?;
    println!("frequency: {}", input.clone().into_iter().sum::<i32>());

    let mut seen = HashSet::new();
    let mut sum = 0;

    for v in input.into_iter().cycle() {
        seen.insert(sum);
        sum += v;
        if seen.contains(&sum) {
            println!("Repeated frequency: {}", sum);
            break;
        }
    }
    Ok(())
}
