mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    // dfs(0, n, k, max_pts, &mut vec![-1.0; 1 + n as usize])
    if k == 0 || n >= k + max_pts {
        return 1.0;
    }
    let [n, k, mp] = [n, k, max_pts].map(|v| v as usize);
    let mut dp = Vec::with_capacity(1 + n);
    dp.push(1.0);
    let mut curr = 1.0; // the sum of previous window [score-mp..score-1]
    for score in 1..=n {
        dp.push(curr / mp as f64);
        if score < k {
            curr += dp[score];
        }
        if score >= mp {
            curr -= dp[score - mp];
        }
    }
    dp[k..].iter().sum()
}

// tle
fn dfs(curr: i32, n: i32, k: i32, max_pts: i32, dp: &mut [f64]) -> f64 {
    if n < curr {
        return 0.0;
    }
    if k <= curr {
        return 1.0;
    }
    if dp[curr as usize] > -1.0 {
        return dp[curr as usize];
    }
    let mut res = 0.0;
    for delta in 1..=max_pts {
        res += dfs(curr + delta, n, k, max_pts, dp) / f64::from(max_pts)
    }
    dp[curr as usize] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(new21_game(10, 1, 10), 1.0);
        debug_assert_eq!(new21_game(6, 1, 10), 0.6);
        debug_assert_eq!(new21_game(21, 17, 10), 0.73278);
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
}
