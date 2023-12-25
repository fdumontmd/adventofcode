use day_01::*;

#[tracing::instrument(level = "trace", skip())]
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[tracing::instrument(level = "trace", skip())]
#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}

#[tracing::instrument(level = "trace", skip())]
#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}
