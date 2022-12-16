use crate::parser::parse_input;

static INPUT: &str = include_str!("input.txt");
static ORIGIN: &str = "AA";

// yes, I hate that code too...
// it does compute the correct answer, but only for my input and the test data... I'm pretty sure
// it won't work for your input

mod parser {
    use itertools::Itertools;
    use std::{
        cmp::Reverse,
        collections::{BTreeMap, BinaryHeap, HashSet},
    };

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::ORIGIN;
    #[derive(Debug)]
    struct Valve {
        id: String,
        flow: usize,
        tunnels: Vec<String>,
    }

    impl Valve {
        fn parse(line: &str) -> Self {
            lazy_static! {
                static ref VALVE_PARSER: Regex = Regex::new(
                    r"Valve ([^ ]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)"
                )
                .unwrap();
            }

            let Some(cap) = VALVE_PARSER.captures(line) else { panic!("Cannot parse {}", line); };
            let id = cap[1].to_string();
            let flow = cap[2].parse().unwrap();
            let tunnels: Vec<_> = cap[3].split(", ").map(|id| id.to_string()).collect();

            Valve { id, flow, tunnels }
        }
    }

    fn distance_to_targets(
        valves: &BTreeMap<String, Valve>,
        id: &str,
        targets: &HashSet<&str>,
    ) -> BTreeMap<String, usize> {
        let mut distances = BTreeMap::new();
        let mut seen = HashSet::new();
        let mut nodes = BinaryHeap::new();
        nodes.push((Reverse(0), id));
        seen.insert(id);

        while let Some((dist, id)) = nodes.pop() {
            let new_dist = dist.0 + 1;
            for valve in &valves.get(id).unwrap().tunnels {
                if seen.contains(&**valve) {
                    continue;
                }
                seen.insert(&**valve);
                if targets.contains(&**valve) {
                    distances.insert(valve.clone(), new_dist);
                }
                nodes.push((Reverse(new_dist), valve));
            }
            if distances.len() == targets.len() {
                break;
            }
        }

        distances
    }

    #[derive(Debug)]
    pub struct Tunnel {
        pub len: usize,
        pub valve_id: String,
        pub flow: usize,
    }

    #[derive(Debug)]
    pub struct Tunnels(pub BTreeMap<String, Vec<Tunnel>>);

    pub(crate) fn parse_input(input: &str) -> Tunnels {
        let valves: BTreeMap<String, Valve> = BTreeMap::from_iter(
            input
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(Valve::parse)
                .map(|v| (v.id.clone(), v)),
        );

        let target_valves: HashSet<&str> = valves
            .values()
            .filter(|v| v.flow > 0)
            .map(|v| v.id.as_ref())
            .collect();

        let mut distance_map: BTreeMap<String, Vec<Tunnel>> = BTreeMap::new();

        for (id, dist) in distance_to_targets(&valves, ORIGIN, &target_valves) {
            distance_map
                .entry(ORIGIN.to_string())
                .or_default()
                .push(Tunnel {
                    len: dist,
                    valve_id: id.clone(),
                    flow: valves[&id].flow,
                });
        }

        for target in &target_valves {
            for (id, dist) in distance_to_targets(&valves, target, &target_valves) {
                distance_map
                    .entry(target.to_string())
                    .or_default()
                    .push(Tunnel {
                        len: dist,
                        valve_id: id.clone(),
                        flow: valves[&id].flow,
                    });
            }
        }

        Tunnels(distance_map)
    }

    impl Tunnels {
        pub fn iter(&self, time: usize, players: usize) -> TunnelIterator {
            TunnelIterator {
                tunnels: self,
                stack: vec![],
                started: false,
                open: HashSet::new(),
                time,
                players,
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Decision<'a> {
        id: &'a str,
        idx: usize,
        time: usize,
        flow: usize,
        player: usize,
    }

    pub struct TunnelIterator<'a> {
        tunnels: &'a Tunnels,
        // time to replace that by a struct
        stack: Vec<Decision<'a>>,
        started: bool,
        open: HashSet<&'a str>,
        time: usize,
        players: usize,
    }

