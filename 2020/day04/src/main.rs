use std::collections::HashMap;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<HashMap<String, String>> {
    let mut passports = Vec::new();
    let mut passport = HashMap::new();

    let re = Regex::new("([[:alpha:]]+):(\\S+)\\s?").unwrap();

    for line in input.lines() {
        if line.is_empty() {
            if !passport.is_empty() {
                passports.push(passport);
                passport = HashMap::new();
            }
        } else {
            for cap in re.captures_iter(line) {
                passport.insert(cap[1].to_string(), cap[2].to_string());
            }
        }
    }

    if !passport.is_empty() {
        passports.push(passport);
    }
    passports
}

fn fields_present(p: &HashMap<String, String>) -> bool {
    p.contains_key("byr") &&
        p.contains_key("iyr") &&
        p.contains_key("eyr") &&
        p.contains_key("hgt") &&
        p.contains_key("hcl") &&
        p.contains_key("ecl") &&
        p.contains_key("pid")
}

fn check_in_range(number: &str, min: usize, max: usize) -> bool {
    number.parse::<usize>().map(|y| min <= y && y <= max).unwrap_or(false)
}

fn valid_height(h: &str) -> bool {
    if h.len() > 2 {
        let start = h.len() - 2;
        match (&h[start..], &h[0..start]) {
            ("cm", n) => check_in_range(n, 150, 193),
            ("in", n) => check_in_range(n, 59, 76),
            _ => false
        }
    } else {
        false
    }
}

fn valid_color(c: &str) -> bool {
    let re = Regex::new("#[0-9a-f]{6}").unwrap();
    re.is_match(c) && c.len() == 7
}

fn valid_eye_color(c: &str) -> bool {
    let re = Regex::new("amb|blu|brn|gry|grn|hzl|oth").unwrap();
    re.is_match(c) && c.len() == 3
}

fn valid_pid(p: &str) -> bool {
    let re = Regex::new("\\d{9}").unwrap();
    re.is_match(p) && p.len() == 9
}

fn valid_info(p: &HashMap<String, String>) -> bool {
    p.get("byr").map(|y| check_in_range(y, 1920, 2002)).unwrap_or(false)
    && p.get("iyr").map(|y| check_in_range(y, 2010, 2020)).unwrap_or(false) 
    && p.get("eyr").map(|y| check_in_range(y, 2020, 2030)).unwrap_or(false)
    && p.get("hgt").map(|h| valid_height(h)).unwrap_or(false)
    && p.get("hcl").map(|c| valid_color(c)).unwrap_or(false)
    && p.get("ecl").map(|c| valid_eye_color(c)).unwrap_or(false)
    && p.get("pid").map(|p| valid_pid(p)).unwrap_or(false)
}

fn part1(data: &Vec<HashMap<String, String>>) -> usize {
    data.iter().filter(|p| fields_present(p)).count()
}

fn part2(data: &Vec<HashMap<String, String>>) -> usize {
    data.iter().filter(|p| valid_info(p)).count()
}

fn main() {
    println!("part 1: {}", part1(&parse_input(INPUT)));
    println!("part 2: {}", part2(&parse_input(INPUT)));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

    #[test]
    fn check_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 2);
    }

    const ALL_INVALID: &str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;

    #[test]
    fn check_invalid() {
        assert_eq!(part2(&parse_input(ALL_INVALID)), 0);
    }

    const ALL_VALID: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;

    #[test]
    fn check_valid() {
        assert_eq!(part2(&parse_input(ALL_VALID)), 4);
    }
}
