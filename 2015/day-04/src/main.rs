extern crate md5;
extern crate hex;

use md5::{Md5, Digest};
use hex::encode;

const KEY: &'static str = "bgvyzdsv";

fn main() {
    let mut skip5 = false;
    for n in 1.. {
        let mut hash = Md5::new();
        hash.update(KEY.as_bytes());
        hash.update(&n.to_string().as_bytes());
        let result = encode(hash.finalize());
        if !skip5 && &result[0..5] == "00000" {
            println!("{} || {} -> {:x?}", KEY, n, result);
            skip5 = true;
        }
        if &result[0..6] == "000000" {
            println!("{} || {} -> {}", KEY, n, result);
            break;
        }
    }
}
