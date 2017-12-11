use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::fmt;

use std::marker;

type Floor = usize;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Locations<'s>(BTreeMap<&'s str, Floor>);

impl<'s> Locations<'s> {
    fn new() -> Self {
        Locations(BTreeMap::new())
    }

    fn add(&mut self, kind: &'s str, floor: Floor) {
        self.0.insert(kind, floor);
    }

    fn move_to(&mut self, kind: &'s str, floor: Floor) {
        self.0.insert(kind, floor);
    }

    fn list(&self, floor: Floor) -> Vec<&'s str> {
        self.0.iter().filter(|p| *p.1 == floor).map(|p| *p.0).collect()
    }

    fn find(&self, kind: &'s str) -> Floor {
        *self.0.get(kind).unwrap()
    }
}

impl<'a, 's> IntoIterator for &'a Locations<'s> {
    type Item = (&'a &'s str, &'a Floor);
    type IntoIter = std::collections::btree_map::Iter<'a, &'s str, Floor>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Clone, Copy, Debug)]
enum Payload<'p> {
    Microchip(&'p str),
    Generator(&'p str),
}

#[derive(Clone, Copy, Debug)]
struct Action<'a> {
    to_floor: Floor,
    first: Payload<'a>,
    second: Option<Payload<'a>>,
}

impl<'a> Action<'a> {
    fn move_generator(kind: &'a str, to_floor: Floor) -> Action<'a> {
        Action{
            to_floor: to_floor,
            first: Payload::Generator(kind),
            second: None,
        }
    }
    fn move_two_generators(kind: &'a str, other: &'a str, to_floor: Floor) -> Action<'a> {
        Action{
            to_floor: to_floor,
            first: Payload::Generator(kind),
            second: Some(Payload::Generator(other)),
        }
    }
    fn move_microchip(kind: &'a str, to_floor: Floor) -> Action<'a> {
        Action{
            to_floor: to_floor,
            first: Payload::Microchip(kind),
            second: None,
        }
    }
    fn move_two_microchips(kind: &'a str, other: &'a str, to_floor: Floor) -> Action<'a> {
        Action{
            to_floor: to_floor,
            first: Payload::Microchip(kind),
            second: Some(Payload::Microchip(other)),
        }
    }
    fn move_both(kind: &'a str, to_floor: Floor) -> Action<'a> {
        Action {
            to_floor: to_floor,
            first: Payload::Generator(kind),
            second: Some(Payload::Microchip(kind)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<'s> {
    current_floor: Floor,
    microchips: Locations<'s>,
    generators: Locations<'s>,
}

impl<'s> SearchSpaceState for State<'s> {
    type Action = Action<'s>;

    fn valid_actions(&self) -> Vec<(Action<'s>, State<'s>)> {
        self.possible_actions().into_iter().map(|a| (a, self.perform(&a))).filter(|&(_, ref s)| s.is_valid()).collect()
    }

    fn is_final(&self) -> bool {
        // floor 1 to 3 are empty (so everything is on fourth floor)
        (1..4).all(|f| self.generators.list(f).is_empty()
                   && self.microchips.list(f).is_empty())
    }
}

impl<'s> fmt::Display for State<'s> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "State is final: {}\n", self.is_final())?;
        write!(fmt, "Elevator: {}\n", self.current_floor)?;
        for floor in 1..5 {
            write!(fmt, "Floor {}: Generators: {:?}\n", floor, self.generators.list(floor))?;
            write!(fmt, "Floor {}: Microchips: {:?}\n", floor, self.microchips.list(floor))?;
        }

        Ok(())
    }
}

impl<'s> State<'s> {
    fn new(current_floor: Floor,
           microchips: Locations<'s>,
           generators: Locations<'s>) -> Self {
        State{
            current_floor: current_floor,
            microchips: microchips,
            generators: generators,
        }
    }

    fn is_valid(&self) -> bool {
        // for each kind of microchip, either the microchip is on the same
        // floor as the generator, or the floor does not contain any generators
        for (kind, &floor) in &self.microchips {
            if self.generators.find(kind) != floor {
                if !self.generators.list(floor).is_empty() {
                    return false;
                }
            }
        }
        true
    }

    fn possible_actions(&self) -> Vec<Action<'s>> {
        let mut actions = Vec::new();
        for &floor in [self.current_floor - 1,
                       self.current_floor + 1].iter() {
            if floor == 0 || floor == 5 {
                continue;
            }

            let generators = self.generators.list(self.current_floor);
            let mut gen_iter = generators.iter();

            let microchips = self.microchips.list(self.current_floor);
            let mut micro_iter = microchips.iter();


            while let Some(generator) = gen_iter.next() {
                actions.push(Action::move_generator(generator, floor));

                if !gen_iter.as_slice().is_empty() {
                    for other in gen_iter.as_slice() {
                        actions.push(Action::move_two_generators(generator, other, floor));
                    }
                }
            }

            while let Some(microchip) = micro_iter.next() {
                actions.push(Action::move_microchip(microchip, floor));

                if !micro_iter.as_slice().is_empty() {
                    for other in micro_iter.as_slice() {
                        actions.push(Action::move_two_microchips(microchip, other, floor));
                    }
                }

                if self.generators.find(microchip) == self.current_floor {
                    actions.push(Action::move_both(microchip, floor));
                }
            }
        }
        actions
    }

