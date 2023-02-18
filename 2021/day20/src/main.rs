// ideas:
// infinite pixels means we need to know how to handle
// all off and all on and alternate as default value
// between iterations
// check 0th and last entry in the imange enhancement algorithm
// if 0 is on, and last is off, we alternate between default
// on and default off
// if 0 is off, default is off, forever, so no need to consider
// (we need to consider)
//
// next idea: each image just increase size by 2 in each
// iteration; index by (isize, isize) and use default for indices
// outside the ranges

use std::{
    array::from_fn,
    fmt::Display,
    ops::{Index, IndexMut},
};

static INPUT: &str = include_str!("input.txt");

struct Image {
    pixels: Vec<bool>,
    width: usize,
    height: usize,
    // only applies to pixels outside the frame
    default: bool,
}

impl Image {
    fn new(width: usize, height: usize, default: bool) -> Self {
        let len = width * height;
        Self {
            pixels: vec![false; len],
            width,
            height,
            default,
        }
    }

    fn parse(input: &str) -> Self {
        let mut width = 0;
        let pixels: Vec<bool> = input
            .bytes()
            .enumerate()
            .filter_map(|(idx, b)| {
                if b == 10 {
                    if width == 0 {
                        width = idx;
                    }
                    None
                } else {
                    Some(b == b'#')
                }
            })
            .collect();
        let height = pixels.len() / width;
        Self {
            pixels,
            width,
            height,
            default: false,
        }
    }

    fn enhance(&self, iea: &[bool; 512]) -> Self {
        let nwidth = self.width + 2;
        let nheight = self.height + 2;
        let default = if self.default { iea[511] } else { iea[0] };

        let mut new_image = Image::new(nwidth, nheight, default);

        for y in -1..(self.height + 1) as isize {
            for x in -1..(self.width + 1) as isize {
                let mut n = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        n <<= 1;
                        n += self[(x + dx, y + dy)] as usize;
                    }
                    new_image[(x + 1, y + 1)] = iea[n];
                }
            }
        }

        new_image
    }

    fn pixel_count(&self) -> usize {
        self.pixels.iter().filter(|b| **b).count()
    }
}

impl Index<(isize, isize)> for Image {
    type Output = bool;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        if (0..self.width as isize).contains(&index.0)
            && (0..self.height as isize).contains(&index.1)
        {
            &self.pixels[index.0 as usize + (index.1 as usize) * self.width]
        } else {
            &self.default
        }
    }
}

impl IndexMut<(isize, isize)> for Image {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        if (0..self.width as isize).contains(&index.0)
            && (0..self.height as isize).contains(&index.1)
        {
            &mut self.pixels[index.0 as usize + (index.1 as usize) * self.width]
        } else {
            panic!("{:?} is out of bounds", index)
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, b) in self.pixels.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            let c = if *b { '#' } else { '.' };
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

fn parse_problem(input: &str) -> ([bool; 512], Image) {
    let parts: Vec<_> = input.split("\n\n").collect();
    assert_eq!(512, parts[0].len());
    let iea: [bool; 512] = from_fn(|idx| parts[0].as_bytes()[idx] == b'#');

    let image = Image::parse(parts[1]);
    (iea, image)
}

fn part_1(input: &str) -> usize {
    let (iea, image) = parse_problem(input);

    let image = image.enhance(&iea);
    let image = image.enhance(&iea);

    image.pixel_count()
}

fn part_2(input: &str) -> usize {
    let (iea, mut image) = parse_problem(input);
    for _ in 0..50 {
        image = image.enhance(&iea);
    }
    image.pixel_count()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part_1() {
        assert_eq!(35, part_1(TEST_INPUT));
        assert_eq!(5479, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3351, part_2(TEST_INPUT));
        assert_eq!(19012, part_2(INPUT));
    }
}
