use std::collections::HashSet;

const INPUT: &'static str = "hepxcrrq";
const A: u8 = 97;
const Z: u8 = 122;

struct Password(String);

impl Password {
    fn new(input: &str) -> Self {
        Password(String::from(input))
    }
}

fn inc(c: u8) -> (u8, u8) {
    let n = c + 1;
    if n > Z {
        (n - Z + A - 1, 1)
    } else {
        (n, 0)
    }
}

impl Iterator for Password {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let original = std::mem::replace(&mut self.0, String::new());

        let mut current: Vec<u8> = original.into();
        current.reverse();

        let mut output = Vec::new();
        let mut carry = true;
        for b in current.into_iter() {
            if carry {
                let (n, c) = inc(b);
                output.push(n);
                carry = c > 0;
            } else {
                output.push(b);
            }
        }

        if carry {
            output.push(A);
        }

        output.reverse();

        self.0.push_str(std::str::from_utf8(&output).unwrap());
        Some(String::from_utf8_lossy(&output).into_owned())
    }
}

fn no_i_o_or_l(input: &str) -> bool {
    ! (input.contains('i') || input.contains('o') || input.contains('l'))
}

fn two_pairs(input: &str) -> bool {
    let mut pairs = HashSet::new();
    for chunks in input.as_bytes().windows(2) {
        if chunks[0] == chunks[1] {
            pairs.insert(chunks[0]);
        }
    }
    pairs.len() >= 2
}

fn three_in_seq(input: &str) -> bool {
    for chunks in input.as_bytes().windows(3) {
        if chunks[1] == chunks[0] + 1 && chunks[2] == chunks[1] + 1 {
            return true;
        }
    }
    false
}

struct PasswordRule {
    password: Password,
    max_len: usize,
    rules: Vec<fn(&str)->bool>,
}

impl PasswordRule {
    fn new(password: &str) -> Self {
        PasswordRule{ password: Password::new(password), max_len: password.len(), rules: Vec::new() }
    }

    fn add_rule(&mut self, f: fn(&str) -> bool) {
        self.rules.push(f);
    }

    fn is_valid_password(&self, input: &str) -> bool {
        self.rules.iter().all(|f| f(input))
    }
}

impl Iterator for PasswordRule {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pass) = self.password.next() {
            if pass.len() > self.max_len {
                return None;
            } else {
                if self.is_valid_password(&pass) {
                    return Some(pass);
                }
            }
        }
        None
    }
}

fn main() {
    let mut iter = PasswordRule::new(INPUT);

    iter.add_rule(no_i_o_or_l);
    iter.add_rule(two_pairs);
    iter.add_rule(three_in_seq);

    println!("Next password: {:?}", iter.next());
    println!("Next next password: {:?}", iter.next());

}

#[test]
fn test_basic_inc() {
    assert_eq!(Some(String::from("xy")), Password::new(&"xx").next());
}

#[test]
fn test_valid_passwords() {
    let mut passwords = PasswordRule::new("abcdefgh");
    passwords.add_rule(no_i_o_or_l);
    passwords.add_rule(two_pairs);
    passwords.add_rule(three_in_seq);
    assert_eq!(Some(String::from("abcdffaa")), passwords.next());
}
