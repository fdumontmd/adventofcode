use std::str::Chars;

static INPUT: &str = include_str!("input.txt");

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
    let mut v: Vec<(i64, i64)> = DeliveryIter::new(INPUT.trim()).collect();

    v.sort();
    v.dedup();
    println!("Houses visited by single Santa: {}", v.len());

    let mut santa = String::new();
    let mut robo_santa = String::new();

    {
        let s1 = &mut santa;
        let s2 = &mut robo_santa;

        for c in INPUT.trim().chars() {
            s1.push(c);
            std::mem::swap(s1, s2);
        }
    }

    let mut v1: Vec<(i64, i64)> = DeliveryIter::new(&santa).collect();
    let mut v2: Vec<(i64, i64)> = DeliveryIter::new(&robo_santa).collect();

    v1.append(&mut v2);

    v1.sort();
    v1.dedup();

    println!("Both Santa and RoboSanta: {}", v1.len());
}

#[cfg(test)]
mod tests {

}
