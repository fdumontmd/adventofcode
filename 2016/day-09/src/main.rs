use std::io::{self, Read};
use std::fmt::{Display, Formatter, Result, Write};
use std::str::FromStr;

struct Decompress<'a>(&'a str);

impl<'a> Decompress<'a> {
    fn new(buffer: &'a str) -> Self {
        Decompress(buffer)
    }
}

impl<'a> Display for Decompress<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut iter = self.0.chars().peekable();

        loop {
            if let Some(&'(') = iter.peek() {
                iter.next().unwrap();
                let params: String = iter.by_ref().take_while(|c| *c != ')').collect();
                let mut param_iter = params.split("x");
                let len = usize::from_str(param_iter.next().unwrap()).unwrap();
                let rep = usize::from_str(param_iter.next().unwrap()).unwrap();
                let mut segment = String::new();
                for _ in 0..len {
                    segment.push(iter.next().unwrap());
                }
                for _ in 0..rep {
                    write!(f, "{}", segment)?;
                }
            } else if let Some(c) = iter.next() {
                write!(f, "{}", c)?;
            } else {
                break;
            }
        } 

        Ok(())
    }
}

fn decompressed_size(s: &[u8]) -> u64 {
    let mut iter = s.iter();
    let mut total_len = 0;
    loop {
        // 40 is ascii code of ( 
        // 41 is ascii code of )
        if let Some(c) = iter.next() {
            if *c == 40 {
                let params = String::from_utf8(iter.by_ref().take_while(|c| **c != 41).cloned().collect()).unwrap();
                let mut param_iter = params.split("x");
                let len = usize::from_str(param_iter.next().unwrap()).unwrap();
                let rep = u64::from_str(param_iter.next().unwrap()).unwrap();
                let segment_len = decompressed_size(&iter.as_slice()[..len]);
                total_len += segment_len * rep;
                for _ in 0..len {
                    iter.next();
                }
            } else {
                total_len += 1;
            }
        } else {
            break;
        }
    }

    total_len
}

fn decompress(s: &str) -> String {
    let d = Decompress::new(s);
    let mut output = String::new();
    write!(output, "{}", d).unwrap();
    output
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let output = decompress(&buffer.trim());

    println!("Decompressed size: {}", output.len());
    println!("Recursively decompressed size: {}", decompressed_size(buffer.trim().as_bytes()));
}

#[test]
fn check() {
    assert_eq!("ADVENT", decompress("ADVENT"));
    assert_eq!("ABBBBBC", decompress("A(1x5)BC"));
    assert_eq!("XYZXYZXYZ", decompress("(3x3)XYZ"));
    assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG"));
    assert_eq!("(1x3)A", decompress("(6x1)(1x3)A"));
    assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY"));
}
