// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
pub fn extended_gcd(a: i64, b: i64) -> (i64, (i64, i64), (i64, i64)) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r.div_euclid(r);
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_r, (old_s, old_t), (t, s))
}

// https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_(direct_construction)
pub fn chinese_remainders(input: &[(i64, i64)]) -> (i64, i64) {
    let n: i64 = input.iter().map(|(_, ni)| ni).product();

    (
        input
            .iter()
            .map(|(ai, ni)| {
                let (_, (bni, _), _) = extended_gcd(n / ni, *ni);
                ai * bni * (n / ni)
            })
            .sum(),
        n,
    )
}

#[cfg(test)]
mod test {
    use crate::num::{chinese_remainders, extended_gcd};

    #[test]
    fn test_egcd_240_46() {
        assert_eq!((2, (-9, 47), (-120, 23)), extended_gcd(240, 46));
    }

    #[test]
    fn test_cr_5_7_12() {
        assert_eq!((37, 420), chinese_remainders(&[(2, 5), (2, 7), (1, 12)]));
    }

    #[test]
    fn test_cr_3_5_7() {
        assert_eq!((23, 105), chinese_remainders(&[(2, 3), (3, 5), (2, 7)]));
    }
}
