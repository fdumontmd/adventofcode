use std::error::Error;
use std::io::Read;

use aoc_utils::get_input;

fn input() -> Result<String, Box<dyn Error>> {
    let mut buf = String::new();
    get_input().read_to_string(&mut buf)?;
    Ok(buf.trim().into())
}

fn fully_react(s: &String) -> String {
    let mut current = s.clone();
    loop {
        let new = current
            .replace("aA", "")
            .replace("Aa", "")
            .replace("bB", "")
            .replace("Bb", "")
            .replace("cC", "")
            .replace("Cc", "")
            .replace("dD", "")
            .replace("Dd", "")
            .replace("eE", "")
            .replace("Ee", "")
            .replace("fF", "")
            .replace("Ff", "")
            .replace("gG", "")
            .replace("Gg", "")
            .replace("hH", "")
            .replace("Hh", "")
            .replace("iI", "")
            .replace("Ii", "")
            .replace("jJ", "")
            .replace("Jj", "")
            .replace("kK", "")
            .replace("Kk", "")
            .replace("lL", "")
            .replace("Ll", "")
            .replace("mM", "")
            .replace("Mm", "")
            .replace("nN", "")
            .replace("Nn", "")
            .replace("oO", "")
            .replace("Oo", "")
            .replace("pP", "")
            .replace("Pp", "")
            .replace("qQ", "")
            .replace("Qq", "")
            .replace("rR", "")
            .replace("Rr", "")
            .replace("sS", "")
            .replace("Ss", "")
            .replace("tT", "")
            .replace("Tt", "")
            .replace("uU", "")
            .replace("Uu", "")
            .replace("vV", "")
            .replace("Vv", "")
            .replace("wW", "")
            .replace("Ww", "")
            .replace("xX", "")
            .replace("Xx", "")
            .replace("yY", "")
            .replace("Yy", "")
            .replace("zZ", "")
            .replace("Zz", "");
        if current.len() == new.len() {
            return new;
        }
        current = new;
    }
}

fn part_two(s: &String) -> usize {
    (0..26).map(|unit| {
        let unit = (unit + b'a') as char;
        let upper = unit.to_uppercase().next().unwrap();

        let new = s.replace(unit, "").replace(upper, "");
        fully_react(&new).len()
    }).min().unwrap()
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = input()?;
    let reduced = fully_react(&input);
    println!("final size after reduction: {}", reduced.len());
    println!("minimal possible size: {}", part_two(&reduced));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_one() {
        let input: String = "dabAcCaCBAcCcaDA".into();
        assert_eq!(fully_react(&input).len(), 10);
    }

    #[test]
    fn test_part_two() {
        let input: String = "dabAcCaCBAcCcaDA".into();
        assert_eq!(part_two(&input), 4);
    }
}
