use crate::{
    custom_error::AocError,
    game::{self},
};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let games = game::parse(input)?;
    let power: u32 = games
        .into_iter()
        .map(|game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for subset in game.subsets {
                red = red.max(subset.red);
                green = green.max(subset.green);
                blue = blue.max(subset.blue);
            }
            red * green * blue
        })
        .sum();
    Ok(format!("{}", power))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    static INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[rstest]
    #[case(INPUT, "2286")]
    #[case(include_str!("../input.txt"), "67363")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
