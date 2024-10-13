mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone)]
struct MyHashMap {
    data: Vec<i32>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            data: vec![-1; 1_000_001],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.data[key as usize] = value;
    }

    fn remove(&mut self, key: i32) {
        self.data[key as usize] = -1;
    }

    fn get(&self, key: i32) -> i32 {
        self.data[key as usize]
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut m = MyHashMap::new();
        m.put(1, 1); // The map is now [[1,1]]
        m.put(2, 2); // The map is now [[1,1], [2,2]]
        debug_assert_eq!(m.get(1), 1); // return 1, The map is now [[1,1], [2,2]]
        debug_assert_eq!(m.get(3), -1); // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
        m.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
        debug_assert_eq!(m.get(2), 1); // return 1, The map is now [[1,1], [2,1]]
        m.remove(2); // remove the mapping for 2, The map is now [[1,1]]
        debug_assert_eq!(m.get(2), -1); // return -1 (i.e., not found), The map is now [[1,1]] // return False, (already removed)
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
