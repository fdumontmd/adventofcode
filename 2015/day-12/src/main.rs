extern crate regex;
extern crate serde_json;

use std::io::{self, Read};
use std::str::FromStr;

use regex::Regex;
use serde_json::{Map, Value, from_str};

fn compute_sum_array(a: &Vec<Value>) -> i64 {
    a.iter().map(|j| compute_sum(j)).sum()
}

fn compute_sum_object(o: &Map<String, Value>) -> i64 {
    let mut sum = 0;
    for (_, v) in o {
        sum += match v {
            Value::String(s) => {
                if s == "red" {
                    return 0;
                }
                0
            }
            _ => compute_sum(v)
        }
    }
    sum
}

fn compute_sum(data: &Value) -> i64 {
    match data {
        Value::Number(i) => i.as_i64().unwrap_or_else(|| i.as_u64().unwrap() as i64),
        Value::Bool(_) => 0,
        Value::String(_) => 0,
        Value::Array(ref a) => compute_sum_array(a),
        Value::Object(ref o) => compute_sum_object(o),
        Value::Null => 0,
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
        sum += i64::from_str(caps.get(1).unwrap().as_str()).unwrap();
    }

    println!("total: {}", sum);

    let data = from_str(&buffer).unwrap();
    sum = compute_sum(&data);

    println!("total (not red): {}", sum);
}
