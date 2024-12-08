extern crate hex;
extern crate md5;

use hex::encode;
use md5::{Digest, Md5};

const KEY: &str = "bgvyzdsv";

fn main() {
    let mut skip5 = false;
    for n in 1.. {
        let mut hash = Md5::new();
        hash.update(KEY.as_bytes());
        hash.update(n.to_string().as_bytes());
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
