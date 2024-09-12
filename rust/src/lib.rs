mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Default)]
struct RandomizedSet {
    map: HashMap<i32, usize>,
    nums: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.nums.push(val);
        let idx = self.nums.len() - 1;
        self.map.insert(val, idx);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.map.remove(&val) {
            self.nums.swap_remove(i);
            if i < self.nums.len() {
                self.map.insert(self.nums[i], i);
            }
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        *self.nums.choose(&mut thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut set = RandomizedSet::new();
        debug_assert!(set.insert(1)); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
        debug_assert!(!set.remove(2)); // Returns false as 2 does not exist in the set.
        debug_assert!(set.insert(2)); // Inserts 2 to the set, returns true. Set now contains [1,2].
        set.get_random(); // getRandom() should return either 1 or 2 randomly.
        debug_assert!(set.remove(1)); // Removes 1 from the set, returns true. Set now contains [2].
        debug_assert!(!set.insert(2)); // 2 was already in the set, so return false.
        set.get_random(); // Since 2 is the only number in the set, getRandom() will always return 2.
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
