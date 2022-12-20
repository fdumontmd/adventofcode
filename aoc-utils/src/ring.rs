use std::collections::VecDeque;

#[derive(Debug)]
pub struct Ring<T>(VecDeque<T>);

impl<T> Ring<T> {
    pub fn new() -> Self {
        Ring(VecDeque::new())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    pub fn remove(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    pub fn insert(&mut self, value: T) {
        self.0.push_front(value)
    }

    pub fn move_right(&mut self, steps: usize) {
        for _ in 0..steps {
            if let Some(v) = self.0.pop_front() {
                self.0.push_back(v);
            }
        }
    }

    pub fn move_left(&mut self, steps: usize) {
        for _ in 0..steps {
            if let Some(v) = self.0.pop_back() {
                self.0.push_front(v);
            }
        }
    }

    pub fn move_signed(&mut self, steps: isize) {
        let steps = steps % self.len() as isize;
        if steps < 0 {
            self.move_left(steps.abs() as usize);
        } else {
            self.move_right(steps as usize);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_inner(self) -> VecDeque<T> {
        self.0
    }
}

impl<T> Default for Ring<T> {
    fn default() -> Self {
        Ring::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ring() {
        let mut ring = Ring::new();
        ring.insert(0);
        ring.move_right(2);
        ring.insert(1);
        ring.move_right(2);
        ring.insert(2);
        ring.move_left(4);

        assert_eq!(vec![&0, &2, &1], ring.iter().collect::<Vec<_>>());
    }
}
