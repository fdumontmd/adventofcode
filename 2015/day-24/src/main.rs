#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Selection {
    Selected,
    Skipped,
}

impl Selection {
    fn next(&self) -> Option<Selection> {
        match self {
            &Selection::Selected => Some(Selection::Skipped),
            &Selection::Skipped => None,
        }
    }
}

struct AllocationIter<'a> {
    weights: &'a Vec<usize>,
    allocation: Vec<Selection>,
    target_weight: usize,
    max_len: usize,
}

impl<'a> AllocationIter<'a> {
    fn new(weights: &'a Vec<usize>, target_weight: usize) -> Self {
        AllocationIter{
            weights,
            allocation: Vec::with_capacity(weights.len()),
            target_weight, 
            max_len: std::usize::MAX,
        }
    }

    fn state(&self) -> (usize, usize) {
        self.allocation.iter().zip(self.weights.iter()).filter(|&(s, _)| *s == Selection::Selected).fold((0, 0), |(s, c), (_,&w)| (s+w, c+1))
    }

    fn maybe_valid(&self) -> bool {
        let (total_weight, count) = self.state();
        total_weight <= self.target_weight && count <= self.max_len
    }

    fn valid(&self) -> bool {
        let (total_weight, count) = self.state();
        total_weight == self.target_weight && count <= self.max_len
    }

    fn push(&mut self) -> bool {
        if self.allocation.len() < self.weights.len() {
            self.allocation.push(Selection::Selected);
            true
        } else {
            false
        }
    }

    fn backtrack(&mut self) -> bool {
        while let Some(top) = self.allocation.pop() {
            if let Some(next) = top.next() {
                self.allocation.push(next);
                return true;
            }
        }
        false
    }
}

impl<'a> Iterator for AllocationIter<'a> {
    type Item = (Vec<usize>, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        // try to prepare for next item; no impact if not ready yet
        self.backtrack();

        loop {
            while self.push() {
                if !self.maybe_valid() {
                    if !self.backtrack() {
                        return None;
                    }
                }
            }

            if self.valid() {
                let (selected, skipped): (Vec<(usize, Selection)>, Vec<(usize, Selection)>) = self.weights.iter().cloned().zip(self.allocation.iter().cloned()).partition(|&(_, s)| s == Selection::Selected);
                return Some((selected.into_iter().map(|(w,_)| w).collect(), skipped.into_iter().map(|(w, _)| w).collect()));
            }

            if !self.backtrack() {
                return None;
            }
        }
    }
}

struct SolutionIter<'a> {
    allocation_iter: AllocationIter<'a>,
}

impl<'a> SolutionIter<'a> {
    fn new(weights: &'a Vec<usize>, group_counts: usize) -> Self {
        let total_weight: usize = weights.iter().sum();
        SolutionIter {
            allocation_iter: AllocationIter::new(weights, total_weight / group_counts),
        }
    }
}

fn quantum_entanglement(weights: &Vec<usize>) -> usize {
    let mut prod = 1;

    for w in weights {
        prod *= w;
    }
    prod
}

impl<'a> Iterator for SolutionIter<'a> {
    type Item = (usize, usize, Vec<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((a, r)) = self.allocation_iter.next() {
            if AllocationIter::new(&r, self.allocation_iter.target_weight).next().is_some() {
                self.allocation_iter.max_len = a.len();
                return Some((a.len(), quantum_entanglement(&a), a));
            }
        }
        None
    }
}

fn read_input_file() -> Vec<usize> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::env::args;

    assert!(args().len() > 1);
    let path = args().nth(1).unwrap();
    let input = File::open(&path).unwrap();
    let buf = BufReader::new(input);
    let mut d = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap();
        d.push(line.parse::<usize>().unwrap());
    }

    d
}

fn main() {
    //let d = vec![1,2,3,4,5,7,8,9,10,11];
    let d = read_input_file();
    let total = d.iter().sum::<usize>();

    println!("Total weight: {}", total);

    println!("Split in 3 {:?}", SolutionIter::new(&d, 3).min().unwrap());
    println!("Split in 4 {:?}", SolutionIter::new(&d, 4).min().unwrap());
}
