mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_deletion_size(strs: &[&str]) -> i32 {
    let n = strs[0].len();
    let mut dp = vec![1; n];
    for i1 in 1..n {
        for i2 in 0..i1 {
            if strs.iter().all(|s| s.as_bytes()[i1] >= s.as_bytes()[i2]) {
                dp[i1] = dp[i1].max(1 + dp[i2])
            }
        }
    }
    n as i32 - dp.into_iter().max().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_deletion_size(&["babca", "bbazb"]), 3);
        debug_assert_eq!(min_deletion_size(&["edcba"]), 4);
        debug_assert_eq!(min_deletion_size(&["ghi", "def", "abc"]), 0);
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
