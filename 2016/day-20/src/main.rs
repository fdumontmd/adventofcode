use std::io::{self, Read};
use std::str::FromStr;

type Intervals = Vec<(u32, u32)>;

fn minimum(intervals: &Intervals) -> u32 {
    let mut minimum = u32::min_value();

    for &(l, h) in intervals {
        if l <= minimum {
            minimum = h + 1;
        } else {
            break;
        }
    }

    minimum
}

fn count(intervals: &Intervals) -> u32 {
    let mut minimum = u32::min_value();
    let mut count = 0;

    for &(l, h) in intervals {
        if l <= minimum {
            if minimum < h + 1 {
                if h == u32::max_value() {
                    return count;
                }
                minimum = h + 1;
            }
        } else {
            count += l - minimum;
            if h == u32::max_value() {
                return count;
            }
            minimum = h+1;
        }
    }

    count += u32::max_value() - minimum + 1;

    count
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut intervals = Vec::new();

    for line in buffer.lines() {
        let mut iter = line.split('-');
        let low = u32::from_str(iter.next().unwrap()).unwrap();
        let high = u32::from_str(iter.next().unwrap()).unwrap();

        intervals.push((low, high));
    }

    intervals.sort();

    println!("Minimum available IP: {}", minimum(&intervals));
    println!("Available IP count: {}", count(&intervals));
}


#[test]
fn test() {
    let mut intervals = Vec::new();
    intervals.push((5, 8));
    intervals.push((0, 2));
    intervals.push((4, 7));

    intervals.sort();

    assert_eq!(3, minimum(&intervals));
}
