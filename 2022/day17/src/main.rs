use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");
const WIDTH: usize = 7;

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

struct Shape {
    width: usize,
    height: usize,
    shape: Vec<(usize, usize)>,
}

impl Shape {
    // bug: is_blocked can be down, but also left or right
    // meaning we need to keep track of not just heights but also blocks
    fn is_blocked(&self, pos: (usize, usize), blocks: &HashSet<(usize, usize)>) -> bool {
        for block in &self.shape {
            let block = (block.0 + pos.0, pos.1 - block.1);
            if blocks.contains(&block) {
                return true;
            }

            // that's the floor
            if block.1 == 0 {
                return true;
            }
        }

        false
    }

    fn settle(
        &self,
        pos: (usize, usize),
        blocks: &mut HashSet<(usize, usize)>,
        mut max_height: usize,
    ) -> usize {
        for block in &self.shape {
            let block = (block.0 + pos.0, pos.1 - block.1);
            blocks.insert(block);
            max_height = max_height.max(block.1);
        }
        max_height
    }
}

fn get_shapes() -> Vec<Shape> {
    vec![
        Shape {
            width: 4,
            height: 1,
            shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        Shape {
            width: 3,
            height: 3,
            shape: vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
        },
        Shape {
            width: 3,
            height: 3,
            shape: vec![(0, 2), (1, 2), (2, 2), (2, 1), (2, 0)],
        },
        Shape {
            width: 1,
            height: 4,
            shape: vec![(0, 3), (0, 2), (0, 1), (0, 0)],
        },
        Shape {
            width: 2,
            height: 2,
            shape: vec![(0, 1), (1, 1), (0, 0), (1, 0)],
        },
    ]
}

fn parse(input: &str) -> Vec<Jet> {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Jet::Left),
            '>' => Some(Jet::Right),
            _ => None,
        })
        .collect()
}

fn dump(max_height: usize, blocks: &HashSet<(usize, usize)>) {
    for row in (1..max_height + 1).rev() {
        print!("|");
        for col in 0..WIDTH {
            if blocks.contains(&(col, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
}

fn drop_blocks(input: &str, max_blocks: usize) -> usize {
    let mut blocks = HashSet::new();

    let jets = parse(input);
    let shapes = get_shapes();

    let mut max_height = 0;

    let mut prev_max_height = 0;
    let mut height_delta = 0;

    // ok, let's document the thinking here before I forget
    // the basic idea is to look for stable increments over
    // long enough strides. The first stride to look at is
    // over individual jets, to see whether the number of
    // rock dropped becomes stable. The stride len was
    // chosen as jets.len() * shapes.len() on the assumption
    // that this would be compatible with repeated patterns.
    // Once we know how many rocks drop over this stride,
    // we check whether there is a stable increase in height
    // over that many rocks dropped.
    //
    // the check is rather crude and will pretend the increase
    // is stable when it matches the previous measure...
    // presumably we could be a bit more thorough and keep the
    // N previous values to confirm we have a stable increase,
    // but 1 here seems to work for test and real data
    //
    // ugly, but fast
    let round_len = jets.len() * shapes.len();

    let round_modulo = max_blocks % round_len;
    let mut prev_block = 0;
    let mut block_delta = 0;

    let mut shortcut = false;
    let mut block_start = 0;

    let mut jet = 0;
    for rock in 0..max_blocks {
        let shape = &shapes[rock % shapes.len()];
        let mut pos: (usize, usize) = (2, max_height + 3 + shape.height);
        if shortcut && (rock - block_start) % block_delta == 0 {
            if height_delta == max_height - prev_max_height {
                return max_height + height_delta * ((max_blocks - rock) / block_delta);
            }
            height_delta = max_height - prev_max_height;
            prev_max_height = max_height;
        }
        loop {
            if jet > round_modulo && (jet - round_modulo) % round_len == 0 {
                if block_delta == rock - prev_block {
                    shortcut = true;
                    block_start = max_blocks % block_delta;
                }
                block_delta = rock - prev_block;
                prev_block = rock;
            }

            // push
            let new_pos = match jets[jet % jets.len()] {
                Jet::Left => (pos.0.saturating_sub(1), pos.1),
                Jet::Right => {
                    if pos.0 + shape.width < WIDTH {
                        (pos.0 + 1, pos.1)
                    } else {
                        pos
                    }
                }
            };

            if !shape.is_blocked(new_pos, &blocks) {
                pos = new_pos;
            }
            jet += 1;

            let new_pos = (pos.0, pos.1 - 1);

            if shape.is_blocked(new_pos, &blocks) {
                max_height = shape.settle(pos, &mut blocks, max_height);
                break;
            }

            pos = new_pos;
        }
    }

    max_height
}

fn part_01(input: &str) -> usize {
    drop_blocks(input, 2022)
}

fn part_02(input: &str) -> usize {
    drop_blocks(input, 1000000000000)
    //drop_blocks(input, 1000000)
}

fn main() {
    println!("Part 1: {}", part_01(INPUT));
    println!("Part 2: {}", part_02(INPUT));
}

#[cfg(test)]
mod test {
    use crate::{part_01, part_02, INPUT};

    static TEST_INPUT: &str = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part_01() {
        assert_eq!(3068, part_01(TEST_INPUT));
        //assert!(false);
    }

    #[test]
    fn test_part_02() {
        assert_eq!(1514285714288, part_02(TEST_INPUT));
    }

    #[test]
    fn real_part_01() {
        assert_eq!(3224, part_01(INPUT));
        //assert!(false);
    }

    #[test]
    fn real_part_02() {
        assert_eq!(1595988538691, part_02(INPUT));
    }
}
