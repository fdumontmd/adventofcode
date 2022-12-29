#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::{self, Read};
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

type Chip = u64;
type Id = usize;

#[derive(Clone, Copy)]
enum Destination {
    Bot(Id),
    Output(Id),
}

impl Destination {
    fn new(dest: &str, id: Id) -> Self {
        if dest == "bot" {
            Destination::bot(id)
        } else if dest == "output" {
            Destination::output(id)
        } else {
            unreachable!()
        }
    }
    fn bot(id: Id) -> Self {
        Destination::Bot(id)
    }
    fn output(id: Id) -> Self {
        Destination::Output(id)
    }
}

struct Program {
    low: Destination,
    high: Destination,
}

impl Program {
    fn new(low_dest: &str, low_id: Id, high_dest: &str, high_id: Id) -> Self {
        Program{
            low: Destination::new(low_dest, low_id),
            high: Destination::new(high_dest, high_id),
        }
    }
}

struct Bot {
    id: Id,
    chips: Vec<Chip>,
    program: Program,
}

impl Bot {
    fn new(id: Id, program: Program) -> Self {
        Bot{ id: id, chips: Vec::new(), program: program }
    }
    fn is_ready(&self) -> bool {
        self.chips.len() == 2
    }
    fn actions(&mut self) -> Vec<Action> {
        assert!(self.is_ready());
        if self.chips[0] > self.chips[1] {
            self.chips.swap(0, 1);
        }
        let low = self.chips[0];
        let high = self.chips[1];

        if low == 17 && high == 61 {
            println!("Bot {} just compared 17 and 61", self.id);
        }

        let mut actions = Vec::new();
        actions.push(Action::new(low, self.program.low));
        actions.push(Action::new(high, self.program.high));

        self.chips.clear();
        actions
    }
}

struct Action {
    chip: Chip,
    destination: Destination,
}

impl Action {
    fn new(chip: Chip, destination: Destination) -> Self {
        Action{ chip: chip, destination: destination }
    }
}

struct Factory {
    bots: HashMap<Id, Bot>,
    outputs: HashMap<Id, Vec<Chip>>,
}

impl Factory {
    fn new() -> Self {
        Factory{ bots: HashMap::new(), outputs: HashMap::new() }
    }

    fn process_ready_bots(&mut self, bots: Vec<Id>) -> Vec<Action> {
        let mut actions = Vec::new();
        for bot in bots {
            actions.append(&mut self.bots.get_mut(&bot).unwrap().actions());
        }
        actions
    }

    fn process_actions(&mut self, actions: Vec<Action>) -> Vec<Id> {
        let mut ready_bots = Vec::new();
        for action in actions {
            match action.destination {
                Destination::Bot(id) => {
                    let bot = self.bots.get_mut(&id).unwrap();
                    bot.chips.push(action.chip);
                    if bot.is_ready() {
                        ready_bots.push(id);
                    }
                }
                Destination::Output(id) => self.outputs.entry(id)
                    .or_insert(Vec::new()).push(action.chip),
            }
        }
        ready_bots
    }

    fn start(&mut self, actions: Vec<Action>) {
        let mut actions = actions;
        loop {
            let ready_bots = self.process_actions(actions);
            if ready_bots.is_empty() {
                break;
            }
            actions = self.process_ready_bots(ready_bots);
        }
    }

    fn build_factory(commands: &str) -> (Factory, Vec<Action>) {
        lazy_static! {
            static ref BOT: Regex =
                Regex::new(r"bot (\d+) gives low to (\w+) (\d+) and high to (\w+) (\d+)").unwrap();
            static ref ACTION: Regex =
                Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
        }

        let mut factory = Factory::new();
        let mut actions = Vec::new();

        for command in commands.lines() {
            if let Some(caps) = BOT.captures(command) {
                let bot_id = Id::from_str(caps.get(1).unwrap().as_str()).unwrap();
                let low_dest = caps.get(2).unwrap().as_str();
                let low_id = Id::from_str(caps.get(3).unwrap().as_str()).unwrap();
                let high_dest = caps.get(4).unwrap().as_str();
                let high_id = Id::from_str(caps.get(5).unwrap().as_str()).unwrap();

                factory.bots.insert(bot_id,
                                    Bot::new(bot_id, Program::new(
                                        low_dest, low_id,
                                        high_dest, high_id
                                    )));
            } else if let Some(caps) = ACTION.captures(command) {
                let chip = Chip::from_str(caps.get(1).unwrap().as_str()).unwrap();
                let bot = Id::from_str(caps.get(2).unwrap().as_str()).unwrap();
                actions.push(Action::new(chip, Destination::bot(bot)));
            } else {
                unreachable!();
            }
        }

        (factory, actions)
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let (mut factory, actions) = Factory::build_factory(&buffer);

    factory.start(actions);
    let product: u64 = factory.outputs.get(&0).unwrap().iter().product::<u64>()
        * factory.outputs.get(&1).unwrap().iter().product::<u64>()
        * factory.outputs.get(&2).unwrap().iter().product::<u64>();

    println!("Product of ouput 0, 1, and 2: {}", product);

}
