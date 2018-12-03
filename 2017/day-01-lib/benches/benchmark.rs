#[macro_use]
extern crate criterion;
extern crate day_01_lib;

use criterion::{Bencher, Criterion, Fun};
use day_01_lib::*;

use std::fmt::{Display, Formatter, Error};

fn read_data() -> Vec<u64> {
    use std::fs::File;
    use std::io::Read;
    let input = File::open("input.txt").unwrap();
    let mut data = Vec::new();

    for b in input.bytes() {
        let b = b.unwrap();
        if '0' as u8 <= b && b <= '9' as u8 {
            data.push((b - '0' as u8) as u64);
        }
    } 
    data
}

#[derive(Debug)]
struct Data {
    offset: usize,
    data: Vec<u64>,
}

impl Data {
    fn new(offset: usize, data: Vec<u64>) -> Self {
        Data {
            offset,
            data,
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Data({}, {:?})", self.offset, self.data)
    }
}

fn benchmark(c: &mut Criterion) {
    {
        let data: Vec<u64> = read_data();
        let orig = Fun::new("Original", |b: &mut Bencher, d: &Data| b.iter(|| compute_code(&d.data, d.offset)));
        let iter = Fun::new("Iterator", |b: &mut Bencher, d: &Data| b.iter(|| compute_code_iter(&d.data, d.offset)));

        let functions = vec!(orig, iter);
        c.bench_functions("Offset 1", functions, Data::new(1, data));
    }
    {
        let data: Vec<u64> = read_data();
        let orig = Fun::new("Original", |b: &mut Bencher, d: &Data| b.iter(|| compute_code(&d.data, d.offset)));
        let iter = Fun::new("Iterator", |b: &mut Bencher, d: &Data| b.iter(|| compute_code_iter(&d.data, d.offset)));

        let functions = vec!(orig, iter);
        c.bench_functions("Offset half", functions, Data::new(data.len() / 2, data));
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
