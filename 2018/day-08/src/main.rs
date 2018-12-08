use aoc_utils::get_input;
fn input_to_usize_vec() -> Vec<usize> {
    use std::io::Read;
    let mut buf = String::new();
    get_input().read_to_string(&mut buf).unwrap();
    to_usize_vec(buf.trim().into())
}

fn to_usize_vec(input: String) -> Vec<usize> {
    input.split(' ').map(|v| v.parse().unwrap()).collect()
}

fn part_one(iter: &mut impl Iterator<Item = usize>) -> usize {
    let children_count = iter.next();

    let mut total_metadata = 0;
    children_count.map(|cc| {
        let metadata_count = iter.next().unwrap();
        for _ in 0..cc {
            total_metadata += part_one(iter);
        }

        for _ in 0..metadata_count {
            total_metadata += iter.next().unwrap();
        }

        total_metadata
    }).unwrap_or(0)
}

fn part_two(iter: &mut impl Iterator<Item = usize>) -> usize {
    let children_count = iter.next();

    let mut total_metadata = 0;
    children_count.map(|cc| {
        let metadata_count = iter.next().unwrap();
        let cmd: Vec<usize> = (0..cc).map(|_| part_two(iter)).collect();

        if cmd.is_empty() {
            for _ in 0..metadata_count {
                total_metadata += iter.next().unwrap();
            }
        } else {
            for _ in 0..metadata_count {
                let idx = iter.next().unwrap();

                if idx <= cmd.len() {
                    total_metadata += cmd[idx - 1];
                }
            }
        }


        total_metadata
    }).unwrap_or(0)
}

fn main() {
    println!("part one: checksum = {}", part_one(&mut input_to_usize_vec().into_iter()));
    println!("part two: checksum = {}", part_two(&mut input_to_usize_vec().into_iter()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input: Vec<usize> = vec![2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2];
        assert_eq!(138, part_one(&mut input.into_iter()));
    }

    #[test]
    fn test_part_two() {
        let input: Vec<usize> = vec![2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2];
        assert_eq!(66, part_two(&mut input.into_iter()));
    }
}
