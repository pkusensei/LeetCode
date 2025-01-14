mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
    let [n, m] = [n, m].map(|v| v as usize);
    // dfs(
    //     n,
    //     m,
    //     k,
    //     0,
    //     &mut vec![vec![vec![-1; 1 + m]; 1 + k as usize]; 1 + n],
    // )
    let k = k as usize;
    let mut dp = vec![vec![vec![0i64; 1 + k]; 1 + m]; 1 + n];
    for max in 1..=m {
        dp[1][max][1] = 1;
    }
    for len in 2..=n {
        for max in 1..=m {
            for cost in 1..=k {
                dp[len][max][cost] += dp[len - 1][max][cost] * (max as i64);
                for prev_max in 1..max {
                    dp[len][max][cost] += dp[len - 1][prev_max][cost - 1];
                }
                dp[len][max][cost] %= 1_000_000_007;
            }
        }
    }
    (1..=m).fold(0, |acc, max| (acc + dp[n][max][k]) % 1_000_000_007) as i32
}

fn dfs(n: usize, m: usize, k: i32, prev: usize, memo: &mut [Vec<Vec<i32>>]) -> i32 {
    if n == 0 {
        return i32::from(0 == k);
    }
    if k < 0 {
        return 0;
    }
    if memo[n][k as usize][prev] > -1 {
        return memo[n][k as usize][prev];
    }
    let mut res = 0;
    for curr in 1..=m {
        if prev < curr {
            res += dfs(n - 1, m, k - 1, curr, memo);
        } else {
            res += dfs(n - 1, m, k, prev, memo);
        }
        res %= 1_000_000_007;
    }
    memo[n][k as usize][prev] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_of_arrays(2, 3, 1), 6);
        assert_eq!(num_of_arrays(5, 2, 3), 0);
        assert_eq!(num_of_arrays(9, 1, 1), 1);
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
