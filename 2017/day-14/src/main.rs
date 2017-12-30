extern crate aoc_utils;
extern crate knot_hash;

use std::fmt::Write;
use std::collections::HashMap;

use aoc_utils::union_find::UnionFind;

const INPUT: &'static str = "uugsqrei";

fn main() {
    println!("Occupied for input {} = {}", INPUT, count_occupied(INPUT));
    println!("Number of regions: {}", count_regions(INPUT));
}

fn row_to_strbytes(row: &Vec<u8>) -> String {
    assert_eq!(row.len(), 16);
    let mut buf = String::new();
    for b in row {
        write!(&mut buf, "{:08b}", b).unwrap();
    }
    assert_eq!(buf.len(), 128);
    buf
}

fn make_connected(s: &str) -> (HashMap<(usize, usize), usize>, UnionFind){
    let mut uf = UnionFind::new();
    let mut hm = HashMap::new();

    let map = make_map(s);

    for (idx, row) in map.iter().enumerate() {
        for (pos, pair) in row_to_strbytes(row).as_bytes().windows(2).enumerate() {
            if pos == 0 && pair[0] == b'1' {
                let next_id = hm.len();
                hm.insert((idx, pos), next_id);
            }
            if pair[1] == b'1' {
                let next_id = hm.len();
                hm.insert((idx, pos + 1), next_id);
            }
            if pair[0] == pair[1] && pair[0] == b'1' {
                let id_1 = *hm.get(&(idx, pos)).unwrap();
                let id_2 = *hm.get(&(idx, pos + 1)).unwrap();
                uf.join(id_1, id_2);
            }
        }
    }

    for (idx, rows) in map.windows(2).enumerate() {
        let row_1 = row_to_strbytes(&rows[0]);
        let row_2 = row_to_strbytes(&rows[1]);

        for (pos, pair) in row_1.as_bytes().iter().zip(row_2.as_bytes().iter()).enumerate() {
            if pair.0 == pair.1 && *pair.0 == b'1' {
                let id_1 = *hm.get(&(idx, pos)).unwrap();
                let id_2 = *hm.get(&(idx + 1, pos)).unwrap();
                uf.join(id_1, id_2);
            }
        }
    }

    (hm, uf)
}

fn make_map(s: &str) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    use knot_hash::Knot;

    for i in 0..128 {
        let row = format!("{}-{}", s, i);
        let knot = Knot::from_str(&row);
        vec.push(knot.dense());
    }

    vec
}

fn count_occupied(s: &str) -> u32 {
    let mut total = 0;

    for row in make_map(s) {
        for u in row {
            total += u.count_ones();
        }
    }

    total
}

fn count_regions(s: &str) -> usize {
    make_connected(s).1.groups().len()
}

#[test]
fn test_count() {
    assert_eq!(count_occupied("flqrgnkx"), 8108);
}

#[test]
fn test_regions() {
    assert_eq!(count_regions("flqrgnkx"), 1242);
}
