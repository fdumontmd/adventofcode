#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::{self, Read};
use std::str::Chars;
use regex::Regex;

#[derive(PartialEq, Eq)]
enum ABBA {
    Empty,
    OneChar(char),
    TwoChars(char, char),
    ThreeChars(char, char),
    Matched,
}

impl ABBA {
    fn new() -> Self {
        ABBA::Empty
    }

    fn is_abba(&self) -> bool {
        *self == ABBA::Matched
    }

    fn push(&mut self, c: char) {
        use ABBA::*;
        *self = match *self {
            Empty => OneChar(c),
            OneChar(a) => {
                if c != a {
                    TwoChars(a, c)
                } else {
                    OneChar(c)
                }
            }
            TwoChars(a, b) => {
                if c == b {
                    ThreeChars(a, c)
                } else {
                    TwoChars(b, c)
                }
            }
            ThreeChars(a, d) => {
                if c == a {
                    Matched
                } else if c == d {
                    OneChar(d)
                } else {
                    TwoChars(d, c)
                }
            }
            Matched => Matched
        }
    }
}

fn is_abba(component: Option<&str>) -> bool {
    if let Some(component) = component {
        if component.len() >= 4 {
            let mut abba_parser = ABBA::new();
            for c in component.chars() {
                abba_parser.push(c);
                if abba_parser.is_abba() {
                    return true;
                }
            }
        }
    }

    false
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"([[:alpha:]]*)(\[[[:alpha:]]*\])?").unwrap();
}

fn is_tsl(address: &str) -> bool {
    let mut tsl = false;
    for caps in RE.captures_iter(address) {
        tsl |= is_abba(caps.get(1).map(|m| m.as_str()));
        if is_abba(caps.get(2).map(|m| m.as_str())) {
            return false;
        }
    }
    tsl
}

enum ABAState {
    Empty,
    OneChar(char),
    TwoChars(char, char),
    Match(char, char),
}

impl ABAState {
    fn new() -> Self {
        ABAState::Empty
    }
    fn push(&mut self, c: char) {
        use ABAState::*;
        *self = match *self {
            Empty => OneChar(c),
            OneChar(a) => {
                if a != c {
                    TwoChars(a, c)
                } else {
                    OneChar(c)
                }
            }
            TwoChars(a, b) => {
                if c == a {
                    Match(a, b)
                } else if c == b {
                    OneChar(c)
                } else {
                    TwoChars(b, c)
                }
            }
            Match(a, b) => {
                if c == b {
                    Match(b, a)
                } else if c == a {
                    OneChar(c)
                } else {
                    TwoChars(a, c)
                }
            }
        }
    }
    fn to_bab(&self) -> Option<String> {
        if let ABAState::Match(a, b) = *self {
            let mut bab = String::new();
            bab.push(b);
            bab.push(a);
            bab.push(b);
            Some(bab)
        } else {
            None
        }
    }
}

struct ABAParser<'a> {
    chars: Chars<'a>,
    state: ABAState,
}

impl<'a> ABAParser<'a> {
    fn iter(part: &'a str) -> Self {
        ABAParser{chars: part.chars(), state: ABAState::new() }
    }
    fn push(&mut self, c: char) -> Option<String> {
        self.state.push(c);
        self.state.to_bab()
    }
}

impl<'a> Iterator for ABAParser<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(obab) = self.chars.next().map(|c| self.push(c)) {
            if obab.is_some() {
                return obab;
            }
        }
        None
    }
}

fn is_ssl(address: &str) -> bool {
    let mut babs = Vec::new();
    let mut hyperseqs = Vec::new();

    for caps in RE.captures_iter(address) {
        if let Some(seq) = caps.get(1).map(|m| m.as_str()) {
            babs.extend(ABAParser::iter(seq));
        }
        if let Some(hyperseq) = caps.get(2).map(|m| m.as_str()) {
            hyperseqs.push(hyperseq)
        }
    }

    for bab in babs {
        if hyperseqs.iter().any(|&hs| hs.contains(&bab)) {
            return true;
        }
    }
    false
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let tsl_count = buffer.lines().filter(|s| is_tsl(s)).count();
    let ssl_count = buffer.lines().filter(|s| is_ssl(s)).count();

    println!("TSL supporting IP addresses: {}", tsl_count);
    println!("SSL supporting IP addresses: {}", ssl_count);
}

#[test]
fn basic_test() {
    assert!(is_abba(Some("abba")));
    assert!(!is_abba(Some("mnop")));
    assert!(!is_abba(Some("qrst")));

    assert!(!is_abba(Some("aaaa")));
    assert!(is_abba(Some("bddb")));
    assert!(is_abba(Some("xyyx")));

    assert!(is_abba(Some("ioxxoj")));
}
