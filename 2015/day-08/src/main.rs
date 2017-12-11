use std::io::{self, Read};

fn bytes_count(input: &str) -> usize {
    let mut bytes_count = 0;
    // strip starting and closing double quotes
    let input = &input[1..input.len()-1];
    let mut skip_count = 0;
    let mut escape = false;
    for c in input.chars() {
        if escape {
            skip_count = match c {
                '"' | '\\' => 0,
                'x' => 2,
                _ => {
                    println!("unrecognized escape code: {}", c);
                    unreachable!()
                }
            };
            bytes_count += 1;
            escape = false;
        }  else if skip_count > 0 {
            skip_count -= 1;
        } else if c == '\\' {
            escape = true;
        } else {
            bytes_count += 1;
        }
    }
    bytes_count
}

fn encoded_bytes_count(input: &str) -> usize {
    // start at 2 because of opening and closing double quotes
    let mut bytes_count = 2;
    for c in input.chars() {
        bytes_count += match c {
            '\\' | '"' => 2,
            _ => 1,
        }
    }
    bytes_count
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    let mut code_count = 0;
    let mut memory_count = 0;
    let mut encoded_count = 0;

    for line in buffer.lines() {
        let line = line.trim();
        code_count += line.len();
        memory_count += bytes_count(line);
        encoded_count += encoded_bytes_count(line);
    }
    println!("Code count: {}, memory count: {}, difference: {}",
             code_count, memory_count, code_count - memory_count);
    println!("Encoded count: {}, memory count: {}, difference: {}",
             encoded_count, code_count, encoded_count - code_count);
}
