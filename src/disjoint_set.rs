use std::collections::HashMap;

#[derive(Debug)]
pub struct DisjointSet {
    pub sets: HashMap<usize, Vec<usize>>,
}

impl DisjointSet {
    pub fn new() -> Self {
        DisjointSet {
            sets: HashMap::new(),
        }
    }

    pub fn make_set(&mut self, x: usize) {
        self.sets.insert(x, vec![x]);
    }

    pub fn find_set(&self, x: usize) -> Option<usize> {
        if self.sets.contains_key(&x) {
            return Some(x);
        }

        for (rep, set) in self.sets.iter() {
            for el in set {
                if *el == x {
                    return Some(*rep);
                }
            }
        }

        None
    }

    pub fn union(&mut self, x: usize, y: usize) {
        if x != y {
            if let Some(y_set) = self.find_set(y) {
                if let Some(x_set) = self.find_set(x) {
                    let mut y_set_values = self.sets.remove(&y_set).unwrap();
                    let x_set_values = self.sets.get_mut(&x_set).unwrap();
                    x_set_values.append(&mut y_set_values);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::DisjointSet;

    #[test]
    fn basic() {
        let mut ds = DisjointSet::new();

        ds.make_set(1);
        ds.make_set(2);

        ds.union(1, 2);

        ds.make_set(4);
        ds.make_set(8);

        ds.union(1, 4);

        assert_eq!(ds.find_set(4), ds.find_set(2));
        assert_ne!(ds.find_set(8), ds.find_set(2));
    }

    #[test]
    fn multiple_sets() {
        let mut ds = DisjointSet::new();
        ds.make_set(1);
        ds.make_set(2);
        ds.make_set(3);
        ds.make_set(4);

        ds.union(1, 2);
        ds.union(3, 4);

        assert_eq!(ds.find_set(1), Some(1));
        assert_eq!(ds.find_set(2), Some(1));
        assert_eq!(ds.find_set(3), Some(3));
        assert_eq!(ds.find_set(4), Some(3));
    }

    #[test]
    fn union_with_non_existing_set() {
        let mut ds = DisjointSet::new();
        ds.make_set(1);
        ds.make_set(2);

        ds.union(1, 3);

        assert_eq!(ds.find_set(1), Some(1));
        assert_eq!(ds.find_set(2), Some(2));
        assert_eq!(ds.find_set(3), None);
    }

    #[test]
    fn find_non_existing_element() {
        let mut ds = DisjointSet::new();
        ds.make_set(1);
        ds.make_set(2);

        assert_eq!(ds.find_set(3), None);
    }

    #[test]
    fn union_with_itself() {
        let mut ds = DisjointSet::new();
        ds.make_set(1);

        ds.union(1, 1);

        assert_eq!(ds.find_set(1), Some(1));
    }
}

