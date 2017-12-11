use std::io::{self, Read};
use std::str::FromStr;

fn possible_triangle(a: i32, b: i32, c: i32) -> bool {
    a < b + c && b < a + c && c < a + b 
}

fn possible_triangle_and_clear(c: &mut Vec<i32>) -> bool {
    if c.len() == 3 {
        let res = possible_triangle(c.pop().unwrap(), c.pop().unwrap(), c.pop().unwrap());
        c.clear();
        res
    } else {
        false
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut count = 0;

    for line in buffer.lines() {
        let mut digits = line.split_whitespace();
        let a = i32::from_str(digits.next().unwrap()).unwrap();
        let b = i32::from_str(digits.next().unwrap()).unwrap();
        let c = i32::from_str(digits.next().unwrap()).unwrap();

        if possible_triangle(a, b,c ) {
            count += 1;
        }
    }

    println!("Possible triangles - horizontal: {}", count);

    count = 0;

    let mut c1 = Vec::new();
    let mut c2 = Vec::new();
    let mut c3 = Vec::new();

    for line in buffer.lines() {
        let mut digits = line.split_whitespace();
        let d1 = i32::from_str(digits.next().unwrap()).unwrap();
        let d2 = i32::from_str(digits.next().unwrap()).unwrap();
        let d3 = i32::from_str(digits.next().unwrap()).unwrap();

        c1.push(d1);
        c2.push(d2);
        c3.push(d3);

        if possible_triangle_and_clear(&mut c1) {
            count += 1;
        }

        if possible_triangle_and_clear(&mut c2) {
            count += 1;
        }

        if possible_triangle_and_clear(&mut c3) {
            count += 1;
        }
    }

    println!("Possible triangles - vertical: {}", count);
}