    impl<'a> TunnelIterator<'a> {
        fn push(&mut self, id: &'a str, idx: usize, player: usize) -> bool {
            let time = self
                .stack
                .iter()
                .rev()
                .find(|d| d.player == player)
                .map(|d| d.time)
                .unwrap_or(self.time);
            if time <= self.tunnels.0[id][idx].len {
                return false;
            }

            let time = time - self.tunnels.0[id][idx].len - 1;
            let flow = self.tunnels.0[id][idx].flow * time;
            assert!(id == ORIGIN || !self.open.contains(&*self.tunnels.0[id][idx].valve_id));

            self.stack.push(Decision {
                id,
                idx,
                time,
                flow,
                player,
            });

            self.open.insert(id);
            self.open.insert(&self.tunnels.0[id][idx].valve_id);
            true
        }

        fn complete(&mut self) {
            if self.stack.is_empty() {
                self.push(ORIGIN, 0, 0);
            }

            while self.stack.len() < self.players {
                let top = self.stack.last().unwrap();
                let idx = top.idx;
                if idx + 1 < self.tunnels.0[top.id].len() {
                    self.push(top.id, idx + 1, self.stack.len());
                } else {
                    // ran out of options
                    return;
                }
            }

            let mut player_times = vec![self.time; self.players];
            self.stack
                .iter()
                .for_each(|d| player_times[d.player] = player_times[d.player].min(d.time));
            'outer: loop {
                let Some(earliest_player) = player_times.iter().position_max() else { panic!("No players?"); };
                let Some(&curr) = self.stack
                    .iter()
                    .rev()
                    .find(|d| d.player == earliest_player) else { panic!("Selected agent {earliest_player} did not act yet?"); };
                // that's the part that needs fixing:
                // 1. each element in the stack can have a parent,
                // which is not simply the previous element; it can be older
                // 2. need to take time into account; each "player" must
                // move as soon as possible
                //let curr = self.stack[self.stack.len() - 1];

                let next_id = &self.tunnels.0[curr.id][curr.idx].valve_id;

                for (idx, tunnel) in self.tunnels.0[next_id].iter().enumerate() {
                    if !self.open.contains(&*tunnel.valve_id) && curr.time > tunnel.len {
                        let new_time = curr.time - tunnel.len - 1;
                        self.stack.push(Decision {
                            id: next_id,
                            idx,
                            time: new_time,
                            flow: tunnel.flow * new_time,
                            player: curr.player,
                        });
                        self.open.insert(&tunnel.valve_id);
                        player_times[curr.player] = new_time;
                        continue 'outer;
                    }
                }

                break;
            }

            self.started = true;
        }

        fn total_flow(&self) -> Option<usize> {
            if self.stack.is_empty() {
                None
            } else {
                let tf = self.stack.iter().map(|d| d.flow).sum();
                if tf > 2745 {
                    dbg!(&self.tunnels.0);
                    dbg!(&self.stack);
                }
                Some(tf)
            }
        }
    }

    impl<'a> Iterator for TunnelIterator<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.started {
                self.complete();
                self.total_flow()
            } else {
                let Some(d) = self.stack.pop() else { return None; };
                let curr = d.id;
                let idx = d.idx;
                self.open.remove(&*self.tunnels.0[d.id][idx].valve_id);

                for idx in idx + 1..self.tunnels.0[curr].len() {
                    let next_id = &self.tunnels.0[curr][idx].valve_id;
                    if self.open.contains(&**next_id) {
                        continue;
                    }

                    if !self.push(curr, idx, d.player) {
                        continue;
                    }

                    self.complete();

                    return self.total_flow();
                }

                self.next()
            }
        }
    }
}

fn part_01(input: &str) -> usize {
    let tunnels = parse_input(input);

    tunnels.iter(30, 1).max().unwrap()
}

fn part_02(input: &str) -> usize {
    let tunnels = parse_input(input);

    tunnels.iter(26, 2).max().unwrap()
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02, INPUT};

    static TEST_INPUT: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_01() {
        assert_eq!(1651, part_01(TEST_INPUT));
    }

    #[test]
    fn real_01() {
        assert_eq!(1845, part_01(INPUT));
    }

    #[test]
    fn test_02() {
        assert_eq!(1707, part_02(TEST_INPUT));
    }

    #[test]
    fn real_02() {
        assert_eq!(2286, part_02(INPUT));
    }
}
