struct Disc {
    id: usize,
    position: usize,
    positions: usize,
}

impl Disc {
    fn new(id: usize, positions: usize, position: usize) -> Self {
        Disc {
            id: id,
            positions: positions,
            position: position,
        }
    }

    fn pass_through(&self, time: usize) -> bool {
        (time + self.id + self.position) % self.positions == 0
    }
}

struct Discs {
    discs: Vec<Disc>,
}

impl Discs {
    fn new() -> Self {
        Discs {
            discs: Vec::new(),
        }
    }

    fn push(&mut self, positions: usize, position: usize) -> &mut Self {
        let id = self.discs.len() + 1;
        self.discs.push(Disc::new(id, positions, position));
        self
    }

    fn pass_through(&self, time: usize) -> bool {
        self.discs.iter().all(|d| d.pass_through(time))
    }

    fn first_pass_time(&self) -> usize {
        for time in 0..usize::max_value() {
            if self.pass_through(time) {
                return time;
            }
        }
        unreachable!()
    }
}

fn main() {
    let mut discs = Discs::new();
    discs
        .push(7, 0)
        .push(13, 0)
        .push(3, 2)
        .push(5, 2)
        .push(17, 0)
        .push(19, 7);


    println!("Time: {}", discs.first_pass_time());

    discs.push(11, 0);

    println!("Extended discs time: {}", discs.first_pass_time());
}

#[test]
fn test() {
    let mut discs = Discs::new();
    discs.push(5, 4).push(2, 1);

    assert_eq!(5, discs.first_pass_time());
}
