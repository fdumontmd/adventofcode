use std::fmt::Write;

const INPUT: &'static str = "3113322113";

fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut previous = None;
    let mut run_count = 0;
    for c in input.chars() {
        if Some(c) != previous && previous.is_some() {
            write!(output, "{}", run_count).unwrap();
            output.push(previous.unwrap());
            run_count = 0;
        }
        previous = Some(c);
        run_count += 1;
    }
    if previous.is_some() {
        write!(output, "{}", run_count).unwrap();
        output.push(previous.unwrap());
    }
    output
}

fn main() {
    let mut result = String::from(INPUT);
    for _ in 0..40 {
        result = look_and_say(&result);
    }

    println!("resulting lenght after 40 iterations: {}", result.len());

    for _ in 0..10 {
        result = look_and_say(&result);
    }

    println!("resulting lenght after 10 more iterations: {}", result.len());
}

#[test]
fn test() {
    assert_eq!("11", look_and_say("1"));
    assert_eq!("21", look_and_say("11"));
    assert_eq!("1211", look_and_say("21"));
    assert_eq!("111221", look_and_say("1211"));
    assert_eq!("312211", look_and_say("111221"));
}
