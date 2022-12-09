static INPUT: &[u8] = include_bytes!("input.txt");

struct State {
    bytes: [u8; 256],
    unique: usize,
}

impl State {
    fn new() -> Self {
        State {
            bytes: [0; 256],
            unique: 0,
        }
    }

    fn unique_count(&self) -> usize {
        self.unique
    }

    fn push(&mut self, byte: u8) {
        match self.bytes[byte as usize] {
            0 => self.unique += 1,
            1 => self.unique -= 1,
            _ => (),
        };
        self.bytes[byte as usize] += 1;
    }

    fn pop(&mut self, byte: u8) {
        match self.bytes[byte as usize] {
            2 => self.unique += 1,
            1 => self.unique -= 1,
            _ => (),
        };
        self.bytes[byte as usize] -= 1;
    }
}

fn start_of_sequence_alt(len: usize, input: &[u8]) -> usize {
    let mut state = State::new();
    for byte in &input[..len] {
        state.push(*byte);
    }

    if state.unique_count() == len {
        return 0;
    }

    for (index, window) in input.windows(len + 1).enumerate() {
        state.pop(window[0]);
        state.push(window[len]);

        if state.unique_count() == len {
            return index + 1 + len;
        }
    }

    panic!("not found");
}

fn start_of_sequence(len: usize, input: &[u8]) -> usize {
    input
        .windows(len)
        .enumerate()
        .find(|(_, bytes)| {
            bytes
                .iter()
                .cloned()
                .collect::<std::collections::HashSet<_>>()
                .len()
                == len
        })
        .map(|(idx, _)| idx + len)
        .unwrap()
}

fn start_of_packet(input: &[u8]) -> usize {
    start_of_sequence(4, input)
}

fn start_of_message(input: &[u8]) -> usize {
    start_of_sequence(14, input)
}

fn main() {
    println!("Part 1: start of packet: {}", start_of_packet(INPUT));
    println!("Part 2: start of message: {}", start_of_message(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(7, b"mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, b"bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, b"nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    #[test_case(1855, INPUT)]
    fn test_part_01(index: usize, input: &[u8]) {
        assert_eq!(index, start_of_packet(input));
    }

    #[test_case(19, b"mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, b"bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, b"nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    #[test_case(3256, INPUT)]
    fn test_part_02(index: usize, input: &[u8]) {
        assert_eq!(index, start_of_message(input));
    }

    #[test_case(7, b"mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, b"bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, b"nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    #[test_case(1855, INPUT)]
    fn test_alt_01(index: usize, input: &[u8]) {
        assert_eq!(index, start_of_sequence_alt(4, input));
    }

    #[test_case(19, b"mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, b"bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, b"nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    #[test_case(3256, INPUT)]
    fn test_alt_02(index: usize, input: &[u8]) {
        assert_eq!(index, start_of_sequence_alt(14, input));
    }
}
