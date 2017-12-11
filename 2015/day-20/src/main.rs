use std::collections::BinaryHeap;

const TARGET: i64 = 29000000;

fn first_puzzle() -> i64 {
    let mut wheel: BinaryHeap<(i64, i64)> = BinaryHeap::new();

    for h in 1.. {
        let mut total = h * 10;

        while let Some(head) = wheel.pop() {
            if head.0 != -h {
                wheel.push(head);
                break;
            }

            total += head.1 * 10;
            wheel.push((head.0 - head.1, head.1));
        }
        if total >= TARGET {
            return h;
        }

        wheel.push((-2*h, h))
    }

    unreachable!()
}

fn second_puzzle() -> i64 {
    let mut wheel: BinaryHeap<(i64, i64, u8)> = BinaryHeap::new();

    for h in 1.. {
        let mut total = h * 11;

        while let Some(head) = wheel.pop() {
            if head.0 != -h {
                wheel.push(head);
                break;
            }

            total += head.1 * 11;
            if head.2 < 50 {
                wheel.push((head.0 - head.1, head.1, head.2 + 1));
            }
        }
        if total >= TARGET {
            return h;
        }

        wheel.push((-2*h, h, 2))
    }

    unreachable!()
}

fn main() {
    println!("First result: {}", first_puzzle());
    println!("Second result: {}", second_puzzle());
}
