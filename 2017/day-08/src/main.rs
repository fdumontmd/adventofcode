extern crate regex;

use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    assert!(args().len() >1);
    let path = args().nth(1).unwrap();

    let input = File::open(path);
    let buf = BufReader::new(input.unwrap());

    let mut registers = HashMap::new();
    let re = Regex::new(r"(\w+) (inc|dec) (-?\d+) if (\w+) (==|!=|>|<|>=|<=) (-?\d+)").unwrap();

    let mut max_ever: i32 = i32::min_value();

    for line in buf.lines() {
        let line = line.unwrap();
        let cap = re.captures(&line).unwrap();
        let reg1 = cap[1].to_owned();
        let reg2 = cap[4].to_owned();
        let reg2_value = *registers.entry(reg2).or_insert(0);
        let num1 = cap[3].parse::<i32>().expect(&line);
        let num2 = cap[6].parse::<i32>().expect(&line);

        if match &cap[5] {
            "==" => reg2_value == num2,
            "!=" => reg2_value != num2,
            ">=" => reg2_value >= num2,
            "<=" => reg2_value <= num2,
            ">" => reg2_value > num2,
            "<" => reg2_value < num2,
            _ => unreachable!(),
        } {
            *registers.entry(reg1).or_insert(0) +=
            match &cap[2] {
                "inc" => num1,
                "dec" => -num1,
                _ => unreachable!(),
            };

            max_ever = max_ever.max(*registers.get(&cap[1]).unwrap());
        }
    }

    println!("max register: {}", registers.values().max().unwrap());
    println!("max ever: {}", max_ever);
}
