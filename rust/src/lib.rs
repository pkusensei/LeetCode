mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_substrings(s: &str, t: &str) -> i32 {
    let n1 = s.len();
    let n2 = t.len();
    // all equal matches
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    // all matches with one miss
    let mut dp1 = vec![vec![0; 1 + n2]; 1 + n1];
    let mut res = 0;
    for i1 in 1..=n1 {
        for i2 in 1..=n2 {
            if s.as_bytes()[i1 - 1] == t.as_bytes()[i2 - 1] {
                dp[i1][i2] = 1 + dp[i1 - 1][i2 - 1]; // a new match!
                dp1[i1][i2] = dp1[i1 - 1][i2 - 1]; // one miss is still one miss
            } else {
                dp1[i1][i2] = 1 + dp[i1 - 1][i2 - 1]; // a new miss!
            }
            res += dp1[i1][i2];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(count_substrings("aba", "baba"), 6);
        assert_eq!(count_substrings("ab", "bb"), 3);
    }

    #[test]
    fn test() {}

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
