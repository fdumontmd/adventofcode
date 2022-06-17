extern crate md5;
extern crate hex;

use std::collections::vec_deque::VecDeque;

use md5::{Md5, Digest};
use hex::encode;

const WINDOW: usize = 1000;
const BUFFER_LEN: usize = WINDOW + 1;

struct Iter<'s> {
    hash: fn(&'s str, u64) -> (u64, String),
    salt: &'s str,
    next_index: u64,
    buffer: VecDeque<(u64, String)>,
}

fn hash(salt: &str, index: u64) -> (u64, String) {
    let mut md5 = Md5::new();
    let index_str = format!("{}", index);
    md5.update(salt.as_bytes());
    md5.update(&index_str.as_bytes());
    (index, encode(md5.finalize()))
}

impl<'s> Iter<'s> {
    fn new(salt: &'s str, h: fn(&'s str, u64) -> (u64, String)) -> Self {
        let mut buffer = VecDeque::with_capacity(BUFFER_LEN);
        buffer.push_back(h(salt, 0));

        let mut this = Iter {
            hash: h,
            salt: salt,
            next_index: 1,
            buffer: buffer,
        };
        this.build();
        this
    }

    fn build(&mut self) {
        let mut index = self.next_index;
        while self.buffer.len() < BUFFER_LEN {
            self.buffer.push_back((self.hash)(self.salt, index));
            index += 1;
        }
        self.next_index = index;
    }

    fn is_key(&self) -> bool {
        let next_index = self.buffer.front().unwrap();
        let mut repeat = None;
        for chunks in next_index.1.as_bytes().windows(3) {
            if chunks[0] == chunks[1] && chunks[1] == chunks[2] {
                repeat = Some(chunks[0]);
                break;
            }
        }
        if let Some(repeat) = repeat {
            for key in &self.buffer {
                if key.0 == next_index.0 {
                    continue;
                }
                for chunks in key.1.as_bytes().windows(5) {
                    if chunks.iter().all(|b| *b == repeat) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn inner_next(&mut self) -> (u64, String) {
        let next_index = self.buffer.pop_front().unwrap();
        self.build();
        next_index
    }
}

impl<'s> Iterator for Iter<'s> {
    type Item = (u64, String);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.is_key() {
                return Some(self.inner_next());
            }
            self.inner_next();
        }
    }
}

fn hash2016(salt: &str, index: u64) -> (u64, String) {
    let mut md5 = Md5::new();
    let index_str = format!("{}", index);
    md5.update(salt.as_bytes());
    md5.update(&index_str.as_bytes());
    let mut str = encode(md5.finalize_reset());
    for _ in 0..2016 {
        md5.update(&str.as_bytes());
        str = encode(md5.finalize_reset());
    }
    (index, str)
}

fn main() {
    let salt = "ahsbgdzn";
    let mut iter = Iter::new(salt, hash);
    println!("{:?}", iter.nth(63));
    let mut iter = Iter::new(salt, hash2016);
    println!("{:?}", iter.nth(63));
}

#[test]
fn test() {
    let salt = "abc";
    let mut iter = Iter::new(salt, hash);
    assert_eq!(iter.nth(63).unwrap().0, 22728);
}

#[test]
fn test2016() {
    let salt = "abc";
    let mut iter = Iter::new(salt, hash2016);
    assert_eq!(iter.nth(63).unwrap().0, 22551);
}
