use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

// approach:
// map all ids to numbers 0..
// build adjacent lists for both directions
// then for each c1, for each c2 in adjacency_map with c2 > c1, check the intersection between
// their adjacency map
fn part1(input: &str) -> usize {
    let mut id_map: HashMap<&str, usize> = HashMap::new();
    let mut name_map: Vec<&str> = vec![];

    let mut adjacency_map: Vec<HashSet<usize>> = vec![];
    let mut t_computers = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let c1 = parts[0];
        let c2 = parts[1];

        let c1_idx = *id_map.entry(c1).or_insert(adjacency_map.len());

        if c1_idx >= adjacency_map.len() {
            adjacency_map.push(HashSet::new());
            name_map.push(c1);
            if c1.starts_with('t') {
                t_computers.insert(c1_idx);
            }
        }

        let c2_idx = *id_map.entry(c2).or_insert(adjacency_map.len());

        if c2_idx >= adjacency_map.len() {
            adjacency_map.push(HashSet::new());
            name_map.push(c2);
            if c2.starts_with('t') {
                t_computers.insert(c2_idx);
            }
        }

        adjacency_map[c1_idx].insert(c2_idx);
        adjacency_map[c2_idx].insert(c1_idx);
    }

    let mut triples = HashSet::new();

    for (c1, adj) in adjacency_map.iter().enumerate() {
        for &c2 in adj {
            if c2 > c1 {
                for &shared in adj.intersection(&adjacency_map[c2]) {
                    if shared != c1 && shared != c2 {
                        // already c1 < c2
                        let triple = if shared < c1 {
                            (shared, c1, c2)
                        } else if shared < c2 {
                            (c1, shared, c2)
                        } else {
                            (c1, c2, shared)
                        };
                        if t_computers.contains(&c1)
                            || t_computers.contains(&c2)
                            || t_computers.contains(&shared)
                        {
                            triples.insert(triple);
                        }
                    }
                }
            }
        }
    }

    triples.len()
}

// that's a maximal clique problem...
fn part2(input: &str) -> String {
    let mut id_map: HashMap<&str, usize> = HashMap::new();
    let mut name_map: Vec<&str> = vec![];

    let mut adjacency_lists: Vec<HashSet<usize>> = vec![];
    let mut t_computers = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let c1 = parts[0];
        let c2 = parts[1];

        let c1_idx = *id_map.entry(c1).or_insert(adjacency_lists.len());

        if c1_idx >= adjacency_lists.len() {
            adjacency_lists.push(HashSet::new());
            name_map.push(c1);
            if c1.starts_with('t') {
                t_computers.insert(c1_idx);
            }
        }

        let c2_idx = *id_map.entry(c2).or_insert(adjacency_lists.len());

        if c2_idx >= adjacency_lists.len() {
            adjacency_lists.push(HashSet::new());
            name_map.push(c2);
            if c2.starts_with('t') {
                t_computers.insert(c2_idx);
            }
        }

        adjacency_lists[c1_idx].insert(c2_idx);
        adjacency_lists[c2_idx].insert(c1_idx);
    }

    let mut cliques = HashSet::new();

    for (c1, adj) in adjacency_lists.iter().enumerate() {
        for &c2 in adj {
            if c2 > c1 {
                for &shared in adj.intersection(&adjacency_lists[c2]) {
                    if shared != c1 && shared != c2 {
                        // already c1 < c2
                        let mut triple = vec![c1, c2, shared];
                        triple.sort();
                        cliques.insert(triple);
                    }
                }
            }
        }
    }

    loop {
        let mut max_len = 0;
        let mut new_cliques = HashSet::new();
        for clique in cliques {
            let shared: HashSet<usize> = clique
                .iter()
                .fold(adjacency_lists[clique[0]].clone(), |s, e| {
                    s.intersection(&adjacency_lists[*e]).copied().collect()
                });
            for candidate in shared {
                let mut new_clique = clique.clone();
                new_clique.push(candidate);
                new_clique.sort();
                max_len = max_len.max(new_clique.len());
                new_cliques.insert(new_clique);
            }
            new_cliques.insert(clique);
        }
        cliques = new_cliques;
        if max_len == 0 {
            break;
        }
        cliques.retain(|c| c.len() == max_len);
    }

    if let Some(clique) = cliques.into_iter().next() {
        let mut names: Vec<&str> = clique.into_iter().map(|c| name_map[c]).collect();
        names.sort();
        names.join(",")
    } else {
        panic!("no maximal clique found");
    }
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test_case(TEST_INPUT, 7; "test input")]
    #[test_case(INPUT, 1304; "input")]
    fn test_part1(input: &str, candidates: usize) {
        assert_eq!(candidates, part1(input));
    }

    #[test_case(TEST_INPUT, "co,de,ka,ta"; "test input")]
    #[test_case(INPUT, "ao,es,fe,if,in,io,ky,qq,rd,rn,rv,vc,vl"; "slow")]
    fn test_part2(input: &str, password: &str) {
        assert_eq!(password, part2(input));
    }
}
