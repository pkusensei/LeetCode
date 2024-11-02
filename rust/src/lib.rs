mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn profitable_schemes(n: i32, min_profit: i32, group: &[i32], profit: &[i32]) -> i32 {
    let size = group.len();
    let mut dp = vec![vec![vec![-1; size]; 1 + min_profit as usize]; 1 + n as usize];
    dfs(n, min_profit, group, profit, 0, &mut dp)
}

fn dfs(
    n: i32,
    min_profit: i32,
    group: &[i32],
    profit: &[i32],
    idx: usize,
    dp: &mut [Vec<Vec<i32>>],
) -> i32 {
    const MOD: i32 = 1_000_000_007;
    if n < 0 {
        return 0;
    }
    if idx == group.len() {
        return (n >= 0 && min_profit == 0).into();
    }
    if dp[n as usize][min_profit as usize][idx] > -1 {
        return dp[n as usize][min_profit as usize][idx];
    }
    let pick = dfs(
        n - group[idx],
        (min_profit - profit[idx]).max(0),
        group,
        profit,
        1 + idx,
        dp,
    );
    let skip = dfs(n, min_profit, group, profit, 1 + idx, dp);
    let res = (pick + skip) % MOD;
    dp[n as usize][min_profit as usize][idx] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(profitable_schemes(5, 3, &[2, 2], &[2, 3]), 2);
        debug_assert_eq!(profitable_schemes(10, 5, &[2, 3, 5], &[6, 7, 8]), 7);
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
