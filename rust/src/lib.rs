mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_profit(prices: &[i32], k: i32) -> i64 {
    let mut memo = vec![vec![[[-1; 2]; 2]; 1 + k as usize]; prices.len()];
    dfs(prices, 0, k as usize, 0, 0, &mut memo)
}

fn dfs(
    nums: &[i32],
    idx: usize,
    k: usize,
    type_: usize,
    running: usize,
    memo: &mut [Vec<[[i64; 2]; 2]>],
) -> i64 {
    if k == 0 || idx >= nums.len() {
        return if running == 0 { 0 } else { i64::MIN >> 1 };
    }
    if memo[idx][k][type_][running] > -1 {
        return memo[idx][k][type_][running];
    }
    let skip = dfs(nums, 1 + idx, k, type_, running, memo);
    let val = i64::from(nums[idx]);
    let take = if running == 0 {
        let normal = -val + dfs(nums, 1 + idx, k, 0, 1, memo);
        let short = val + dfs(nums, 1 + idx, k, 1, 1, memo);
        normal.max(short)
    } else {
        if type_ == 0 {
            val + dfs(nums, 1 + idx, k - 1, type_, 0, memo)
        } else {
            -val + dfs(nums, 1 + idx, k - 1, type_, 0, memo)
        }
    };
    memo[idx][k][type_][running] = skip.max(take);
    memo[idx][k][type_][running]
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
    fn basics() {
        assert_eq!(maximum_profit(&[1, 7, 9, 8, 2], 2), 14);
        assert_eq!(maximum_profit(&[12, 16, 19, 19, 8, 1, 19, 13, 9], 3), 36);
    }

    #[test]
    fn test() {}
}
