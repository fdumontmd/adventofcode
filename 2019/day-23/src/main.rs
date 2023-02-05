use std::{
    array,
    collections::{hash_map::Entry, HashMap, VecDeque},
};

use intcode::Computer;

static INPUT: &str = include_str!("input.txt");

struct Network {
    nics: [Computer; 50],
}

impl Network {
    fn from_str(input: &str) -> Self {
        let nic: Computer = input.parse().unwrap();
        let nics = array::from_fn(|i| {
            let mut nic = nic.clone();
            nic.add_input(i as i64);
            nic
        });
        Self { nics }
    }

    fn run_until_nat(&mut self, reset: Option<(i64, i64)>) -> Option<(i64, i64)> {
        let mut messages: HashMap<usize, VecDeque<i64>> = HashMap::new();

        if let Some((x, y)) = reset {
            messages.entry(0).or_default().push_front(x);
            messages.entry(0).or_default().push_front(y);
        }

        let mut idle: HashMap<usize, usize> = HashMap::new();

        loop {
            for (idx, nic) in self.nics.iter_mut().enumerate() {
                if nic.waiting_for_input() {
                    if let Entry::Occupied(mut message) = messages.entry(idx) {
                        let Some(m) = message.get_mut().pop_back() else { panic!("empty message queue")};
                        nic.add_input(m);
                        if message.get().is_empty() {
                            message.remove();
                        }
                    } else {
                        *idle.entry(idx).or_default() += 1;
                        nic.add_input(-1);
                    }
                }
                nic.step();
                if nic.has_output() {
                    idle.remove(&idx);
                    let output = nic.get_output();
                    if output.len() != 3 {
                        continue;
                    }
                    let output = nic.get_and_clear_output();
                    if output[0] == 255 {
                        return Some((output[1], output[2]));
                    } else {
                        let queue = messages.entry(output[0] as usize).or_default();
                        queue.push_front(output[1]);
                        queue.push_front(output[2]);
                    }
                }
            }

            // not clear how long we have to wait to say network is idle
            // looks like we can have nic waiting for input and sending nothing
            // for up to 125 iteration while still eventually emitting something
            // for part_1 to consume so will handle retry in the part_? fns
            if idle.len() == 50 && idle.values().all(|c| *c > 10) {
                return None;
            }
        }
    }
}

fn part_1() -> i64 {
    let mut network = Network::from_str(INPUT);
    loop {
        if let Some((_, y)) = network.run_until_nat(None) {
            return y;
        }
    }
}

fn part_2() -> i64 {
    let mut network = Network::from_str(INPUT);
    let mut delivered = (0, 0);
    let mut msg = None;
    let mut reset = None;

    loop {
        let new_msg = network.run_until_nat(reset);
        reset = None;
        match new_msg {
            Some((x, y)) => msg = Some((x, y)),
            None => {
                // not sure this is correct: if we have received
                // a message since last time we got stuck, we sent it,
                // then mark it as consumed (by setting msg to None)
                // text suggest we could send the same packet multiple
                // time when the network is idle, but the definition
                // of idle is unclear
                // anyway, the answers are correct, so ¯\_(ツ)_/¯
                if let Some(msg) = msg {
                    if delivered.1 == msg.1 {
                        return msg.1;
                    }
                    delivered = msg;
                    reset = Some(msg)
                }
                msg = None
            }
        }
    }
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}
