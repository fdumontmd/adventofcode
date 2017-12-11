use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    assert!(args().len() > 1);
    let input = File::open(args().nth(1).unwrap()).unwrap();

    let (groups, garbage) = groups_sum(&mut &input);

    println!("Group sum: {}", groups);
    println!("garbage sum: {}", garbage);
}

fn groups_sum(input: &mut Read) -> (u32, u32) {
    let mut bytes = input.bytes();
    let mut groups = Vec::new();

    let mut sum = 0;
    let mut garbage = 0;

    while let Some(c) = bytes.next() {
        match c.unwrap() {
            b'{' => {
                let previous = if groups.is_empty() {
                    0
                } else {
                    groups[groups.len() - 1]
                };
                groups.push(previous + 1);
            }
            b'}' => {
                sum += groups.pop().unwrap();
            }
            b'<' => {
                let mut escape = false;
                while let Some(c) = bytes.next() {
                    if escape {
                        escape = false;
                        continue;
                    }
                    match c.unwrap() {
                        b'!' => escape = true,
                        b'>' => break,
                        _ => garbage += 1,
                    }
                }
            }
            _ => {}
        }
    }

    (sum, garbage)
}

#[test]
fn test() {
    use std::io::Cursor;
    assert_eq!(groups_sum(&mut Cursor::new("{}")).0, 1);
    assert_eq!(groups_sum(&mut Cursor::new("{{{}}}")).0, 6);
    assert_eq!(groups_sum(&mut Cursor::new("{{}{}}")).0, 5);
    assert_eq!(groups_sum(&mut Cursor::new("{{{},{},{{}}}}")).0, 16);
    assert_eq!(groups_sum(&mut Cursor::new("{<a>,<a>,<a>,<a>}")).0, 1);
    assert_eq!(groups_sum(&mut Cursor::new("{{<ab>},{<ab>},{<ab>},{<ab>}}")).0, 9);
    assert_eq!(groups_sum(&mut Cursor::new("{{<!!>},{<!!>},{<!!>},{<!!>}}")).0, 9);
    assert_eq!(groups_sum(&mut Cursor::new("{{<a!>},{<a!>},{<a!>},{<ab>}}")).0, 3);

}
