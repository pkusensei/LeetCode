mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_subarrays(arr: &[i32], k: i32, threshold: i32) -> i32 {
    let sum = k * threshold;
    let (n, k) = (arr.len(), k as usize);
    let mut curr: i32 = arr[..k].iter().sum();
    let mut res = i32::from(curr >= sum);
    for i in 1..=n - k {
        curr += arr[i + k - 1] - arr[i - 1];
        res += i32::from(curr >= sum);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_of_subarrays(&[2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
        assert_eq!(
            num_of_subarrays(&[11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
            6
        );
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
