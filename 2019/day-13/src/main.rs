use std::collections::HashMap;
use intcode::*;

static INPUT: &str = include_str!("input.txt");

struct Game {
    score: MemItem,
    screen: HashMap<(MemItem, MemItem), MemItem>,
}

fn run_game(mut computer: Computer) -> Game {
    let mut screen = HashMap::new();
    let mut score = 0;

    let mut ball = None;
    let mut pad = None;

    while !computer.is_stopped() {
        if let Some(x) = computer.wait_until_output() {
            if let Some(y) = computer.wait_until_output() {
                if let Some(id) = computer.wait_until_output() {
                    if x == -1 && y == 0 {
                        score = id;
                    } else {
                        screen.insert((x, y), id);
                        if id == 3 {
                            pad = Some(x);
                        } else if id == 4 {
                            ball = Some(x);
                        }

                        if let Some(p) = pad {
                            if let Some(b) = ball {
                                computer.add_input((b-p).signum());
                                ball = None;
                            }
                        }
                    }
                }
            }
        }
    }

    Game {
        screen,
        score,
    }
}

fn display(game: &Game) {
    let screen = &game.screen;
    let min_x = screen.keys().map(|k| k.0).min().unwrap();
    let min_y = screen.keys().map(|k| k.1).min().unwrap();
    let max_x = screen.keys().map(|k| k.0).max().unwrap();
    let max_y = screen.keys().map(|k| k.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let tile = screen.get(&(x, y)).unwrap_or(&0);
            let c= match tile {
                0 => ' ',
                1 => '#',
                2 => '=',
                3 => '_',
                4 => 'o',
                _ => unreachable!(),
            };

            print!("{}", c);
        }
        println!();
    }
}

fn count_bricks(desc: &str) -> usize {
    let computer: Computer = desc.parse().unwrap();
    run_game(computer).screen.iter().filter(|(_, id)| **id == 2).count()
}

fn part_1() -> usize {
    count_bricks(INPUT)
}

fn part_2() -> MemItem {
    let mut computer: Computer = INPUT.parse().unwrap();
    computer.set_at(0, 2);
    let game = run_game(computer);
    display(&game);
    game.score
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[cfg(test)]
mod test {
}
