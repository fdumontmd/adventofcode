pub mod custom_error;

pub mod part1;
pub mod part2;

#[tracing::instrument]
fn hash(input: &str) -> usize {
    input.bytes().fold(0, |a, b| ((a + b as usize) * 17) % 256)
}
