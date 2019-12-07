pub use std::fs::File;
pub use std::io::{BufRead, BufReader};

// should use Path and AsRef(Path) for everything
pub fn get_input_path() -> String {
    std::env::args().nth(1).unwrap_or("input.txt".to_owned())
}

pub fn get_input() -> BufReader<File> {
    let path = get_input_path();
    let input = File::open(path).expect("Cannot open input file");
    BufReader::new(input)
}


pub mod union_find;
pub mod ring;
pub mod permutations;
