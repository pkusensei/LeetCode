mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_dp_opt(nums: &[i32], colors: &[i32]) -> i64 {
    let [mut dp1, mut dp2] = [0, 0];
    for (i, val) in nums.iter().enumerate() {
        let val = i64::from(*val);
        // dp1, dp2, curr
        let curr = if i > 0 && colors[i] == colors[i - 1] {
            dp2.max(val + dp1)
        } else {
            val + dp2
        };
        dp1 = dp2;
        dp2 = curr;
    }
    dp2
}

pub fn with_dp(nums: &[i32], colors: &[i32]) -> i64 {
    let n = nums.len();
    let mut dp = vec![0; n];
    for (i, &num) in nums.iter().enumerate().rev() {
        let val = i64::from(num);
        dp[i] = if colors.get(1 + i).is_some_and(|&c| c == colors[i]) {
            dp[1 + i].max(val + dp.get(2 + i).unwrap_or(&0))
        } else {
            val + dp.get(1 + i).unwrap_or(&0)
        };
    }
    dp[0]
}

pub fn rob(nums: &[i32], colors: &[i32]) -> i64 {
    let n = nums.len();
    dfs(&nums, &colors, 0, &mut vec![-1; n])
}

fn dfs(nums: &[i32], colors: &[i32], idx: usize, memo: &mut [i64]) -> i64 {
    if idx >= nums.len() {
        return 0;
    }
    if memo[idx] > -1 {
        return memo[idx];
    }
    let skip = dfs(nums, colors, 1 + idx, memo);
    let take = if colors.get(1 + idx).is_some_and(|&c| c == colors[idx]) {
        i64::from(nums[idx]) + dfs(nums, colors, 2 + idx, memo)
    } else {
        i64::from(nums[idx]) + dfs(nums, colors, 1 + idx, memo)
    };
    memo[idx] = skip.max(take);
    memo[idx]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
