const INPUT: &'static str = "^.....^.^^^^^.^..^^.^.......^^..^^^..^^^^..^.^^.^.^....^^...^^.^^.^...^^.^^^^..^^.....^.^...^.^.^^.^";

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Safe,
    Trap,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => unreachable!(),
        }
    }

    fn from_previous_row(r: &[Tile]) -> Self {
        use Tile::*;
        let r = (r[0], r[1], r[2]);
        match r {
            (Trap, Trap, Safe) => Trap,
            (Safe, Trap, Trap) => Trap,
            (Trap, Safe, Safe) => Trap,
            (Safe, Safe, Trap) => Trap,
            _ => Safe,
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Row(Vec<Tile>);

impl Row {
    fn new(input: &str) -> Self {
        let mut row = Vec::new();
        row.push(Tile::Safe);
        for c in input.chars() {
            row.push(Tile::new(c));
        }
        row.push(Tile::Safe);

        Row(row)
    }

    fn from_row(previous: &Row) -> Self {
        let mut row = Vec::new();
        row.push(Tile::Safe);

        for chunks in previous.0.as_slice().windows(3) {
            row.push(Tile::from_previous_row(chunks));
        }

        row.push(Tile::Safe);

        assert_eq!(row.len(), previous.0.len());
        Row(row)
    }

    fn safe_count(&self) -> usize {
        let sum: usize = self.0.iter().map(|t|
                            match *t {
                                Tile::Safe => 1,
                                Tile::Trap => 0,
                            }).sum();
        sum - 2
    }
}

fn main() {
    let mut row = Row::new(INPUT);
    let mut sum = 0;
    for _ in 0..40 {
        sum += row.safe_count();
        row = Row::from_row(&row);
    }

    println!("Safe count for 40 rows: {}", sum);

    let mut row = Row::new(INPUT);
    let mut sum = 0;
    for _ in 0..400000 {
        sum += row.safe_count();
        row = Row::from_row(&row);
    }

    println!("Safe count for 400000 rows: {}", sum);
}

#[test]
fn test() {
    let first = Row::new("..^^.");
    let second = Row::from_row(&first);
    assert_eq!(second, Row::new(".^^^^"));
    let third = Row::from_row(&second);
    assert_eq!(third, Row::new("^^..^"));
}

#[test]
fn larger_test() {
    let mut sum = 0;
    let mut row = Row::new(".^^.^.^^^^");
    for _ in 0..10 {
        sum += row.safe_count();
        row = Row::from_row(&row);
    }
    assert_eq!(38, sum);
}
