mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn knapsack(prices: &[i32], k: i32) -> i64 {
    let k = k as usize;
    let mut normal = vec![0; 1 + k];
    let mut buy = vec![i64::MIN >> 2; 1 + k];
    let mut short = vec![0; 1 + k];
    for num in prices.iter().map(|&v| i64::from(v)) {
        for i in (1..=k).rev() {
            normal[i] = normal[i].max(buy[i] + num).max(short[i] - num);
            buy[i] = buy[i].max(normal[i - 1] - num);
            short[i] = short[i].max(normal[i - 1] + num);
        }
    }
    normal[k]
}

pub fn maximum_profit(prices: &[i32], k: i32) -> i64 {
    let n = prices.len();
    let k = k as usize;
    let mut memo = vec![vec![[[-1; 2]; 2]; 1 + k]; 1 + n];
    dfs(&prices, k, 0, 0, &mut memo)
}

fn dfs(nums: &[i32], k: usize, type_: usize, run: usize, memo: &mut [Vec<[[i64; 2]; 2]>]) -> i64 {
    if k == 0 || nums.is_empty() {
        return if run == 0 { 0 } else { i64::MIN >> 4 };
    }
    let n = nums.len();
    if memo[n][k][type_][run] > -1 {
        return memo[n][k][type_][run];
    }
    let skip = dfs(&nums[1..], k, type_, run, memo);
    let num = i64::from(nums[0]);
    let take = if run == 0 {
        let buy = -num + dfs(&nums[1..], k, 0, 1 - run, memo);
        let short = num + dfs(&nums[1..], k, 1, 1 - run, memo);
        buy.max(short)
    } else {
        if type_ == 0 {
            num + dfs(&nums[1..], k - 1, type_, 1 - run, memo)
        } else {
            -num + dfs(&nums[1..], k - 1, type_, 1 - run, memo)
        }
    };
    memo[n][k][type_][run] = skip.max(take);
    memo[n][k][type_][run]
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

        assert_eq!(knapsack(&[1, 7, 9, 8, 2], 2), 14);
        assert_eq!(knapsack(&[12, 16, 19, 19, 8, 1, 19, 13, 9], 3), 36);
    }

    #[test]
    fn test() {}
}
