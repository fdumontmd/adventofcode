use intcode::*;
use anyhow::*;

static INPUT: &str = include_str!("input.txt");

fn part_1() -> Result<MemItem> {
    let mut computer: Computer = INPUT.parse()?;
    computer.add_input(1);
    computer.run();
    if computer.get_output().len() != 1 {
        panic!("incorrect execution of BOOST program");
    }
    Ok(computer.get_output()[0])
}

fn part_2() -> Result<MemItem> {
    let mut computer: Computer = INPUT.parse()?;
    computer.add_input(2);
    computer.run();
    if computer.get_output().len() != 1 {
        panic!("incorrect execution of BOOST program");
    }
    Ok(computer.get_output()[0])
}


fn main() -> Result<()> {
    println!("part 1: {}", part_1()?);
    println!("part 2: {}", part_2()?);
    Ok(())
}
