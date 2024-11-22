mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_uncrossed_lines(nums1: &[i32], nums2: &[i32]) -> i32 {
    let (n1, n2) = (nums1.len(), nums2.len());
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for i1 in 1..=n1 {
        for i2 in 1..=n2 {
            if nums1[i1 - 1] == nums2[i2 - 1] {
                dp[i1][i2] = 1 + dp[i1 - 1][i2 - 1];
            } else {
                dp[i1][i2] = dp[i1 - 1][i2].max(dp[i1][i2 - 1])
            }
        }
    }
    dp[n1][n2]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_uncrossed_lines(&[1, 4, 2], &[1, 2, 4]), 2);
        debug_assert_eq!(
            max_uncrossed_lines(&[2, 5, 1, 2, 5], &[10, 5, 2, 1, 5, 2]),
            3
        );
        debug_assert_eq!(
            max_uncrossed_lines(&[1, 3, 7, 1, 7, 5], &[1, 9, 2, 5, 1]),
            2
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
