fn point_to_seq(row: usize, column: usize) -> usize {
    let start_row = row + column - 2;
    let base = start_row * (start_row + 1) / 2;
    base + column
}

fn next_code(i: u64) -> u64 {
    (i * 252533u64) % 33554393u64
}

fn main() {
    let pos = point_to_seq(2947, 3029);
    let mut code = 20151125u64;
    for _ in 1..pos {
        code = next_code(code);
    }

    println!("code: {}", code);
}

#[cfg(test)]
mod tests {
    use super::point_to_seq;

    #[test]
    fn point_to_seq_check() {
        assert_eq!(point_to_seq(1,1), 1);
        assert_eq!(point_to_seq(2,1), 2);
        assert_eq!(point_to_seq(4,2), 12);
        assert_eq!(point_to_seq(1,5), 15);
    }
}
