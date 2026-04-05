mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    if k == 0 {
        return 0;
    }
    if 2 * k > n {
        return -1;
    }
    let costs: Vec<i32> = nums
        .iter()
        .enumerate()
        .map(|(idx, v)| {
            let prev_i = (idx + n - 1) % n;
            let next_i = (idx + 1) % n;
            ((1 + nums[prev_i].max(nums[next_i])) - v).max(0)
        })
        .collect();
    // f(&costs, 1, n - 1, k).min(costs[0] + f(&costs, 2, n - 2, k - 1))
    with_dp(&costs, 1, n - 1, k).min(costs[0] + with_dp(&costs, 2, n - 2, k - 1))
}

fn with_dp(costs: &[i32], start: usize, end: usize, k: usize) -> i32 {
    let n = costs.len();
    let mut dp = vec![vec![i32::MAX >> 1; 1 + k]; 2 + n];
    dp[start][0] = 0;
    for i in start..=end {
        for k_ in 0..=k {
            dp[1 + i][k_] = dp[i][k_]; // skip
            if k_ > 0 {
                dp[2 + i][k_] = dp[2 + i][k_].min(costs[i] + dp[i][k_ - 1]);
            }
        }
    }
    dp[1 + end][k].min(dp[2 + end][k])
}

fn f(costs: &[i32], start: usize, end: usize, k: usize) -> i32 {
    let n = costs.len();
    let mut memo = vec![vec![-1; 1 + k]; n];
    dfs(&costs, start, end, k, &mut memo)
}

fn dfs(costs: &[i32], idx: usize, end: usize, k: usize, memo: &mut [Vec<i32>]) -> i32 {
    if k == 0 {
        return 0;
    }
    if idx > end {
        return i32::MAX >> 1;
    }
    if memo[idx][k] > -1 {
        return memo[idx][k];
    }
    let pick = costs[idx] + dfs(costs, 2 + idx, end, k - 1, memo);
    let skip = dfs(costs, 1 + idx, end, k, memo);
    memo[idx][k] = pick.min(skip);
    memo[idx][k]
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
    fn test() {
        assert_eq!(min_operations(&[6, -7, 11, 13], 2), 11);
    }
}
