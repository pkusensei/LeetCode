mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_size_slices(slices: &[i32]) -> i32 {
    let k = slices.len();
    let n = k / 3;
    // let mut dp = vec![vec![-1; 1 + n]; k];
    // dfs(&slices[1..], 0, n, &mut dp.clone()).max(dfs(&slices[..k - 1], 0, n, &mut dp))
    tabulation(&slices[1..], n).max(tabulation(&slices[..k - 1], n))
}

fn dfs(nums: &[i32], idx: usize, remains: usize, dp: &mut [Vec<i32>]) -> i32 {
    if idx >= nums.len() || remains == 0 {
        return 0;
    }
    if dp[idx][remains] > 0 {
        return dp[idx][remains];
    }
    let pick = nums[idx] + dfs(nums, 2 + idx, remains - 1, dp);
    let skip = dfs(nums, 1 + idx, remains, dp);
    let res = pick.max(skip);
    dp[idx][remains] = res;
    res
}

fn tabulation(nums: &[i32], n: usize) -> i32 {
    let len = nums.len();
    let mut dp = vec![vec![0; 1 + n]; 1 + len];
    for i1 in 1..=len {
        for i2 in 1..=n {
            if i1 == 1 {
                dp[i1][i2] = nums[0]
            } else {
                let pick = dp[i1 - 2][i2 - 1] + nums[i1 - 1];
                let skip = dp[i1 - 1][i2];
                dp[i1][i2] = pick.max(skip);
            }
        }
    }
    dp[len][n]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_size_slices(&[1, 2, 3, 4, 5, 6]), 10);
        assert_eq!(max_size_slices(&[8, 9, 8, 6, 1, 1]), 16);
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
