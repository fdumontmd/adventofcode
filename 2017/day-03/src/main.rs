fn main() {
    let input = 265149;

    println!("{}", manhattan(to_pos(input)));

    let mut store = std::collections::HashMap::new();

    store.insert(to_pos(1), 1);

    for p in 2.. {
        let pos = to_pos(p);

        let mut to_store = 0;

        for n in pos.neighbours() {
            to_store += *store.get(&n).unwrap_or(&0);
        }

        if to_store > input {
            println!("{:?} <- {}", pos, to_store);
            break;
        }

        store.insert(pos, to_store);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Position(i32, i32);

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        let mut results = Vec::new();

        for i in -1..2 {
            for j in -1..2 {
                if i != 0 || j != 0 {
                    results.push(Position(self.0 + i, self.1 + j));
                }
            }
        }
        results
    }
}

fn manhattan(p: Position) -> u32 {
    p.0.abs() as u32 + p.1.abs() as u32
}

fn to_pos(i: i32) -> Position {
    if i == 1 {
        return Position(0, 0);
    }
    let mut base = ((i - 1)  as f64).sqrt() as i32;
    if base % 2 == 0 {
        base -= 1;
    }

    let corner = base * base;
    let side = base + 1;
    let mut start = corner;

    // right side
    if start < i && i <= start + side {
        return Position(side / 2, start - i + side / 2);
    }
    start += side;

    // top
    if start < i && i <= start + side {
        return Position(start - i + (side / 2), - side / 2);
    }
    start += side;

    // left
    if start < i && i <= start + side {
        return Position(- side / 2, i - start - side / 2);
    }
    start += side;

    // bottom
    return Position(i - start - (side / 2) , side / 2);
}

#[test]
fn test_1() {
    assert_eq!(to_pos(1), Position(0, 0));
    assert_eq!(manhattan(to_pos(1)), 0);
}

#[test]
fn test_12() {
    assert_eq!(to_pos(12), Position(2, -1));
    assert_eq!(manhattan(to_pos(12)), 3);
}

#[test]
fn test_23() {
    assert_eq!(to_pos(23), Position(0, 2));
    assert_eq!(manhattan(to_pos(23)), 2);
}

#[test]
fn test_1024() {
    assert_eq!(manhattan(to_pos(1024)), 31);
}

#[test]
fn test() {
    // should test each side
    assert_eq!(to_pos(1), Position(0, 0));
    assert_eq!(to_pos(2), Position(1, 0));
    assert_eq!(to_pos(3), Position(1, -1));
    assert_eq!(to_pos(4), Position(0, -1));
    assert_eq!(to_pos(5), Position(-1, -1));
    assert_eq!(to_pos(6), Position(-1, 0));
    assert_eq!(to_pos(7), Position(-1, 1));
    assert_eq!(to_pos(8), Position(0, 1));
    assert_eq!(to_pos(9), Position(1, 1));
}
