const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*:

    const TEST_INPUT = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;

    fn check_1() {
        assert!(part1(&parse(TEST_INPUT)), 37);
    }
}