    fn perform(&self, action: &Action<'s>) -> State<'s> {
        let mut microchips = self.microchips.clone();
        let mut generators = self.generators.clone();

        match action.first {
            Payload::Generator(ref kind) => generators.move_to(kind, action.to_floor),
            Payload::Microchip(ref kind) => microchips.move_to(kind, action.to_floor),
        };

        if let Some(ref pl) = action.second {
            match pl {
                &Payload::Generator(ref kind) => generators.move_to(kind, action.to_floor),
                &Payload::Microchip(ref kind) => microchips.move_to(kind, action.to_floor),
            };
        }

        State::new(action.to_floor, microchips, generators)
    }
}

trait SearchSpaceState: Sized {
    type Action;
    fn valid_actions(&self) -> Vec<(Self::Action, Self)>;
    fn is_final(&self) -> bool;
}

struct SearchState<T: SearchSpaceState> {
    depth: usize,
    state: T,
    action: Option<T::Action>,
}

impl<T: SearchSpaceState+PartialEq> PartialEq for SearchState<T> {
    fn eq(&self, other: &SearchState<T>) -> bool {
        self.state == other.state && self.depth == other.depth
    }
}

impl<T: SearchSpaceState+Eq> Eq for SearchState<T> {}

impl<T: SearchSpaceState+Eq> PartialOrd for SearchState<T> {
    fn partial_cmp(&self, other: &SearchState<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: SearchSpaceState+Eq> Ord for SearchState<T> {
    fn cmp(&self, other: &SearchState<T>) -> Ordering {
        other.depth.cmp(&self.depth)
    }
}

impl<T: SearchSpaceState> SearchState<T> {
    fn new(initial_state: T) -> Self {
        SearchState{
            depth: 0,
            state: initial_state,
            action: None,
        }
    }

    fn into_children(self) -> Vec<SearchState<T>> {
        self.state.valid_actions().into_iter().map(|(a,s)| {
            SearchState{
                depth: self.depth + 1,
                state: s,
                action: Some(a),
            }
        }).collect()
    }
}

struct Search<'a, T: SearchSpaceState + 'a> {
    fringe: BinaryHeap<SearchState<T>>,
    seen_states: HashSet<T>,
    _marker: marker::PhantomData<&'a T>,
}

impl<'a, T: SearchSpaceState+Clone+Eq> Search<'a, T> where T: std::hash::Hash {
    fn new(state: T) -> Self {
        let mut fringe = BinaryHeap::new();
        fringe.push(SearchState::new(state.clone()));
        let mut seen_states = HashSet::new();
        seen_states.insert(state);

        Search{ _marker: marker::PhantomData, fringe: fringe, seen_states: seen_states }
    }

    fn next(&mut self) -> Option<&SearchState<T>> {
        if let Some(ss) = self.fringe.pop() {
            let children = ss.into_children();
            for child in children {
                if self.seen_states.contains(&child.state) {
                    continue;
                } else {
                    self.seen_states.insert(child.state.clone());
                    self.fringe.push(child)
                }
            }
            self.fringe.peek()
        } else {
            None
        }
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_two() {
    println!("Part two:");
    let mut microchips = Locations::new();
    microchips.add("promethium", 1);
    microchips.add("elerium", 1);
    microchips.add("dilithium", 1);
    microchips.add("cobalt", 3);
    microchips.add("curium", 3);
    microchips.add("ruthenium", 3);
    microchips.add("plutonium", 3);

    let mut generators = Locations::new();
    generators.add("promethium", 1);
    generators.add("elerium", 1);
    generators.add("dilithium", 1);
    generators.add("cobalt", 2);
    generators.add("curium", 2);
    generators.add("ruthenium", 2);
    generators.add("plutonium", 2);

    let state = State::new(1, microchips, generators);

    let mut search = Search::new(state);

    loop {
        if let Some(s) = search.next() {
            if s.state.is_final() {
                println!("Depth: {}", s.depth);
                break;
            }
        } else {
            println!("No solution found");
        }
    }
}

fn part_one() {
    println!("Part one:");
    let mut microchips = Locations::new();
    microchips.add("promethium", 1);
    microchips.add("cobalt", 3);
    microchips.add("curium", 3);
    microchips.add("ruthenium", 3);
    microchips.add("plutonium", 3);

    let mut generators = Locations::new();
    generators.add("promethium", 1);
    generators.add("cobalt", 2);
    generators.add("curium", 2);
    generators.add("ruthenium", 2);
    generators.add("plutonium", 2);

    let state = State::new(1, microchips, generators);

    let mut search = Search::new(state);

    loop {
        if let Some(s) = search.next() {
            if s.state.is_final() {
                println!("Depth: {}", s.depth);
                break;
            }
        } else {
            println!("No solution found");
        }
    }
}

#[test]
fn test() {
    let mut microchips = Locations::new();
    microchips.add("hydrogen", 1);
    microchips.add("lithium", 1);

    let mut generators = Locations::new();
    generators.add("hydrogen", 2);
    generators.add("lithium", 3);

    let state = State::new(1, microchips, generators);

    let mut search = Search::new(state);

    loop {
        if let Some(s) = search.next() {
            println!("depth: {}", s.depth);
            println!("Action: {:?}", s.action);
            println!("{}", s.state);
            if s.state.is_final() {
                assert_eq!(11, s.depth);
                break;
            }
        } else {
            println!("No solution found");
            assert!(false);
        }
    }
}
