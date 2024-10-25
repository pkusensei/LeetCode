mod helper;
mod trie;

use core::f64;

#[allow(unused_imports)]
use helper::*;

pub fn largest_sum_of_averages(nums: &[i32], k: i32) -> f64 {
    let (n, k) = (nums.len(), k as usize);
    let mut prefix = Vec::with_capacity(1 + n);
    prefix.push(0.0);
    for &num in nums.iter() {
        prefix.push(f64::from(num) + prefix.last().unwrap_or(&0.0));
    }
    // dfs(&prefix, n, 0, k, &mut vec![vec![-1.0; 1 + k]; n])
    let mut dp: Vec<_> = (0..n)
        .map(|i| (prefix[n] - prefix[i]) / (n - i) as f64)
        .collect();
    for _ in 0..k - 1 {
        for i1 in 0..n {
            for i2 in 1 + i1..n {
                dp[i1] = dp[i1].max((prefix[i2] - prefix[i1]) / (i2 - i1) as f64 + dp[i2]);
            }
        }
    }
    dp[0]
}

fn dfs(prefix: &[f64], n: usize, idx: usize, k: usize, dp: &mut [Vec<f64>]) -> f64 {
    if idx >= n {
        return 0.0;
    }
    if k == 0 {
        return f64::MIN;
    }
    if dp[idx][k] > -1.0 {
        return dp[idx][k];
    }
    let mut res = f64::MIN;
    for i in idx..n {
        // 1+i => prefix starts with artificial 0.0
        let mut temp = (prefix[1 + i] - prefix[idx]) / (1 + i - idx) as f64;
        temp += dfs(prefix, n, 1 + i, k - 1, dp);
        res = res.max(temp);
    }
    dp[idx][k] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_sum_of_averages(&[9, 1, 2, 3, 9], 3), 20.0);
        debug_assert_eq!(largest_sum_of_averages(&[1, 2, 3, 4, 5, 6, 7], 4), 20.5);
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
