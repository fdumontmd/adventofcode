use std::collections::VecDeque;

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

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
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
