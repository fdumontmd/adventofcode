pub fn get_input_path() -> String {
    std::env::args().nth(1).unwrap_or("input.txt".to_owned())
}

pub mod union_find;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
