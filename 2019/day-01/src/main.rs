use aoc_utils::*;
use anyhow::{Context, Result};

fn basic_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}

fn adv_fuel(mass: i64) -> i64 {
    let mut fuel = 0;
    let mut delta = mass;
    loop {
        let add = basic_fuel(delta);
        if add <= 0 {
            return fuel;
        }
        fuel += add;
        delta = add;
    }
}

// this begs being a new utility function
fn total_fuel(f: impl Fn(i64) -> i64) -> Result<i64> {
    let fuel = get_input().lines()
        .map(|line| {
            let line = line?;
            line.parse()
                .with_context(|| format!("cannot parse \"{}\" into number",
                                         line))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter().map(f)
        .sum();
    Ok(fuel)
}

fn main() -> Result<()> {
    println!("Part 1: {}", total_fuel(basic_fuel)?);
    println!("Part 2: {}", total_fuel(adv_fuel)?);
    Ok(())
}
