use anyhow::{anyhow, Result};
use aoc_utils::*;
use intcode::*;
use std::io::Read;

fn part_1(buf: &String) -> Result<MemItem> {
    let mut computer: Computer = buf.parse()?;
    computer.set_noun(12);
    computer.set_verb(2);
    computer.run();

    Ok(computer[0])
}

fn part_2(buf: &String, target: MemItem) -> Result<String> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer: Computer = buf.parse()?;
            computer.set_noun(noun);
            computer.set_verb(verb);
            computer.run();
            if computer[0] == target {
                return Ok(format!("{}{}", noun, verb));
            }
        }
    }
    Err(anyhow!("Could not find noun - verb to match target {}", target))
}

fn main() -> Result<()> {
    let mut buf = String::new();
    get_input().read_to_string(&mut buf)?;
    println!("part 1: location 0 = {}", part_1(&buf)?);
    println!("part 2: {}", part_2(&buf, 19690720)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let mut buf = String::new();
        get_input().read_to_string(&mut buf)?;
        assert_eq!(part_1(&buf)?, 4930687);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let mut buf = String::new();
        get_input().read_to_string(&mut buf)?;
        assert_eq!(part_2(&buf, 19690720)?, "5335");
        Ok(())
    }
}
