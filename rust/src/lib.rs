mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn partition_disjoint(nums: &[i32]) -> i32 {
    let n = nums.len();
    let [mut left, mut right] = [0, 1].map(|_| Vec::with_capacity(n));
    for &num in nums.iter() {
        left.push(num.max(left.last().copied().unwrap_or(0)));
    }
    for &num in nums.iter().rev() {
        right.push(num.min(right.last().copied().unwrap_or(i32::MAX)));
    }
    right.reverse();
    for i in 0..n - 1 {
        if left[i] <= right[i + 1] {
            return 1 + i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(partition_disjoint(&[5, 0, 3, 8, 6]), 3);
        debug_assert_eq!(partition_disjoint(&[1, 1, 1, 0, 6, 12]), 4);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
