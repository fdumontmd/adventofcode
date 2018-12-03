pub fn compute_code(data: &Vec<u64>, offset: usize) -> u64 {
    let mut sum = 0;

    for (i, n) in data.iter().enumerate() {
        if *n == data[ (i + offset) % data.len() ] {
            sum += n;
        }
    }
    return sum;
}

pub fn compute_code_iter(data: &Vec<u64>, offset: usize) -> u64 {
    data.iter().zip(data.iter().cycle().skip(offset))
        .filter_map(|(a, b)| if a == b { Some(*a) } else { None })
        .sum()
}

#[cfg(test)]
mod test{
    use super::*;
    use std::fs::File;
    use std::io::Read;
    fn read_data() -> Vec<u64> {
        let input = File::open("input.txt").unwrap();
        let mut data = Vec::new();

        for b in input.bytes() {
            let b = b.unwrap();
            if '0' as u8 <= b && b <= '9' as u8 {
                data.push((b - '0' as u8) as u64);
            }
        } 
        data
    }

    #[test]
    fn skip_1() {
        let data = read_data();
        assert_eq!(compute_code(&data, 1), compute_code_iter(&data, 1));
    }

    #[test]
    fn skip_half() {
        let data = read_data();
        assert_eq!(compute_code(&data, data.len() / 2),
                   compute_code_iter(&data, data.len() / 2));
    }
}

