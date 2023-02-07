use std::collections::{BinaryHeap, HashMap};

const INPUT: &str = include_str!("input");

// ok that was pretty dumb
fn part01(input: &str, days: usize) -> usize {
    let mut fish = BinaryHeap::new();
    for f in input.split(",").map(|n| n.trim().parse::<isize>().expect(&format!("cannot parse <{}>", n))) {
        fish.push(-f);
    }

    loop {
        // must use peek here in case we break 
        if let Some(day) = fish.peek() {
            let day = day.abs() as usize;
            if day >= days {
                break;
            }
            fish.pop();
            
            fish.push(-(day as isize + 7));
            fish.push(-(day as isize + 9));

            loop {
                if let Some(d) = fish.peek() {
                    if *d == -(day as isize) {
                        fish.pop();
                        
                        fish.push(-(day as isize + 7));
                        fish.push(-(day as isize + 9));
                    } else {
                        break;
                    }
                }
            }

        }
    }

    fish.len()
}

fn part02(input: &str, days: usize) -> usize {
    let mut fish: HashMap<usize, usize> = HashMap::new();
    for f in input.split(",").map(|n| n.trim().parse::<usize>().expect(&format!("cannot parse <{}>", n))) {
        *fish.entry(f).or_default() += 1;
    }

    for d in 0..days {
        if let Some(f) = fish.get(&d) {
            let f = *f;
            fish.remove(&d);
            *fish.entry(d + 7).or_default() += f;
            *fish.entry(d + 9).or_default() += f;
        }
    }

    fish.values().sum()
}

fn main() {
    println!("{}", part02(INPUT, 80));
    println!("{}", part02(INPUT, 256));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "3,4,3,1,2";

    #[test]
    fn test_part01_01() {
        assert_eq!(26, part01(TEST, 18));
    }

    #[test]
    fn test_part01_02() {
        assert_eq!(5934, part01(TEST, 80));
    }

    #[test]
    fn test_part02_01() {
        assert_eq!(26, part02(TEST, 18));
    }

    #[test]
    fn test_part02_02() {
        assert_eq!(5934, part02(TEST, 80));
    }

    #[test]
    fn test_part02_03() {
        assert_eq!(26984457539, part02(TEST, 256));
    }

    #[test]
    fn tst_part01_input() {
        assert_eq!(354564, part02(INPUT, 80));
    }

    #[test]
    fn tst_part02_input() {
        assert_eq!(1609058859115, part02(INPUT, 256));
    }
}
