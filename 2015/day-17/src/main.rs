use std::io::{self, Read};
use std::str::FromStr;

const TARGET: u64 = 150;

fn count_filling(containers: &[u64], target: u64) -> usize {
    if target == 0 {
        1
    } else if containers.is_empty() {
        0
    } else {
        if target < containers[0] {
            count_filling(&containers[1..], target)
        } else {
            count_filling(&containers[1..], target) + count_filling(&containers[1..], target - containers[0])
        }
    }
}

fn list_filling(containers: &[u64], target: u64, path: &Vec<u64>, collect: &mut Vec<Vec<u64>>) {
    if target == 0 {
        collect.push(path.clone());
    } else if !containers.is_empty() {
        if target < containers[0] {
            list_filling(&containers[1..], target, path, collect);
        } else {
            list_filling(&containers[1..], target, path, collect);
            let mut alt = path.clone();
            alt.push(containers[0]);
            list_filling(&containers[1..], target - containers[0], &alt, collect);
        }
    }
}


fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut containers = Vec::new();

    for line in buffer.lines() {
        containers.push(u64::from_str(line.trim()).unwrap());
    }

    containers.sort();

    let count = count_filling(containers.as_slice(), TARGET);
    println!("count: {}", count);

    let mut solutions = Vec::new();
    list_filling(containers.as_slice(), TARGET, &Vec::new(), &mut solutions);

    let min_len = solutions.iter().map(|s| s.len()).min().unwrap();
    let count = solutions.iter().filter(|s| s.len() == min_len).count();

    println!("Minimum len: {}, achieved by {} solutions", min_len, count);
}

#[test]
fn test() {
    let containers = vec![20, 15, 10, 5, 5];
    assert_eq!(4, count_filling(containers.as_slice(), 25));
}
