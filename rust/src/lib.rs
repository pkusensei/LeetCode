mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_coins(mut nums: Vec<i32>) -> i32 {
    nums.reserve_exact(2);
    nums.insert(0, 1);
    nums.push(1);
    let n = nums.len();
    let mut dp = vec![vec![0; n]; n];
    solve(&nums, &mut dp, 1, n - 2)
}

fn solve(nums: &[i32], dp: &mut [Vec<i32>], left: usize, right: usize) -> i32 {
    if left > right {
        return 0;
    }
    if 0 < dp[left][right] {
        return dp[left][right];
    }
    let mut res = 0;
    for i in left..=right {
        let curr = nums[left - 1] * nums[i] * nums[right + 1];
        let remain = solve(nums, dp, left, i - 1) + solve(nums, dp, i + 1, right);
        res = res.max(curr + remain);
    }
    dp[left][right] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_coins(vec![3, 1, 5, 8]), 167);
        debug_assert_eq!(max_coins(vec![1, 5]), 10);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
