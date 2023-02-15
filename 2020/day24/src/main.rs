use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");
// use cube coordinates from https://www.redblobgames.com/grids/hexagons/
// pointy-top orientation

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

// x 0 is nw-se axis
// y 0 is sw-ne axis
// z 0 is e-w axis
impl Direction {
    fn delta(&self) -> (isize, isize, isize) {
        match self {
            Direction::East => (1, -1, 0),
            Direction::SouthEast => (0, -1, 1),
            Direction::SouthWest => (-1, 0, 1),
            Direction::West => (-1, 1, 0),
            Direction::NorthWest => (0, 1, -1),
            Direction::NorthEast => (1, 0, -1),
        }
    }

    fn from_str(input: &str) -> Self {
        match input {
            "e" => Direction::East,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            "w" => Direction::West,
            "nw" => Direction::NorthWest,
            "ne" => Direction::NorthEast,
            _ => panic!("unknown direction {input}"),
        }
    }

    fn neighbours(&self, coord: (isize, isize, isize)) -> (isize, isize, isize) {
        let d = self.delta();
        (coord.0 + d.0, coord.1 + d.1, coord.2 + d.2)
    }

    fn all_neighbours(coord: (isize, isize, isize)) -> impl Iterator<Item = (isize, isize, isize)> {
        use Direction::*;
        [East, SouthEast, SouthWest, West, NorthWest, NorthEast]
            .into_iter()
            .map(move |d| d.neighbours(coord))
    }
}

fn parse_directions(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input
        .split_inclusive(|c| c == 'e' || c == 'w')
        .map(Direction::from_str)
}

fn compute_pos(input: &str) -> (isize, isize, isize) {
    let mut tile = (0, 0, 0);
    for d in parse_directions(input) {
        tile = d.neighbours(tile);
    }
    tile
}

fn tile_floor(input: &str) -> HashSet<(isize, isize, isize)> {
    let mut tiles = HashSet::new();

    for line in input.lines() {
        let pos = compute_pos(line);
        if tiles.contains(&pos) {
            tiles.remove(&pos);
        } else {
            tiles.insert(pos);
        }
    }
    tiles
}

fn part_1(input: &str) -> usize {
    tile_floor(input).len()
}

fn part_2(input: &str) -> usize {
    let mut tiles = tile_floor(input);
    // basic idea: compute neighbours in all 6 directions (add that to Direction)
    // collect in a Set, then iterate over that set:
    // compute neighbours that are in tiles, and apply rules to flip or not

    for _d in 0..100 {
        let mut all_neighbours = HashSet::new();
        for c in &tiles {
            all_neighbours.extend(Direction::all_neighbours(*c));
        }

        let mut tmp = HashSet::new();

        for c in all_neighbours {
            let count = tiles
                .intersection(&Direction::all_neighbours(c).collect())
                .count();

            if tiles.contains(&c) {
                if count == 1 || count == 2 {
                    tmp.insert(c);
                }
            } else if count == 2 {
                tmp.insert(c);
            }
        }

        tiles = tmp;
        // println!("Day {}: {}", _d + 1, tiles.len());
    }

    tiles.len()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, INPUT};

    static TEST_INPUT: &str = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part_1() {
        assert_eq!(10, part_1(TEST_INPUT));
        assert_eq!(497, part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(2208, part_2(TEST_INPUT));
        assert_eq!(4156, part_2(INPUT));
    }
}
