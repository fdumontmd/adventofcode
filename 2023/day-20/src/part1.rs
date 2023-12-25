use std::collections::{BTreeMap, VecDeque};

use crate::custom_error::AocError;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FlipFlop {
    state: bool,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Conjunction {
    inputs: BTreeMap<&'static str, bool>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Module {
    name: &'static str,
    module: ModuleType,
    output: Vec<&'static str>,
}

impl Module {
    pub fn receive_pulse(&mut self, from: &'static str, pulse: PulseType) -> Option<PulseType> {
        //eprintln!("{} -{:?}-> {}", from, pulse, self.name);
        match self.module {
            ModuleType::Broadcaster => Some(pulse),
            ModuleType::FlipFlop(ref mut fp) => match pulse {
                PulseType::Low => {
                    fp.state = !fp.state;
                    if fp.state {
                        Some(PulseType::High)
                    } else {
                        Some(PulseType::Low)
                    }
                }
                PulseType::High => None,
            },
            ModuleType::Conjunction(ref mut c) => {
                c.inputs.insert(from, pulse == PulseType::High);
                if c.inputs.values().all(|b| *b) {
                    Some(PulseType::Low)
                } else {
                    Some(PulseType::High)
                }
            }
        }
    }

    pub fn inputs(&self) -> &BTreeMap<&'static str, bool> {
        static DEFAULT: BTreeMap<&'static str, bool> = BTreeMap::new();
        match self.module {
            ModuleType::Broadcaster => &DEFAULT,
            ModuleType::FlipFlop(_) => &DEFAULT,
            ModuleType::Conjunction(ref c) => &c.inputs,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PulseType {
    Low,
    High,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pulse {
    pulse_type: PulseType,
    destination: &'static str,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct State {
    modules: BTreeMap<&'static str, Module>,
}

#[tracing::instrument]
pub fn parse_input(input: &'static str) -> State {
    let mut modules = BTreeMap::new();
    for line in input.lines() {
        let mut rule = line.split(" -> ");
        let name = rule.next().unwrap();
        let output: Vec<&'static str> = rule.next().unwrap().split(", ").collect();

        let module = if name == "broadcaster" {
            Module {
                name,
                module: ModuleType::Broadcaster,
                output,
            }
        } else if let Some(name) = name.strip_prefix('%') {
            Module {
                name,
                module: ModuleType::FlipFlop(FlipFlop::default()),
                output,
            }
        } else if let Some(name) = name.strip_prefix('&') {
            Module {
                name,
                module: ModuleType::Conjunction(Conjunction::default()),
                output,
            }
        } else {
            panic!("Cannot parse rule {line}")
        };
        modules.insert(module.name, module);
    }

    let mut all_inputs = BTreeMap::new();

    // for all Conjunction, find inputs
    for module in modules.values() {
        if let ModuleType::Conjunction(_) = module.module {
            let target = module.name;
            let mut inputs = Vec::new();

            for m in modules.values() {
                if m.output.contains(&target) {
                    inputs.push(m.name);
                }
            }
            all_inputs.insert(module.name, inputs);
        }
    }
    for module in modules.values_mut() {
        if let ModuleType::Conjunction(ref mut c) = module.module {
            c.inputs = BTreeMap::from_iter(all_inputs[module.name].iter().map(|n| (*n, false)));
        }
    }

    State { modules }
}

impl State {
    pub fn press_button(&mut self) -> (usize, usize) {
        let mut low: usize = 0;
        let mut high: usize = 0;

        let mut queue = VecDeque::new();

        queue.push_front(("button", PulseType::Low, "broadcaster"));
        low += 1;

        while let Some((from, pulse, target)) = queue.pop_back() {
            if let Some(target) = self.modules.get_mut(target) {
                if let Some(pulse) = target.receive_pulse(from, pulse) {
                    if pulse == PulseType::Low {
                        low += target.output.len();
                    } else {
                        high += target.output.len();
                    }

                    for name in &target.output {
                        queue.push_front((target.name, pulse, name));
                    }
                }
            }
        }

        (low, high)
    }

    pub fn press_button_with_monitor(&mut self, monitor: &str) -> bool {
        let mut queue = VecDeque::new();

        queue.push_front(("button", PulseType::Low, "broadcaster"));
        while let Some((from, pulse, target)) = queue.pop_back() {
            if from == monitor && pulse == PulseType::High {
                // eprintln!("{monitor} pulsed high");
                return true;
            }
            if let Some(target) = self.modules.get_mut(target) {
                if let Some(pulse) = target.receive_pulse(from, pulse) {
                    for name in &target.output {
                        queue.push_front((target.name, pulse, name));
                    }
                }
            }
        }
        false
    }
}

#[tracing::instrument]
pub fn process(input: &'static str) -> Result<String, AocError> {
    let mut state = parse_input(input);
    let mut low = 0;
    let mut high = 0;

    // looks dump, but part 2 shows it's not: there's no cycle in the
    // first 1000 button presses. Probably no cycle in the first billion
    // button presses
    for _ in 0..1000 {
        let (l, h) = state.press_button();
        low += l;
        high += h;
    }

    Ok(format!("{}", low * high))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    static INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[rstest]
    #[case(INPUT1, "32000000")]
    #[case(INPUT2, "11687500")]
    #[case(include_str!("../input.txt"), "819397964")]
    fn test_process(#[case] input: &'static str, #[case] res: &str) -> miette::Result<()> {
        assert_eq!(res, process(input)?);
        Ok(())
    }
}
