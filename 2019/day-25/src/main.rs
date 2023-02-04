use std::fmt::Write;

use intcode::Computer;

static INPUT: &str = include_str!("input.txt");

struct Ascii {
    computer: Computer,
}

// hardcode the navigation for now
// ideas:
// - code in current shape could be automated; just need a method
//   to extract possible movements from room description
// - need to understand how to avoid specific commands:
//   - for a given room, if a command causes the computer to
//     stop, avoid that command in the given room
static SCRIPT: &[&str] = &[
    "north",
    "take wreath",
    "east",
    "east",
    "east",
    "take weather machine",
    "west",
    "west",
    "west",
    "south",
    "south",
    "south",
    "take candy cane",
    "north",
    "west",
    "take prime number",
    "west",
    "take astrolabe",
    "east",
    "east",
    "north",
    "east",
    "take food ration",
    "south",
    "east",
    "south",
    "take hypercube",
    "east",
    "take space law space brochure",
    "north",
    "inv",
];

impl Ascii {
    fn execute(&mut self, cmd: &str) {
        println!("{cmd}");
        let cmd = cmd.trim();
        for b in cmd.trim().bytes() {
            self.computer.add_input(b as i64);
        }
        self.computer.add_input(10);
    }
    // refactor to extract a state with:
    // - room name
    // - exits
    // - items
    fn show_output(&mut self) -> String {
        let mut line = String::new();
        while let Some(o) = self.computer.wait_until_output() {
            if let Ok(b) = u8::try_from(o) {
                write!(&mut line, "{}", b as char).expect("write, dammit");

                if line.ends_with("Command?") {
                    return line;
                }
            }
        }
        line
    }

    fn get_inventory(&mut self) -> Vec<String> {
        self.execute("inv");

        let inv = self.show_output();
        inv.lines()
            .filter_map(|l| l.strip_prefix("- ").map(|i| i.to_owned()))
            .collect()
    }
}

fn main() {
    let computer: Computer = INPUT.parse().unwrap();
    let mut comp = Ascii { computer };

    for cmd in SCRIPT {
        print!("{}", comp.show_output());
        comp.execute(cmd);
    }
    print!("{}", comp.show_output());
    let all_items = comp.get_inventory();

    let max = 2 << all_items.len();
    for comb in 0..max {
        let carried_items = comp.get_inventory();
        for i in carried_items {
            comp.execute(&format!("drop {i}"));
            print!("{}", comp.show_output());
        }

        for (idx, i) in all_items.iter().enumerate() {
            if comb & (2 << idx) != 0 {
                comp.execute(&format!("take {i}"));
                print!("{}", comp.show_output());
            }
        }

        comp.execute("west");
        let resp = comp.show_output();
        println!("{resp}");
        if !resp.contains("ejected back") {
            break;
        }
    }
}
