mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_div_three(nums: &[i32]) -> i32 {
    // let n = nums.len();
    // dfs(nums, 0, 0, &mut vec![[0; 3]; n])
    let mut dp = [0; 3];
    for num in nums.iter() {
        let mut next = dp;
        for v in dp.iter() {
            let sum = v + num;
            // this sum falls into idx==sum%3 bucket
            let idx = sum as usize % 3;
            next[idx] = next[idx].max(sum);
        }
        dp = next;
    }
    dp[0]
}

fn dfs(nums: &[i32], idx: usize, rem: usize, dp: &mut [[i32; 3]]) -> i32 {
    if idx >= nums.len() {
        return if rem == 0 { 0 } else { i32::MIN };
    }
    if dp[idx][rem] > 0 {
        return dp[idx][rem];
    }
    let num = nums[idx];
    let res =
        (num + dfs(nums, 1 + idx, (rem + num as usize) % 3, dp)).max(dfs(nums, 1 + idx, rem, dp));
    dp[idx][rem] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_sum_div_three(&[3, 6, 5, 1, 8]), 18);
        assert_eq!(max_sum_div_three(&[4]), 0);
        assert_eq!(max_sum_div_three(&[1, 2, 3, 4, 4]), 12);
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
            "left = {a:?}, right = {b:?}",
        );
    }
}
