mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_difference(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 3 {
        return 0;
    }
    nums.sort_unstable();
        (0..4).map(|i| nums[n - 4 + i] - nums[i]).min().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_difference(vec![5, 3, 2, 4]), 0);
        assert_eq!(min_difference(vec![1, 5, 0, 10, 14]), 1);
        assert_eq!(min_difference(vec![3, 100, 20]), 0);
    }

    #[test]
    fn test() {
        assert_eq!(min_difference(vec![90, 35, 67, 53, 61]), 6);
        assert_eq!(min_difference(vec![6, 6, 0, 1, 1, 4, 6]), 2);
    }

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
