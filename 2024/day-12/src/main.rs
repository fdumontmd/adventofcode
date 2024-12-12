use std::collections::{HashMap, HashSet};

use aoc_utils::{
    grid::{Grid, Taxicab},
    union_find::UnionFind,
};

const INPUT: &str = include_str!("input.txt");

// parse into a grid; init UF for each plot;
// iterate over each element,
// comparing right and bottom neighbours.
// if identical, merge in UF
fn part1(input: &str) -> usize {
    let grid: Grid<char, Taxicab> = Grid::try_from(input).unwrap();

    // should really be just one call
    let mut regions = UnionFind::new();
    regions.ensure_capacity(grid.width() * grid.height() - 1);

    for y in 0..grid.height() - 1 {
        for x in 0..grid.width() - 1 {
            if grid[(x, y)] == grid[(x + 1, y)] {
                regions.join(grid.pos_to_idx((x, y)), grid.pos_to_idx((x + 1, y)));
            }
            if grid[(x, y)] == grid[(x, y + 1)] {
                regions.join(grid.pos_to_idx((x, y)), grid.pos_to_idx((x, y + 1)));
            }
        }
    }

    for x in 0..grid.width() - 1 {
        if grid[(x, grid.height() - 1)] == grid[(x + 1, grid.height() - 1)] {
            regions.join(
                grid.pos_to_idx((x, grid.height() - 1)),
                grid.pos_to_idx((x + 1, grid.height() - 1)),
            );
        }
    }

    for y in 0..grid.height() - 1 {
        if grid[(grid.width() - 1, y)] == grid[(grid.width() - 1, y + 1)] {
            regions.join(
                grid.pos_to_idx((grid.width() - 1, y)),
                grid.pos_to_idx((grid.width() - 1, y + 1)),
            );
        }
    }

    // the regios.same_group() called in a loop is bad;
    // should scan just once building and index each pos by leader

    let mut whole_regions = HashMap::new();
    for idx in 0..regions.len() {
        whole_regions
            .entry(regions.leader(idx))
            .or_insert(vec![])
            .push(idx);
    }

    whole_regions
        .values()
        .map(|whole_region| region_price(whole_region, &grid))
        .sum()
}

fn region_price(whole_region: &Vec<usize>, grid: &Grid<char, Taxicab>) -> usize {
    let area = whole_region.len();
    area * whole_region
        .iter()
        .map(|p| {
            let pos = grid.idx_to_pos(*p);
            let plant = grid[pos];
            4 - grid.neighbours(pos).filter(|n| grid[*n] == plant).count()
        })
        .sum::<usize>()
}

// refactor the building of whole regions and just use a region fence price calculator
fn part2(input: &str) -> usize {
    let grid: Grid<char, Taxicab> = Grid::try_from(input).unwrap();

    // should really be just one call
    let mut regions = UnionFind::new();
    regions.ensure_capacity(grid.width() * grid.height() - 1);

    for y in 0..grid.height() - 1 {
        for x in 0..grid.width() - 1 {
            if grid[(x, y)] == grid[(x + 1, y)] {
                regions.join(grid.pos_to_idx((x, y)), grid.pos_to_idx((x + 1, y)));
            }
            if grid[(x, y)] == grid[(x, y + 1)] {
                regions.join(grid.pos_to_idx((x, y)), grid.pos_to_idx((x, y + 1)));
            }
        }
    }

    for x in 0..grid.width() - 1 {
        if grid[(x, grid.height() - 1)] == grid[(x + 1, grid.height() - 1)] {
            regions.join(
                grid.pos_to_idx((x, grid.height() - 1)),
                grid.pos_to_idx((x + 1, grid.height() - 1)),
            );
        }
    }

    for y in 0..grid.height() - 1 {
        if grid[(grid.width() - 1, y)] == grid[(grid.width() - 1, y + 1)] {
            regions.join(
                grid.pos_to_idx((grid.width() - 1, y)),
                grid.pos_to_idx((grid.width() - 1, y + 1)),
            );
        }
    }

    // the regios.same_group() called in a loop is bad;
    // should scan just once building and index each pos by leader

    let mut whole_regions = HashMap::new();
    for idx in 0..regions.len() {
        whole_regions
            .entry(regions.leader(idx))
            .or_insert(vec![])
            .push(idx);
    }

    whole_regions
        .values()
        .map(|whole_region| discounted_region_price(whole_region, &grid))
        .sum()
}

// ugly... but it works and is still fast
fn discounted_region_price(whole_region: &[usize], grid: &Grid<char, Taxicab>) -> usize {
    let area = whole_region.len();
    // keep track of each plot facing outside in a certain direction
    // (up, down, left, right); then for each direction, for each
    // plot if the next neighbour is also facing out in that direction
    // decrease the total perimeter by 1
    let mut up_facing = HashSet::new();
    let mut down_facing = HashSet::new();
    let mut left_facing = HashSet::new();
    let mut right_facing = HashSet::new();

    let mut perimeter = whole_region
        .iter()
        .map(|p| {
            let pos = grid.idx_to_pos(*p);
            let plant = grid[pos];
            let mut fences = 4;
            let mut up = true;
            let mut down = true;
            let mut left = true;
            let mut right = true;
            for n in grid.neighbours(pos) {
                if grid[n] == plant {
                    fences -= 1;
                    if n.0 == pos.0 {
                        if n.1 > pos.1 {
                            down = false;
                        } else {
                            up = false;
                        }
                    } else if n.0 > pos.0 {
                        right = false;
                    } else {
                        left = false;
                    }
                }
            }
            if up {
                up_facing.insert(pos);
            }
            if down {
                down_facing.insert(pos);
            }
            if left {
                left_facing.insert(pos);
            }
            if right {
                right_facing.insert(pos);
            }
            fences
        })
        .sum::<usize>();

    // now for each direction, we check whether the next (right or down) neighbour is also facing
    // out in the same direction, and if so, reduce perimeter by 1
    for pos in &up_facing {
        if pos.0 < grid.width() - 1 && up_facing.contains(&(pos.0 + 1, pos.1)) {
            perimeter -= 1;
        }
    }

    for pos in &down_facing {
        if pos.0 < grid.width() - 1 && down_facing.contains(&(pos.0 + 1, pos.1)) {
            perimeter -= 1;
        }
    }

    for pos in &left_facing {
        if pos.1 < grid.height() - 1 && left_facing.contains(&(pos.0, pos.1 + 1)) {
            perimeter -= 1;
        }
    }

    for pos in &right_facing {
        if pos.1 < grid.height() - 1 && right_facing.contains(&(pos.0, pos.1 + 1)) {
            perimeter -= 1;
        }
    }

    area * perimeter
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_INPUT_SMALL: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST_INPUT_LARGE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test_case(TEST_INPUT_SMALL, 772; "small test input")]
    #[test_case(TEST_INPUT_LARGE, 1930; "large test input")]
    #[test_case(INPUT, 1457298; "input")]
    fn test_part1(input: &str, price: usize) {
        assert_eq!(price, part1(input));
    }

    #[test_case("AAAA
BBCD
BBCC
EEEC", 80; "tiny test input")]
    #[test_case(TEST_INPUT_SMALL, 436; "small test input")]
    #[test_case("EEEEE
EXXXX
EEEEE
EXXXX
EEEEE", 236; "e-shaped test input")]
    #[test_case("AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA", 368; "medion test input")]
    #[test_case(TEST_INPUT_LARGE, 1206; "large test input")]
    #[test_case(INPUT, 921636; "input")]
    fn test_part2(input: &str, price: usize) {
        assert_eq!(price, part2(input));
    }
}
