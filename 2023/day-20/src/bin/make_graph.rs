pub fn main() {
    let input = include_str!("../../input.txt");
    println!("digraph {{");
    println!(
        "{{
node [shape=circle style=filled]"
    );

    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let name = parts.next().unwrap();
        if let Some(name) = name.strip_prefix('%') {
            // flipflop
            println!("{name} [fillcolor=yellow]");
        }
        if let Some(name) = name.strip_prefix('&') {
            println!("{name} [fillcolor=green]");
        // disjunction
        } else {
            // other
        }
    }
    println!("}}");

    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let name = parts.next().unwrap();
        let name = if let Some(name) = name.strip_prefix('%') {
            name
        } else if let Some(name) = name.strip_prefix('&') {
            name
        } else {
            name
        };

        for target in parts.next().unwrap().split(", ") {
            println!("{name} -> {target}");
        }
    }
    println!("}}");
}
