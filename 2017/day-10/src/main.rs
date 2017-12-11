use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::{Error, Formatter, LowerHex};

fn main() {
    assert!(args().len() > 1);
    let path = args().nth(1).unwrap();
    let input = File::open(&path).unwrap();
    let buf = BufReader::new(input);

    for line in buf.lines() {
        let line = line.unwrap();
        let v = line.split(",").map(|w| w.parse().unwrap()).collect();
        let r = Knot::knot(256, v);
        println!("result: {}", r[0] as u32 * r[1] as u32);
    }

    let input = File::open(&path).unwrap();

    let buf = BufReader::new(input);

    for line in buf.lines() {
        let line = line.unwrap();

        println!("{}", Knot::hash(&line));
    }
}

#[derive(Debug)]
struct Knot {
    v: Vec<u8>,
    cur: usize,
    skip: usize,
}

impl Knot {
    fn new(r: usize) -> Self {
        assert!(r <= 256);
        let mut v: Vec<u8> = (0..).take(r-1).collect();
        v.push((r-1) as u8);
        Knot {
            v, 
            cur: 0,
            skip: 0,
        }
    }

    fn round(&mut self, v: &Vec<u8>) {
        let r = self.v.len();

        let mut cur = self.cur;
        let mut skip = self.skip;

        for s in v {
            let s = *s as usize;
            if cur+s <= r {
                self.v[cur..cur+s].reverse();
            } else {
                let h = s / 2;
                for i in 0..h {
                    let b = (cur + i) % r;
                    let u = (cur + s - i - 1) % r;
                    self.v.swap(b, u);
                }
            }
            cur += s + skip;
            cur %= r;
            skip += 1;
        }

        self.cur = cur;
        self.skip = skip;
    }

    fn rounds(&mut self, r: u8, v: &Vec<u8>) {
        for _ in 0..r {
            self.round(v);
        }
    }

    fn knot(r: usize, v: Vec<u8>) -> Vec<u8> {
        let mut k = Knot::new(r);
        k.round(&v);
        k.v
    }

    fn hash(s: &str) -> String {
        let mut knot = Knot::new(256);
        let mut v: Vec<u8> = s.bytes().collect();
        v.extend([17, 31, 73, 47, 23].iter());

        knot.rounds(64, &v);

        format!("{:x}", knot)
    }
}

impl LowerHex for Knot {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), Error> {
        let dense = self.v.chunks(16).map(|c| c.iter().fold(0u8, |c, n| c ^ n));

        for d in dense {
            write!(f, "{:02x}", d)?;
        }
        Ok(())
    }
}


#[test]
fn test() {
    assert_eq!(Knot::knot(5, vec![3,4,1,5]), [3,4,2,1,0]);
}

#[test]
fn test_hash() {
    assert_eq!(Knot::hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(Knot::hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(Knot::hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(Knot::hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}
