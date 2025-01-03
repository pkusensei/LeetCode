mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ways_to_split_array(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut prefix = Vec::with_capacity(n);
    for &num in nums.iter() {
        prefix.push(i64::from(num) + prefix.last().unwrap_or(&0));
    }
    let mut res = 0;
    for i in 0..n - 1 {
        let a = prefix[i];
        let b = prefix[n - 1] - prefix[i];
        res += i32::from(a >= b);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(ways_to_split_array(&[10, 4, -8, 7]), 2);
        assert_eq!(ways_to_split_array(&[2, 3, 1, 0]), 2);
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
