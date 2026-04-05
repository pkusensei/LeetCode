mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_greedy(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut res = 0;
    let mut pref_odd = vec![0; n];
    for i in (1..n - 1).step_by(2) {
        let curr = (1 + nums[i - 1].max(nums[1 + i]) - nums[i]).max(0);
        res += i64::from(curr);
        pref_odd[i] = res;
    }
    if n & 1 == 1 {
        return res;
    }
    let mut suf_even = vec![0; n];
    let mut temp = 0;
    for i in (2..n - 1).rev().step_by(2) {
        let curr = (1 + nums[i - 1].max(nums[1 + i]) - nums[i]).max(0);
        temp += i64::from(curr);
        suf_even[i] = temp;
    }
    res = res.min(temp);
    for i in (2..n - 2).step_by(2) {
        res = res.min(pref_odd[i - 1] + suf_even[2 + i]);
    }
    res
}

pub fn min_increase(nums: &[i32]) -> i64 {
    let n = nums.len();
    if n & 1 == 1 {
        return f(&nums, 1);
    }
    // let mut memo = vec![[-1; 2]; n];
    // dfs(&nums, 1, 0, &mut memo)
    with_dp(&nums)
}

fn with_dp(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut dp = vec![[i64::MAX >> 2; 2]; n];
    dp[n - 1].fill(0);
    for i in (1..n - 1).rev() {
        let curr = i64::from((1 + nums[i - 1].max(nums[1 + i]) - nums[i]).max(0));
        for skip in [0, 1] {
            dp[i][skip] = dp[i][skip].min(curr + dp.get(2 + i).map(|v| v[skip]).unwrap_or(0));
        }
        dp[i][0] = dp[i][0].min(dp[1 + i][1]);
    }
    dp[1][0].min(dp[1][1])
}

fn dfs(nums: &[i32], idx: usize, skip: usize, memo: &mut [[i64; 2]]) -> i64 {
    let n = nums.len();
    if idx >= n - 1 {
        return 0;
    }
    if memo[idx][skip] > -1 {
        return memo[idx][skip];
    }
    let curr = i64::from((1 + nums[idx - 1].max(nums[1 + idx]) - nums[idx]).max(0));
    // take
    let mut res = curr + dfs(nums, 2 + idx, skip, memo);
    if skip == 0 {
        res = res.min(dfs(nums, 1 + idx, 1, memo));
    }
    memo[idx][skip] = res;
    res
}

fn f(nums: &[i32], start: usize) -> i64 {
    let mut ops = 0;
    let n = nums.len();
    for i in (start..n - 1).step_by(2) {
        ops += i64::from((1 + nums[i - 1].max(nums[1 + i])).max(nums[i]) - nums[i]);
    }
    ops
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
        assert_eq!(min_increase(&[12, 23, 13, 17, 21, 3]), 0);
        assert_eq!(min_increase(&[21, 12, 18, 19]), 2);

        assert_eq!(with_greedy(&[12, 23, 13, 17, 21, 3]), 0);
        assert_eq!(with_greedy(&[21, 12, 18, 19]), 2);
    }
}
