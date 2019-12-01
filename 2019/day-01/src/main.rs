use std::error::Error;
use std::result::Result;
use aoc_utils::*;

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
fn total_fuel<F>(f: F) -> Result<i64, Box<dyn Error>>
    where F: Fn(i64) -> i64 {
    let fuel = get_input().lines()
        .map(|line| {
            line.map_err(|e| Box::new(e) as Box<dyn Error>)?
                .parse()
                .map_err(|e| Box::new(e) as Box<dyn Error>)

        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter().map(f)
        .sum();
    Ok(fuel)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", total_fuel(basic_fuel)?);
    println!("Part 2: {}", total_fuel(adv_fuel)?);
    Ok(())
}
