pub struct UnionFind {
    leaders: Vec<usize>,
    heights: Vec<usize>,
}

impl UnionFind {
    pub fn new() -> Self {
        UnionFind {
            leaders: Vec::new(),
            heights: Vec::new(),
        }
    }

    pub fn leader(&self, i: usize) -> usize {
        assert!(i < self.leaders.len());
        if self.leaders[i] == i {
            i
        } else {
            self.leader(self.leaders[i])
        }
    }

    pub fn ensure_capacity(&mut self, i: usize) {
        if i >= self.leaders.len() {
            let len = self.leaders.len();
            self.leaders.extend(len..i+1);
            self.heights.extend((len..i+1).map(|_| 1));
        }
        assert!(self.leaders.len() > i && self.heights.len() > i);
    }

    pub fn len(&self) -> usize {
        self.leaders.len()
    }

    pub fn join(&mut self, i: usize, j: usize) {
        self.ensure_capacity(i);
        self.ensure_capacity(j);

        let li = self.leader(i);
        let lj = self.leader(j);

        if li != lj {
            let hi = self.heights[li];
            let hj = self.heights[lj];

            if hi > hj {
                self.leaders[lj] = li;
                // heights cannot change here
            } else {
                self.leaders[li] = lj;
                self.heights[li] = hi.max(hj + 1);
            }
        }
    }

    pub fn same_group(&self, i: usize) -> Vec<usize> {
        let l = self.leader(i);

        (0..self.leaders.len()).filter(|&j| self.leader(j) == l).collect()
    }

    pub fn groups(&self) -> Vec<usize> {
        let mut g: Vec<usize> = (0..self.leaders.len()).map(|j| self.leader(j)).collect();
        g.sort();
        g.dedup();
        g
    }
}
