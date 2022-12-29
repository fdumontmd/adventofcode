use std::collections::BTreeSet;
use std::iter::FromIterator;

const ELFS: u64 = 3012210;

// actually, that problem is the binary Josephus problem from Concrete Mathematics
fn josephus(n: u64) -> u64 {
    let l = n - n.next_power_of_two() / 2;
    2 * l + 1
}

fn across_slow(n: u64) -> u64 {
    let mut elves = BTreeSet::from_iter(1..n + 1);

    let mut current = 0;
    loop {
        if elves.len() == 1 {
            return *elves.iter().next().unwrap();
        }

        let across = (elves.len() / 2 + current) % elves.len();

        let elt = *elves.iter().nth(across).unwrap();
        elves.remove(&elt);

        if across > current {
            current = (current + 1) % elves.len();
        }
        if elves.len() % 100 == 0 {
            println!("remaining elves: {}", elves.len());
        }
    }
}

// design: only handle the simple rule for the first half of the remaining elves
// with
// L = number of elves in the current iteration
// R = number of removed elves this round
// P = position of currently considered elf
// the elf across is at (L - R) / 2 + P + R
// because if L was the number of remaining elves, the across would be L / 2 + P
// but L is the number of elves, so we need to substract those that were removed (i.e. L -R)
// also, those that were removed are between P and across (because there were across elves with position
// less than P), so we add R to compensate.
// We break when we are about to go beyond the end of the vector, to avoid handling weird corner cases
fn across(n: u64) -> u64 {
    let mut elves = Vec::from_iter(1..n + 1);

    loop {
        if elves.len() == 1 {
            return elves[0];
        }

        let mut removed = 0;

        let len = elves.len();
        let mut breakoff = len / 2;

        for pos in 0..len / 2 {
            let across = (len - removed) / 2 + pos + removed;
            if across >= len {
                breakoff = pos;
                break;
            }
            elves[across] = 0;
            removed += 1;
        }
        let mut new_elves = Vec::new();
        for elt in &elves.as_slice()[breakoff..] {
            if *elt != 0 {
                new_elves.push(*elt);
            }
        }
        new_elves.extend_from_slice(&elves.as_slice()[..breakoff]);
        elves = new_elves;
    }
}

fn main() {
    println!("Remaining elf: {}", josephus(ELFS));
    println!("Remaining elf with new rule: {}", across(ELFS));
}

#[test]
fn test() {
    assert_eq!(3, josephus(5));
    assert_eq!(1830117, josephus(3012210));
    assert_eq!(2, across(5));
    assert_eq!(1, across(10));
    assert_eq!(1417887, across(3012210));
}
