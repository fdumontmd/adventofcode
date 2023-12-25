use std::collections::HashMap;

use aoc_utils::num::extended_gcd;

use crate::{custom_error::AocError, part1::parse_input};

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    // visualization is key.
    // used make_graph to visualize the graph of dependencies;
    // tg will pulse low into rx if tf, db, vq and ln all pulse high together
    // each of tf, db, vq, and ln, have a single conjunction as source:
    // vd -> tf
    // tp -> db
    // pt -> vq
    // bk -> ln
    //
    // each of these conjunctions have a state of 12 bits each, so
    // it should be possible to compute their activating cycle in
    // no more than 4096 iterations
    let mut cycles = HashMap::new();
    // technically, could work out the inputs to the gate firing into rx
    // but no, I'm done here
    for gate in ["tf", "db", "vq", "ln"] {
        let mut state = parse_input(input);
        for idx in 1usize..4097 {
            // is that even a cycle?
            if let Some(cycle) = cycles.get(gate) {
                if idx.rem_euclid(*cycle) == 0 {
                    assert!(state.press_button_with_monitor(gate));
                    eprintln!("{gate} changed state at {idx}, again, at a multiple of {cycle}");
                    break;
                }
            } else if state.press_button_with_monitor(gate) {
                //eprintln!("{gate} pulsed high at {idx}");
                cycles.insert(gate, idx);
                // comment this to test the cycle
                break;
            }
        }
    }

    // ok, all values are odd, so normally they should loop even if the modules
    // still fires a little bit afterwards

    let mut cycle = *cycles.values().next().unwrap();
    for v in cycles.values() {
        cycle = (cycle * v) / extended_gcd(cycle as i64, *v as i64).0 as usize;
    }

    Ok(format!("{cycle}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(include_str!("../input.txt"), "252667369442479")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
