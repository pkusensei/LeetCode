mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn xor_all_nums(nums1: &[i32], nums2: &[i32]) -> i32 {
    let [n1, n2] = [nums1, nums2].map(|v| v.len());
    let f = |acc, v| acc ^ v;
    match [n1 & 1, n2 & 1] {
        [0, 0] => 0,
        [1, 0] => nums2.iter().fold(0, f),
        [0, 1] => nums1.iter().fold(0, f),
        _ => nums1.iter().chain(nums2.iter()).fold(0, f),
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(xor_all_nums(&[2, 1, 3], &[10, 2, 5, 0]), 13);
        assert_eq!(xor_all_nums(&[1, 2], &[3, 4]), 0);
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
