mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_arrays(s: &str, k: i32) -> i32 {
    let n = s.len();
    // dfs(s, k, 0, &mut vec![-1; n])
    let mut dp = vec![0; 1 + n];
    dp[n] = 1;
    for idx in (0..n).rev() {
        let mut curr = 0;
        for i in idx..n {
            if s[idx..=i]
                .parse::<i32>()
                .is_ok_and(|v| (1..=k).contains(&v))
            {
                curr += dp[1 + i];
                curr %= 1_000_000_007;
            }
        }
        dp[idx] = curr;
    }
    dp[0]
}

fn dfs(s: &str, k: i32, idx: usize, memo: &mut [i32]) -> i32 {
    if idx >= s.len() {
        return 1;
    }
    // [1..=k]
    if s[idx..].starts_with('0') {
        return 0;
    }
    if memo[idx] > -1 {
        return memo[idx];
    }
    let n = k.ilog10() as usize + 1;
    let mut res = 0;
    for len in 1..=n {
        if idx + len > s.len() {
            break;
        }
        if s[idx..idx + len].parse::<i32>().is_ok_and(|v| v <= k) {
            res += dfs(s, k, idx + len, memo);
            res %= 1_000_000_007;
        }
    }
    memo[idx] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(number_of_arrays("1000", 10000), 1);
        assert_eq!(number_of_arrays("1000", 10), 0);
        assert_eq!(number_of_arrays("1317", 2000), 8);
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
