use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 1);

    let input = File::open(&args[1]).unwrap();
    let buffered = BufReader::new(input);

    let mut sheet: Vec<Vec<u64>> = Vec::new();
    for line in buffered.lines() {
        let line = line.unwrap();

        let row = line.split_whitespace().map(|w| w.parse().unwrap()).collect();

        sheet.push(row);
    }

    println!("basic checksum: {}", compute_checksum(&sheet, basic_checksum));
    println!("divisor checksum: {}", compute_checksum(&sheet, divisor_checksum));
}

fn basic_checksum(row: &Vec<u64>) -> u64 {
    let max = *row.iter().max().unwrap();
    let min = *row.iter().min().unwrap();

    max - min
}

fn divisor_checksum(row: &Vec<u64>) -> u64 {
    for (i, n) in row.iter().enumerate() {
        for m in row[i+1..].iter() {
            if n % m == 0 {
                return n / m;
            }

            if m % n == 0 {
                return m / n;
            }
        }
    }
    0
}

fn compute_checksum(sheet: &Vec<Vec<u64>>, check_fn: fn (&Vec<u64>) -> u64) -> u64  {
    let mut checksum = 0;
    for row in sheet {
        checksum += check_fn(row);
    }

    checksum
}
