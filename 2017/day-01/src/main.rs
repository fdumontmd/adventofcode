use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() > 1);
    let input = File::open(&args[1]).unwrap();
    let mut data = Vec::new();

    for b in input.bytes() {
        let b = b.unwrap();
        if '0' as u8 <= b && b <= '9' as u8 {
            data.push((b - '0' as u8) as u64);
        }
    }

    println!("{}", compute_code(&data, 1));
    println!("{}", compute_code(&data, data.len() / 2));
}

fn compute_code(data: &Vec<u64>, offset: usize) -> u64 {
    let mut sum = 0;

    for (i, n) in data.iter().enumerate() {
        if *n == data[ (i + offset) % data.len() ] {
            sum += n;
        }
    }
    return sum;
}
