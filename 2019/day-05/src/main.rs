use intcode::*;
use anyhow::*;
use aoc_utils::*;

use std::io::Read;

fn main() -> Result<()> {
    let mut buf = String::new();
    get_input().read_to_string(&mut buf)?;
    let mut computer: Computer = buf.parse()?;
    computer.run();
    println!("Part 1: output: {:?}", computer.get_output());


    let mut computer: Computer = buf.parse()?;
    computer.set_input(5);
    computer.run();
    println!("Part 2: output: {:?}", computer.get_output());
    Ok(())
}
