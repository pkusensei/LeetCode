mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyHashSet {
    data: Vec<bool>,
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            data: vec![false; 100_001],
        }
    }

    fn add(&mut self, key: i32) {
        self.data[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.data[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
        self.data[key as usize]
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut s = MyHashSet::new();
        s.add(1); // set = [1]
        s.add(2); // set = [1, 2]
        debug_assert!(s.contains(1)); // return True
        debug_assert!(!s.contains(3)); // return False, (not found)
        s.add(2); // set = [1, 2]
        debug_assert!(s.contains(2)); // return True
        s.remove(2); // set = [1]
        debug_assert!(!s.contains(2)); // return False, (already removed)
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
