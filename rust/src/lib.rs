mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_common_subsequence(text1: &str, text2: &str) -> i32 {
    let (n1, n2) = (text1.len(), text2.len());
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for i1 in 1..=n1 {
        for i2 in 1..=n2 {
            if text1.as_bytes()[i1 - 1] == text2.as_bytes()[i2 - 1] {
                dp[i1][i2] = 1 + dp[i1 - 1][i2 - 1];
            } else {
                dp[i1][i2] = dp[i1 - 1][i2].max(dp[i1][i2 - 1]);
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
        debug_assert_eq!(longest_common_subsequence("abcde", "ace"), 3);
        debug_assert_eq!(longest_common_subsequence("abc", "abc"), 3);
        debug_assert_eq!(longest_common_subsequence("abc", "def"), 0);
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
