mod dsu;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
    let mut heap: BinaryHeap<_> = arr
        .into_iter()
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_values()
        .map(Reverse)
        .collect();
    while k > 0 {
        let Some(Reverse(count)) = heap.pop() else {
            break;
        };
        k -= count;
    }
    heap.len() as i32 + i32::from(k < 0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
