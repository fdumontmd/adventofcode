extern crate regex;
extern crate rustc_serialize;

use std::io::{self, Read};
use std::str::FromStr;

use regex::Regex;
use rustc_serialize::json::{Array, Object, Json};

fn compute_sum_array(a: &Array) -> i64 {
    a.iter().map(|j| compute_sum(j)).sum()
}

fn compute_sum_object(o: &Object) -> i64 {
    let mut sum = 0;
    for (_, v) in o {
        sum += match v {
            &Json::String(ref s) => {
                if s == "red" {
                    return 0;
                }
                0
            }
            _ => compute_sum(&v)
        }
    }
    sum
}

fn compute_sum(data: &Json) -> i64 {
    match *data {
        Json::I64(i) => i,
        Json::U64(u) => u as i64,
        Json::F64(_) => unreachable!(),
        Json::Boolean(_) => 0,
        Json::String(_) => 0,
        Json::Array(ref a) => compute_sum_array(a),
        Json::Object(ref o) => compute_sum_object(o),
        Json::Null => 0,
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut sum = 0;
    for caps in re.captures_iter(&buffer) {
        sum += i64::from_str(caps.at(1).unwrap()).unwrap();
    }

    println!("total: {}", sum);

    let data = Json::from_str(&buffer).unwrap();
    sum = compute_sum(&data);

    println!("total (not red): {}", sum);
}
