mod helper;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn top_k_frequent(nums: &[i32], k: i32) -> Vec<i32> {
        let counts = nums.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
        let mut heap = BinaryHeap::new();
        for (num, count) in counts.into_iter() {
            heap.push((Reverse(count), num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.into_iter().map(|(_, num)| num).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(top_k_frequent(&[1, 1, 1, 2, 2, 3], 2), [1, 2]);
        sort_eq(top_k_frequent(&[1], 1), [1]);
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
