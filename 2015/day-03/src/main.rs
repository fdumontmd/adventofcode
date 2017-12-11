use std::io::Read;
use std::str::Chars;

struct DeliveryIter<'a> {
    c: Chars<'a>,
    x: i64,
    y: i64,
    start: bool,
}

impl<'a> DeliveryIter<'a> {
    fn new(s: &'a str) -> DeliveryIter<'a> {
        DeliveryIter {
            c: s.chars(),
            x: 0,
            y: 0,
            start: true,
        }
    }
}

impl<'a> Iterator for DeliveryIter<'a> {
    type Item = (i64, i64);
    fn next(&mut self) -> Option<(i64, i64)> {
        if self.start {
            self.start = false;
            return Some((0,0));
        }
        if let Some(c) = self.c.next() {
            match c {
                '<' => {
                    self.x -= 1;
                }
                '>' => {
                    self.x += 1;
                }
                '^' => {
                    self.y -= 1;
                }
                'v' => {
                    self.y += 1;
                }
                _ => unreachable!(),
            }
            Some((self.x, self.y))
        } else {
            None
        }
    }
}


fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);

    let mut v: Vec<(i64, i64)> = DeliveryIter::new(&input.trim()).collect();

    v.sort();
    v.dedup();
    println!("Houses visited by single Santa: {}", v.len());

    let mut santa = String::new();
    let mut roboSanta = String::new();

    {
        let s1 = &mut santa;
        let s2 = &mut roboSanta;

        for c in input.trim().chars() {
            s1.push(c);
            std::mem::swap(s1, s2);
        }
    }

    let mut v1: Vec<(i64, i64)> = DeliveryIter::new(&santa).collect();
    let mut v2: Vec<(i64, i64)> = DeliveryIter::new(&roboSanta).collect();

    v1.append(&mut v2);

    v1.sort();
    v1.dedup();

    println!("Both Santa and RoboSanta: {}", v1.len());
}

#[cfg(test)]
mod tests {

}
