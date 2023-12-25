use day_06::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}

mod part2 {
    use day_06::*;
    #[divan::bench]
    fn quadratic_equation() {
        part2::process(divan::black_box(include_str!("../input.txt",))).unwrap();
    }

    #[divan::bench]
    fn part2_brute_force() {
        part2::process_brute_force(divan::black_box(include_str!("../input.txt",))).unwrap();
    }
}
