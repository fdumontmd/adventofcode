use crate::{
    custom_error::AocError,
    game::{self},
};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let games = game::parse(input)?;
    let mut id_sum = 0;
    'game_loop: for game in games {
        for subset in game.subsets {
            if subset.red > 12 || subset.green > 13 || subset.blue > 14 {
                continue 'game_loop;
            }
        }
        id_sum += game.id
    }
    Ok(format!("{}", id_sum))
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
    #[case(INPUT, "8")]
    #[case(include_str!("../input.txt"), "2528")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
