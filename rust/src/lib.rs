mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_non_overlapping(nums: &[i32], target: i32) -> i32 {
    let mut prefix = 0;
    let mut seen = std::collections::HashSet::from([0]);
    let mut res = 0;
    for &num in nums.iter() {
        prefix += num;
        if seen.contains(&(prefix - target)) {
            res += 1;
            // As soon as one subarr is found, discard all previous numbers
            // So that further subarrs cannot extend lefthand => no overlap
            seen.clear();
        }
        seen.insert(prefix);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_non_overlapping(&[1, 1, 1, 1, 1], 2), 2);
        assert_eq!(max_non_overlapping(&[-1, 3, 5, 1, 4, 2, -9], 6), 2);
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
            "left = {a:?}, right = {b:?}"
        );
    }
}
