mod helper;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Default)]
struct RandomizedCollection {
    map: HashMap<i32, HashSet<usize>>,
    nums: Vec<i32>,
}

impl RandomizedCollection {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        self.nums.push(val);
        let idx = self.nums.len() - 1;
        let e = self.map.entry(val).or_default();
        e.insert(idx);
        e.len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(ids) = self.map.get_mut(&val) {
            if let Some(&i) = ids.iter().next() {
                ids.remove(&i);
                self.nums.swap_remove(i);
                if i < self.nums.len() {
                    let s = self.map.get_mut(&self.nums[i]).unwrap();
                    s.remove(&self.nums.len());
                    s.insert(i);
                }
                return true;
            }
        }
        false
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
        let mut set = RandomizedCollection::new();
        debug_assert!(set.insert(1)); // return true since the collection does not contain 1.
                                      // Inserts 1 into the collection.
        debug_assert!(!set.insert(1)); // return false since the collection contains 1.
                                       // Inserts another 1 into the collection. Collection now contains [1,1].
        debug_assert!(set.insert(2)); // return true since the collection does not contain 2.
                                      // Inserts 2 into the collection. Collection now contains [1,1,2].
        set.get_random(); // getRandom should:
                          // - return 1 with probability 2/3, or
                          // - return 2 with probability 1/3.
        debug_assert!(set.remove(1)); // return true since the collection contains 1.
                                      // Removes 1 from the collection. Collection now contains [1,2].
        set.get_random(); // getRandom should return 1 or 2, both equally likely.
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
