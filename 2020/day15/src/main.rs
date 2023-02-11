use std::collections::HashMap;

static INPUT: &[u64] = &[14, 8, 16, 0, 1, 17];

fn sequence(start: &[u64]) -> impl Iterator<Item = u64> + '_ {
    let mut last_pos = HashMap::new();
    for (idx, v) in start.iter().cloned().enumerate() {
        last_pos.insert(v, idx);
    }
    last_pos.remove(&start[start.len() - 1]);

    struct State {
        last_pos: HashMap<u64, usize>,
        last_val: u64,
        cur_pos: usize,
    }

    impl Iterator for State {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let cur_val = if let Some(prev_pos) = self.last_pos.get(&self.last_val) {
                (self.cur_pos - prev_pos) as u64 - 1
            } else {
                0
            };
            self.last_pos.insert(self.last_val, self.cur_pos - 1);
            self.last_val = cur_val;
            self.cur_pos += 1;
            Some(cur_val)
        }
    }

    let state = State {
        last_pos,
        last_val: start[start.len() - 1],
        cur_pos: start.len(),
    };

    start.iter().cloned().chain(state)
}

fn part_1(input: &[u64]) -> u64 {
    sequence(input).nth(2019).unwrap()
}

// stupid, but fast enough in release mode...
// should actually look for repeated patterns
fn part_2(input: &[u64]) -> u64 {
    sequence(input).nth(30000000 - 1).unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{part_1, part_2};
    #[test_case(&[0,3,6], 436)]
    #[test_case(&[1,3,2], 1)]
    #[test_case(&[2,1,3], 10)]
    #[test_case(&[1,2,3], 27)]
    #[test_case(&[2,3,1], 78)]
    #[test_case(&[3,2,1], 438)]
    #[test_case(&[3,1,2], 1836)]
    fn test_part_1(input: &[u64], res: u64) {
        assert_eq!(res, part_1(input));
    }

    #[test_case(&[0,3,6], 175594)]
    #[test_case(&[1,3,2], 2578)]
    #[test_case(&[2,1,3], 3544142)]
    #[test_case(&[1,2,3], 261214)]
    #[test_case(&[2,3,1], 6895259)]
    #[test_case(&[3,2,1], 18)]
    #[test_case(&[3,1,2], 362)]
    fn test_part_2(input: &[u64], res: u64) {
        assert_eq!(res, part_2(input))
    }
}
