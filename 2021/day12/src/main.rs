use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("input.txt");

const START: &str = "start";
const END: &str = "end";

fn is_small(cave_name: &str) -> bool {
    cave_name.bytes().all(|b| b.is_ascii_lowercase())
}

// fn build_graph<'a>(input: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
fn build_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();

    input.lines().for_each(|l| {
        let ends: Vec<_> = l.split('-').collect();
        map.entry(ends[0]).or_insert(vec![]).push(ends[1]);
        map.entry(ends[1]).or_insert(vec![]).push(ends[0]);
    });

    map
}

fn part_1(input: &str) -> usize {
    let graph = build_graph(input);

    let mut count = 0;

    // DFS through the graph:
    // stack contains state: current cave, visited caves
    // when pop: if current cave == END, increase count
    // otherwise, look for all reachable caves not in visited cave already
    // add current cave to visited cave if is_small, and create new states in dfs

    let mut stack = Vec::new();
    stack.push((START, HashSet::new()));

    while let Some((cave, mut visited)) = stack.pop() {
        if cave == END {
            count += 1;
        } else {
            if is_small(cave) {
                visited.insert(cave);
            }
            for &other_cave in &graph[cave] {
                if !visited.contains(other_cave) {
                    stack.push((other_cave, visited.clone()));
                }
            }
        }
    }

    count
}

fn part_2(input: &str) -> usize {
    let graph = build_graph(input);

    let mut count = 0;

    let mut stack = Vec::new();
    stack.push((START, HashSet::new(), None));

    while let Some((cave, mut visited, small)) = stack.pop() {
        if cave == END {
            // make sure we've visited the cave we said we'd visit twice,
            // to avoid duplicates
            if let Some(small) = small {
                if !visited.contains(small) {
                    continue;
                }
            }

            count += 1;
        } else {
            if let Some(small) = small {
                // second visit
                if cave == small {
                    visited.insert(cave);
                }
            }
            if is_small(cave) {
                if small.is_none() && cave != START {
                    for &other_cave in &graph[cave] {
                        if !visited.contains(other_cave) {
                            stack.push((other_cave, visited.clone(), Some(cave)));
                        }
                    }
                }
                visited.insert(cave);
            }
            for &other_cave in &graph[cave] {
                if !visited.contains(other_cave) {
                    stack.push((other_cave, visited.clone(), small));
                }
            }
        }
    }

    count
}
fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{part_1, part_2};

    #[test_case(
        r"start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        10
    )]
    #[test_case(
        r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
        19
    )]
    #[test_case(
        r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
        226
    )]
    #[test_case(crate::INPUT, 4707)]
    fn test_part_1(input: &str, paths: usize) {
        assert_eq!(paths, part_1(input));
    }

    #[test_case(
        r"start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        36
    )]
    #[test_case(
        r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
        103
    )]
    #[test_case(
        r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
        3509
    )]
    #[test_case(crate::INPUT, 130493)]
    fn test_part_2(input: &str, paths: usize) {
        assert_eq!(paths, part_2(input));
    }
}
