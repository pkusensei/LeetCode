mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn get_final_state(nums: &[i32], k: i32, multiplier: i32) -> Vec<i32> {
    let mut heap: BinaryHeap<_> = nums
        .iter()
        .enumerate()
        .map(|(i, &v)| Reverse((v, i)))
        .collect();
    for _ in 0..k {
        let Some(Reverse((mut v, i))) = heap.pop() else {
            continue;
        };
        v *= multiplier;
        heap.push(Reverse((v, i)));
    }
    let mut res = vec![0; nums.len()];
    for Reverse((v, i)) in heap.into_iter() {
        res[i] = v;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(get_final_state(&[2, 1, 3, 5, 6], 5, 2), [8, 4, 6, 5, 6]);
        assert_eq!(get_final_state(&[1, 2], 3, 4), [16, 8]);
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}",
        );
    }
}
