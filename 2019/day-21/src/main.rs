use intcode::Ascii;

static SCRIPT_PART_1: &[&str] = &[
    "NOT C T", "NOT B J", "OR T J", "AND D J", "NOT A T", "OR T J", "WALK",
];

static SCRIPT_PART_2: &[&str] = &[
    "NOT A T", "OR T J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT D T", "OR E T",
    "OR H T", "AND T J", "RUN",
];

static INPUT: &str = include_str!("input.txt");

fn part_1() -> i64 {
    let mut computer: Ascii = INPUT.parse().unwrap();

    for cmd in SCRIPT_PART_1 {
        print!("{}", computer.show_output());
        computer.execute(cmd);
    }
    print!("{}", computer.show_output());
    computer.non_ascii_output().unwrap()
}

fn part_2() -> Option<i64> {
    let mut computer: Ascii = INPUT.parse().unwrap();
    for cmd in SCRIPT_PART_2 {
        print!("{}", computer.show_output());
        computer.execute(cmd);
    }
    print!("{}", computer.show_output());
    computer.non_ascii_output()
}
fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {:?}", part_2());
}
