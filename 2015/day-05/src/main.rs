use std::io::Read;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Nice,
    Naughty,
}

pub trait Processor {
    fn accept(&mut self, c: char) -> Option<Kind>;
    fn reset(&mut self);
}

#[derive(Clone, Copy, Debug)]
pub struct VoyelCount {
    count: usize,
    kind: Option<Kind>,
}

impl VoyelCount {
    fn new() -> VoyelCount {
        VoyelCount { count: 0, kind: None, }
    }
}

impl Processor for VoyelCount {
    fn accept(&mut self, c: char) -> Option<Kind> {
        if let Some(_) = self.kind {
            return self.kind;
        }
        match c {
            'a'|'e'|'i'|'u'|'o' => {
                self.count += 1;
            }
            _ => {}
        }
        if self.count >= 3 {
            self.kind = Some(Kind::Nice);
        }
        self.kind
    }

    fn reset(&mut self) {
        self.count = 0;
        self.kind = None;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Double {
    previous: Option<char>,
    kind: Option<Kind>,
}

impl Double {
    fn new() -> Double {
        Double { previous: None, kind: None }
    }
}

impl Processor for Double {
    fn accept(&mut self, c: char) -> Option<Kind> {
        if let Some(_) = self.kind {
            return self.kind;
        }
        if let Some(p) = self.previous {
            if p == c {
                self.kind = Some(Kind::Nice);
            }
        }
        self.previous = Some(c);
        self.kind
    }

    fn reset(&mut self) {
        self.previous = None;
        self.kind = None;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PairState {
    Start,
    FirstChar,
    Accepted,
}

#[derive(Clone, Copy, Debug)]
pub struct PairMatcher {
    state: PairState,
    c1: char,
    c2: char,
    kind: Kind,
}

impl PairMatcher {
    fn new(c1: char, c2: char) -> PairMatcher {
        PairMatcher {
            state: PairState::Start,
            c1: c1,
            c2: c2,
            kind: Kind::Nice,
        }
    }
}

impl Processor for PairMatcher {
    fn accept(&mut self, c: char) -> Option<Kind> {
        match self.state {
            PairState::Start => {
                if c == self.c1 {
                    self.state = PairState::FirstChar;
                }
            }
            PairState::FirstChar => {
                if c == self.c2 {
                    self.state = PairState::Accepted;
                } else if c == self.c1 {
                    self.state = PairState::FirstChar;
                } else {
                    self.state = PairState::Start;
                }
            }
            _ => {}
        }

        if self.state == PairState::Accepted {
            self.kind = Kind::Naughty;
        }
        Some(self.kind)
    }

    fn reset(&mut self) {
        self.state = PairState::Start;
        self.kind = Kind::Nice;
    }
}


pub struct CompoundProcessor<'a> {
    processors: Vec<&'a mut Processor>,
}

impl<'a> CompoundProcessor<'a> {
    fn new() -> CompoundProcessor<'a> {
        CompoundProcessor {
            processors: Vec::new(),
        }
    }

    fn add_processor(&mut self, p: &'a mut Processor) {
        self.processors.push(p);
    }
}

impl<'a> Processor for CompoundProcessor<'a> {
    fn accept(&mut self, c: char) -> Option<Kind> {
        // iterate over each processor; they must all give an answer
        // for the compound processor to return one
        let mut kind = None;
        let mut any_none = false;
        for p in &mut self.processors {
            if let Some(k) = p.accept(c) {
                if kind != Some(Kind::Naughty) {
                    kind = Some(k);
                }
            } else {
                any_none = true;
            }
        }

        if any_none && kind == Some(Kind::Nice) {
            kind = None;
        }

        kind
    }

    fn reset(&mut self) {
        for p in &mut self.processors {
            p.reset();
        }
    }
}

pub fn check<P: Processor>(p: &mut P, s: &str) -> Kind {
    let mut nice = false;
    for c in s.chars() {
        if let Some(k) = p.accept(c) {
            if k == Kind::Nice {
                nice = true;
            } else {
                return Kind::Naughty;
            }
        }
    }

    if nice {
        return Kind::Nice;
    } else {
        return Kind::Naughty;
    }
}

pub fn standard_check(s: &str) -> Kind {
    let mut v = VoyelCount::new();
    let mut d = Double::new();
    let mut ab = PairMatcher::new('a', 'b');
    let mut cd = PairMatcher::new('c', 'd');
    let mut pq = PairMatcher::new('p', 'q');
    let mut xy = PairMatcher::new('x', 'y');
    let mut cp = CompoundProcessor::new();

    cp.add_processor(&mut v);
    cp.add_processor(&mut d);
    cp.add_processor(&mut ab);
    cp.add_processor(&mut cd);
    cp.add_processor(&mut pq);
    cp.add_processor(&mut xy);

    check(&mut cp, s)
}

fn main() {
    let mut input = String::new();
    let mut nice = 0;

    std::io::stdin().read_to_string(&mut input).unwrap();

    for n in input.split('\n') {
        if !n.trim().is_empty() {
            let k = standard_check(&n);
            println!("{} is {:?}", n, k);
            if k == Kind::Nice {
                nice += 1;
            }
        }
    }

    println!("Nice count: {}", nice);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_voyels() {
        assert_eq!(check(&mut VoyelCount::new(), "aei"), Kind::Nice);
        assert_eq!(check(&mut VoyelCount::new(), "xazegov"), Kind::Nice);
        assert_eq!(check(&mut VoyelCount::new(), "aeiouaeiouaeiou"), Kind::Nice);
    }

    #[test]
    fn voyel_and_double() {
        let mut v = VoyelCount::new();
        let mut d = Double::new();
        let mut cp = CompoundProcessor::new();
        cp.add_processor(&mut v);
        cp.add_processor(&mut d);
        assert_eq!(check(&mut cp, "aaa"), Kind::Nice);

        cp.reset();

        assert_eq!(check(&mut cp, "jchzalrnumimnmhp"), Kind::Naughty);

        cp.reset();

        assert_eq!(check(&mut cp, "dvszwmarrgswjxmb"), Kind::Naughty);
    }

    #[test]
    fn pair_check() {
        let mut v = VoyelCount::new();
        let mut d = Double::new();
        let mut xy = PairMatcher::new('x', 'y');
        let mut cp = CompoundProcessor::new();

        cp.add_processor(&mut v);
        cp.add_processor(&mut d);
        cp.add_processor(&mut xy);

        assert_eq!(check(&mut cp, "haegwjzuvuyypxyu"), Kind::Naughty);
    }
}
