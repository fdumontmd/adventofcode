use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;

use faer::{prelude::*, Mat, Side};

// Graph partition with Fiedler eigenvalue and eigenvector:
// https://en.wikipedia.org/wiki/Graph_partition#Fiedler_eigenvalue_and_eigenvector

#[tracing::instrument]
pub fn process<'a>(input: &'a str) -> Result<String, AocError> {
    let mut graph: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(':');
        let comp = parts.next().unwrap().trim();
        let connected = parts.next().unwrap().split_whitespace().collect::<Vec<_>>();

        for conn in connected {
            graph.entry(comp).or_default().insert(conn);
            graph.entry(conn).or_default().insert(comp);
        }
    }

    let names = graph.keys().collect::<Vec<_>>();
    let names_rev: HashMap<&'a str, usize> =
        HashMap::from_iter(names.iter().enumerate().map(|(idx, n)| (**n, idx)));
    let mut a: Mat<f64> = Mat::zeros(names.len(), names.len());
    let d = Mat::from_fn(names.len(), names.len(), |i, j| {
        if i == j {
            graph[names[i]].len() as f64
        } else {
            0f64
        }
    });

    for (idx, name) in names.iter().enumerate() {
        for conn in &graph[*name] {
            let oidx = names_rev[conn];
            a[(idx, oidx)] = 1f64;
        }
    }

    let l = d - a;
    let v = l.selfadjoint_eigendecomposition(Side::Upper);

    let mut part_1 = 0;
    let mut part_2 = 0;

    for i in 0..v.u().nrows() {
        if v.u().read(i, 1) < 0f64 {
            part_1 += 1;
        } else {
            part_2 += 1;
        }
    }
    Ok(format!("{}", part_1 * part_2))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[rstest]
    #[case(INPUT, "54")]
    fn test_process(#[case] input: &str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
