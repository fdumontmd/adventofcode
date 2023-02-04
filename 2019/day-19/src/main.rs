use std::collections::{BTreeSet, HashSet};

use intcode::Computer;

static INPUT: &str = include_str!("input.txt");

fn part_01(input: &str, dim: usize) -> usize {
    (0..(dim * dim))
        .filter(|idx| {
            let mut computer: Computer = input.parse().unwrap();
            let x = idx % dim;
            let y = idx / dim;

            computer.add_input(x as i64);
            computer.add_input(y as i64);
            let Some(o) = computer.wait_until_output() else { panic!("computer not responding")};
            o == 1
        })
        .count()
}

// ideally we'd want to computer the slope of both sides of the beam
// but tests show the beam is so flat there might not be any slope
// for a while
fn part_02(input: &str) -> i64 {
    let x = 20000;
    let mut min = 0;
    let mut max = usize::MAX;
    for y in 0.. {
        let mut computer: Computer = input.parse().unwrap();
        computer.add_input(x);
        computer.add_input(y);
        let Some(o) = computer.wait_until_output() else { panic!("computer not responding")};
        if min == 0 && o == 1 {
            min = y as usize;
        } else if min > 0 && o == 0 {
            max = y as usize - 1;
            break;
        }
    }
    println!("at x = {x}, y = [{min}, {max}]");

    let s1 = (min as f64) / (x as f64);
    let s2 = (max as f64) / (x as f64);

    let a = (100.0 * s1 + 100.0) / (s2 - s1);
    let b = s1 * (a + 100.0);

    dbg!(a, b);

    let max_x = a.ceil() as i64;

    // we are close, but need a bit of search to find the
    // minimum coord.
    // for my input, max_x - 20 works; could start from 0
    // otherwise
    for x in max_x - 20..=max_x {
        let y_min = (x as f64 * s1).floor() as i64;
        let y_max = (x as f64 * s2).ceil() as i64;

        'search: for y in y_min..=y_max {
            for d in [(0, 0), (99, 0), (0, 99), (99, 99)] {
                let mut computer: Computer = input.parse().unwrap();
                computer.add_input(x + d.0);
                computer.add_input(y + d.1);
                let Some(o) = computer.wait_until_output() else { panic!() };
                if o != 1 {
                    continue 'search;
                }
            }
            dbg!((x, y));
            return x * 10000 + y;
        }
    }
    panic!("not found")
}

fn main() {
    println!("Part 1, {}", part_01(INPUT, 50));
    println!("Part 1, {}", part_02(INPUT));
}
