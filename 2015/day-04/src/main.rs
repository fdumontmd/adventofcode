extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

const KEY: &'static str = "bgvyzdsv";

fn main() {
    for n in 1.. {
        let mut hash = Md5::new();
        hash.input_str(KEY);
        hash.input_str(&n.to_string());
        if hash.result_str().starts_with("000000") {
            println!("{}{} -> {}", KEY, n, hash.result_str());
            break;
        }
    }
}
