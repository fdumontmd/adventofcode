use aoc_utils::permutations::*;
use intcode::*;

static INPUT: &str = include_str!("input.txt");

struct Amplifier {
    computer: Computer,
}

impl Amplifier {
    fn new(code: &str, phase: MemItem) -> Self {
        let mut computer: Computer = code.parse().expect("cannot parse code");
        computer.add_input(phase);
        Amplifier {
            computer
        }
    }

    fn get_output(&mut self, input: MemItem) -> Option<MemItem> {
        self.computer.add_input(input);
        self.computer.wait_until_output()
    }
}

fn thruster_signal<'a>(code: &'a str, phases: Vec<MemItem>) -> MemItem {
    let mut input = 0;
    for phase in phases {
        input = Amplifier::new(code, phase).get_output(input).expect(&format!("no output for phase {} and input {}", phase, input));
    }

    input
}

fn thruster_max_signal<'a>(code: &'a str) -> MemItem {
    let phases: Vec<MemItem> = vec![0,1,2,3,4];
    Permutation::new(phases).map(|phases| thruster_signal(code, phases)).max().expect("no max signal")
}

fn thruster_signal_with_loop<'a>(code: &'a str, phases: Vec<MemItem>) -> MemItem {
    let mut amplifiers: Vec<_> = phases.into_iter().map(|phase| Amplifier::new(code, phase)).collect();

    let mut input = 0;
    let mut final_output = 0;

    for idx in (0..5).cycle() {
        if let Some(output) = amplifiers[idx].get_output(input) {
            input = output;
            if idx == 4 {
                final_output = output;
            }
        } else {
            break;
        }
    }

    final_output
}

fn thruster_max_signal_with_loop<'a>(code: &'a str) -> MemItem {
    let phases: Vec<MemItem> = vec![5,6,7,8,9];
    Permutation::new(phases).map(|phases| thruster_signal_with_loop(code, phases)).max().expect("no max signal")
}

fn main() {
    println!("part 1: {}", thruster_max_signal(INPUT));
    println!("part 2: {}", thruster_max_signal_with_loop(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_amplifiers() {
        assert_eq!(thruster_signal("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", vec![4,3,2,1,0]), 43210);
        assert_eq!(thruster_signal("3,23,3,24,1002,24,10,24,1002,23,-1,23,
101,5,23,23,1,24,23,23,4,23,99,0,0", vec![0,1,2,3,4]), 54321);
        assert_eq!(thruster_signal("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", vec![1,0,4,3,2]), 65210)
    }

    #[test]
    fn test_thruster_max_signal() {
        assert_eq!(thruster_max_signal("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), 43210);
        assert_eq!(thruster_max_signal("3,23,3,24,1002,24,10,24,1002,23,-1,23,
101,5,23,23,1,24,23,23,4,23,99,0,0"), 54321);
        assert_eq!(thruster_max_signal("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210)
    }

    #[test]
    fn test_amplifiers_with_loop() {
        assert_eq!(thruster_signal_with_loop("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26, 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", vec![9,8,7,6,5]), 139629729);
        assert_eq!(thruster_signal_with_loop("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", vec![9,7,8,5,6]), 18216);
    }

    #[test]
    fn test_thruster_max_signal_with_loop() {
        assert_eq!(thruster_max_signal_with_loop("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26, 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);
        assert_eq!(thruster_max_signal_with_loop("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
    }
}
