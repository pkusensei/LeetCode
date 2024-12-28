mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_insertions(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    if n <= 1 {
        return 0;
    }
    if n == 2 {
        return i32::from(s[0] != s[1]);
    }
    let mut dp = vec![vec![0; n]; n];
    for len in 2..=n {
        for left in 0..=n - len {
            let right = left + len - 1;
            if s[left] == s[right] {
                dp[left][right] = dp[left + 1][right - 1];
            } else {
                dp[left][right] = 1 + dp[left + 1][right].min(dp[left][right - 1]);
            }
        }
    }
    dp[0][n - 1]
    // dfs(s, 0, n - 1, &mut vec![vec![-1; n]; n])
}

fn dfs(s: &[u8], left: usize, right: usize, memo: &mut [Vec<i32>]) -> i32 {
    if left >= right {
        return 0;
    }
    if s[left] == s[right] {
        return dfs(s, left + 1, right - 1, memo);
    }
    if memo[left][right] > -1 {
        return memo[left][right];
    }
    let res = 1 + dfs(s, left + 1, right, memo).min(dfs(s, left, right - 1, memo));
    memo[left][right] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_insertions("zzazz"), 0);
        assert_eq!(min_insertions("mbadm"), 2);
        assert_eq!(min_insertions("leetcode"), 5);
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
