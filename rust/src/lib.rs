mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn find_score(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut seen = vec![false; n];
    let mut heap: BinaryHeap<_> = nums
        .iter()
        .enumerate()
        .map(|(i, &v)| Reverse((v, i)))
        .collect();
    let mut res = 0;
    while let Some(Reverse((num, idx))) = heap.pop() {
        if seen[idx] {
            continue;
        }
        seen[idx] = true;
        res += i64::from(num);
        let left = idx.saturating_sub(1);
        let right = (1 + idx).min(n - 1);
        seen[left] = true;
        seen[right] = true;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(find_score(&[2, 1, 3, 4, 5, 2]), 7);
        assert_eq!(find_score(&[2, 3, 5, 1, 3, 2]), 5);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
