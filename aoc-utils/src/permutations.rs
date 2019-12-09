use std::cmp::Ord;

pub struct Permutation<T: Ord> {
    init: bool,
    base: Vec<T>,
}

impl<T: Ord+Copy> Permutation<T> {
    pub fn new(mut elts: Vec<T>) -> Self {
        elts.sort();

        Permutation {
            init: true,
            base: elts.clone(),
        }
    }

    fn compute_next_permutation(&mut self) -> Option<Vec<T>> {
        let k = self.base.windows(2).rposition(|elts| elts[0] <elts[1])?;
        let l = self.base[k..].iter().rposition(|&al| self.base[k] < al)? + k;
        self.base.swap(k, l);
        self.base[k+1..].reverse();
        Some(self.base.clone())
    }
}

impl<T: Ord+Copy> Iterator for Permutation<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.init {
            self.init = false;
            Some(self.base.clone())
        } else {
            self.compute_next_permutation()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_perm() {
        let mut perm = Permutation::new(vec![1,2,3,4,]);
        assert_eq!(perm.next(), Some(vec![1,2,3,4]));
        assert_eq!(perm.next(), Some(vec![1,2,4,3]));
        assert_eq!(perm.next(), Some(vec![1,3,2,4]));
    }

    #[test]
    fn test_count_perm() {
        let perm = Permutation::new(vec![1,2,3,4]);
        assert_eq!(perm.count(), 24);
    }
}
