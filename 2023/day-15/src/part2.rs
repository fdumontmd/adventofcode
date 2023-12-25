use crate::{custom_error::AocError, hash};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AocError> {
    let input = input.trim();

    let mut boxes: [Vec<(&str, u8)>; 256] = std::array::from_fn(|_| Vec::new());

    let mut sum = 0;

    for op in input.split(',').map(Op::from_str) {
        match op {
            Op::Remove(lbl) => {
                let b = hash(lbl);
                if let Some(idx) = boxes[b].iter().position(|(l, _)| l == &lbl) {
                    boxes[b].remove(idx);
                }
            }
            Op::Add(lbl, f) => {
                let b = hash(lbl);
                if let Some(idx) = boxes[b].iter().position(|(l, _)| l == &lbl) {
                    boxes[b][idx].1 = f;
                } else {
                    boxes[b].push((lbl, f));
                }
            }
        }
    }

    for (idx, b) in boxes.into_iter().enumerate() {
        for (lidx, l) in b.into_iter().enumerate() {
            sum += (idx + 1) * (lidx + 1) * (l.1 as usize);
        }
    }

    Ok(format!("{sum}"))
}

enum Op<'a> {
    Add(&'a str, u8),
    Remove(&'a str),
}

impl<'a> Op<'a> {
    fn from_str(input: &'a str) -> Self {
        if let Some(rem) = input.strip_suffix('-') {
            Op::Remove(rem)
        } else {
            let parts: Vec<&'a str> = input.split('=').collect();
            Op::Add(parts[0], parts[1].parse().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[rstest]
    #[case(INPUT, "145")]
    #[case(include_str!("../input.txt"), "210906")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
