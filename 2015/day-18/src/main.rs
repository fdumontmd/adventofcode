use std::io::{self, Read};
use std::fmt;

struct Lights {
    lights: Vec<Vec<u8>>,
}

impl Lights {
    fn from_str(input: &str) -> Self {
        let mut lights = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();
            row.push(0);
            for c in line.chars() {
                match c {
                    '#' => row.push(1),
                    '.' => row.push(0),
                    _ => (),
                }
            }
            row.push(0);
            if lights.is_empty() {
                lights.push(vec![0; row.len()]);
            }
            lights.push(row);
        }

        let len = lights[0].len();
        lights.push(vec![0; len]);

        assert!(lights.iter().all(|r| r.len() == len),
                "not all rows have the same length");

        Lights { lights: lights }
    }

    fn next_state(&self) -> Self {
        let line_len = self.lights[0].len() - 2;

        let mut accum: Vec<Vec<u8>> = Vec::new();
        for _ in 0..line_len {
            accum.push(Vec::new());
        }

        for row in &self.lights {
            for (col, chunk) in row.as_slice().windows(3).enumerate() {
                accum[col].push(chunk.iter().sum());
            }
        }

        let mut lights = Vec::new();

        lights.push(vec![0; line_len + 2]);
        for _ in 0..self.lights.len() - 2 {
            lights.push(vec![0; 1]);
        }
        lights.push(vec![0; line_len + 2]);

        for (col, count) in accum.into_iter().enumerate() {
            for (row, chunk) in count.as_slice().windows(3).enumerate() {
                let total: u8 = chunk.iter().sum();

                let on = if self.lights[row + 1][col + 1] == 1 {
                    total == 3 || total == 4
                } else {
                    total == 3
                };
                if on {
                    lights[row + 1].push(1);
                } else {
                    lights[row + 1].push(0);
                }
            }
        }

        for row in 1..lights.len() - 1 {
            lights[row].push(0);
        }

        assert_eq!(self.lights.len(), lights.len());

        Lights { lights: lights }
    }

    fn count_on(&self) -> usize {
        let mut total = 0;
        for row in &self.lights {
            total += row.iter().map(|u| *u as usize).sum();
        }
        total
    }

    fn force_on(&mut self) {
        let line_len = self.lights[0].len();
        let rows = self.lights.len();
        self.lights[1][1] = 1;
        self.lights[1][line_len - 2] = 1;
        self.lights[rows - 2][1] = 1;
        self.lights[rows - 2][line_len - 2] = 1;
    }
}

impl fmt::Display for Lights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.lights.as_slice()[1..self.lights.len() - 1] {
            for t in &row.as_slice()[1..row.len() - 1] {
                if *t == 0 {
                    try!(write!(f, "."));
                } else {
                    try!(write!(f, "#"));
                }
            }
            try!(write!(f, "\n"));
        }
        Ok(())
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();

    let mut lights = Lights::from_str(&buffer);

    for _ in 0..100 {
        lights = lights.next_state();
    }

    println!("Count of lights after 100 steps: {}", lights.count_on());

    let mut lights = Lights::from_str(&buffer);
    lights.force_on();

    for _ in 0..100 {
        lights = lights.next_state();
        lights.force_on();
    }

    println!("Count of lights after 100 steps: {}", lights.count_on());
}

#[cfg(test)]
mod test {
    use std::fmt::Write;
    use super::Lights;
    #[test]
    fn test_parsing() {
        let input = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

        let lights = Lights::from_str(input);

        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(input, out.trim());

    }

    #[test]
    fn test_next_state() {
        let input = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

        let second_step = r"..##..
..##.#
...##.
......
#.....
#.##..";

        let third_step = r"..###.
......
..###.
......
.#....
.#....";

        let fourth_step = r"...#..
......
...#..
..##..
......
......";

        let fifth_step = r"......
......
..##..
..##..
......
......";

        let lights = Lights::from_str(input);

        let lights = lights.next_state();

        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(second_step, out.trim());

        let lights = lights.next_state();

        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(third_step, out.trim());

        let lights = lights.next_state();

        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(fourth_step, out.trim());

        let lights = lights.next_state();

        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(fifth_step, out.trim());
    }

    #[test]
    fn test_force_on() {
        let input = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

        let second_step = r"#.##.#
####.#
...##.
......
#...#.
#.####";

        let third_step = r"#..#.#
#....#
.#.##.
...##.
.#..##
##.###";

        let fourth_step = r"#...##
####.#
..##.#
......
##....
####.#";

        let fifth_step = r"#.####
#....#
...#..
.##...
#.....
#.#..#";

        let sixth_step = r"##.###
.##..#
.##...
.##...
#.#...
##...#";

        let mut lights = Lights::from_str(input);
        lights.force_on();

        let mut lights = lights.next_state();
        lights.force_on();
        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(second_step, out.trim());

        let mut lights = lights.next_state();
        lights.force_on();
        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(third_step, out.trim());

        let mut lights = lights.next_state();
        lights.force_on();
        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(fourth_step, out.trim());

        let mut lights = lights.next_state();
        lights.force_on();
        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(fifth_step, out.trim());

        let mut lights = lights.next_state();
        lights.force_on();
        let mut out = String::new();
        write!(&mut out, "{}", lights).unwrap();

        assert_eq!(sixth_step, out.trim());
    }
}
