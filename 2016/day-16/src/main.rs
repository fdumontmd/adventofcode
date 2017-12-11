const INPUT: &'static str = "11011110011011101";

const ZERO: u8 = 48;
const ONE: u8 = 49;

fn checksum(input: &Vec<u8>) -> String {
    let mut result = Vec::new();
    assert!(input.len() % 2 == 0, "Input length is not even");
    for pair in input.as_slice().chunks(2) {
        result.push(
            if pair[0] == pair[1] {
                ONE
            } else {
                ZERO
            }
        )
    }

    if result.len() % 2 == 0 {
        checksum(&result)
    } else {
        String::from_utf8(result).unwrap()
    }
}

struct Checksum {
    state: Vec<u8>,
    max_len: usize,
}

impl Checksum {
    fn new(input: &str, max_len: usize) -> Self {
        Checksum {
            state: Vec::from(input),
            max_len: max_len,
        }
    }

    fn len(&self) -> usize {
        self.state.len()
    }

    fn extend(&mut self) {
        let mut b: Vec<u8> = self.state.iter()
            .map(|b| match b {
                &ZERO => ONE,
                &ONE => ZERO,
                _ => unreachable!() 
            })
            .collect();
        self.state.push(ZERO);
        b.reverse();
        self.state.append(&mut b);
    }

    fn checksum(&mut self) -> String {
        while self.len() < self.max_len {
            self.extend();
        }

        self.state.resize(self.max_len, ZERO);

        checksum(&self.state)
    }
}

fn main() {
    let mut checksum = Checksum::new(INPUT, 272);
    println!("Checksum: {}", checksum.checksum());

    let mut checksum = Checksum::new(INPUT, 35651584);
    println!("Checksum: {}", checksum.checksum());
}

#[test]
fn test_checksum() {
    assert_eq!("100", checksum(&Vec::from("110010110100")));
}

#[test]
fn test_extend_1() {
    let mut checksum = Checksum::new("1", usize::max_value());
    checksum.extend();
    assert_eq!(Vec::from("100"), checksum.state);
}

#[test]
fn test_extend_0() {
    let mut checksum = Checksum::new("0", usize::max_value());
    checksum.extend();
    assert_eq!(Vec::from("001"), checksum.state);
}

#[test]
fn test_extend_11111() {
    let mut checksum = Checksum::new("11111", usize::max_value());
    checksum.extend();
    assert_eq!(Vec::from("11111000000"), checksum.state);
}

#[test]
fn test_extend_111100001010() {
    let mut checksum = Checksum::new("111100001010", usize::max_value());
    checksum.extend();
    assert_eq!(Vec::from("1111000010100101011110000"), checksum.state);
}

#[test]
fn test_combined() {
    let mut checksum = Checksum::new("10000", 20);
    assert_eq!("01100", checksum.checksum());
}
