extern crate md5;
extern crate hex;

use std::char;

use md5::{Md5, Digest};
use hex::encode;

static KEY: &'static str = "abbhdwsy";

fn is_password_letter(index: u32, key: &str) -> Option<(char, char)> {
    let mut buffer = String::new();
    buffer.push_str(key);
    buffer.push_str(&index.to_string());

    let mut hasher = Md5::new();
    hasher.update(buffer.as_bytes());
    let hash = encode(hasher.finalize());

    if hash.starts_with("00000") {
        let bytes = hash.as_bytes();
        Some((bytes[5] as char, bytes[6] as char) )
    } else {
        None
    }
}

fn char_to_usize(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => unreachable!()
    }
}

fn basic_code(key: &str) -> String {
    let mut code = String::new();

    for i in 0.. {
        if let Some((c, _)) = is_password_letter(i, key) {
            code.push(c);

            if code.len() == 8 {
                return code
            }
        }
    }

    unreachable!()
}

fn advanced_code(key: &str) -> String {
    let mut code = vec![None; 8];

    for i in 0.. {
        if let Some((p, c)) = is_password_letter(i, key) {
            let pos = char_to_usize(p);
            if pos > 7 || code[pos].is_some() {
                continue
            }
            code[pos] = Some(c);
            println!("code so far: {:?}", code);
        }
        if code.iter().all(Option::is_some) {
            return code.into_iter().map(Option::unwrap).collect();
        }
    }

    unreachable!()
}

fn main() {
    println!("basic code: {}", basic_code(KEY));
    println!("advanced code: {}", advanced_code(KEY));
}

#[test]
fn test_basic() {
    assert_eq!(Some('1'), is_password_letter(3231929, "abc").map(|t| t.0));
    assert_eq!(Some('8'), is_password_letter(5017308, "abc").map(|t| t.0));
    assert_eq!(Some('f'), is_password_letter(5278568, "abc").map(|t| t.0));
}

#[test]
fn test_advance() {
    assert_eq!(Some(('1', '5')), is_password_letter(3231929, "abc"));
    assert_eq!(Some(('4', 'e')), is_password_letter(5357525, "abc"));
    // runs but too slow in debug mode; only tests in release mode
    assert_eq!("05ace8e3", advanced_code("abc"));
}
