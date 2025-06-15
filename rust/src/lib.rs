mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_cost(nums: &[i32], cost: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let f = |mut acc: Vec<i64>, &num| {
        acc.push(i64::from(num) + acc.last().unwrap_or(&0));
        acc
    };
    let prefn = nums.iter().fold(vec![0], f);
    let prefc = cost.iter().fold(vec![0], f);
    dfs(&prefn, &prefc, k.into(), 0, 0, &mut vec![-1; n])
}

fn dfs(
    prefixn: &[i64],
    prefixc: &[i64],
    k: i64,
    left: usize,
    right: usize,
    memo: &mut [i64],
) -> i64 {
    let n = prefixn.len();
    if right == n - 1 {
        return if left == n - 1 { 0 } else { i64::MAX >> 1 };
    }
    if memo[left] > -1 {
        return memo[left];
    }
    let num_sum = prefixn[1 + right] - prefixn[left];
    let cost_sum = prefixc[n - 1] - prefixc[left];
    let stay = dfs(prefixn, prefixc, k, left, 1 + right, memo);
    let end = (num_sum + k) * cost_sum + dfs(prefixn, prefixc, k, 1 + right, 1 + right, memo);
    memo[left] = stay.min(end);
    // dbg!(left, right);
    memo[left]
}

pub fn bottom_up(nums: &[i32], cost: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let k = i64::from(k);
    let f = |mut acc: Vec<i64>, &num| {
        acc.push(i64::from(num) + acc.last().unwrap_or(&0));
        acc
    };
    let prefn = nums.iter().fold(vec![0], f);
    let prefc = cost.iter().fold(vec![0], f);
    let ctotal = prefc[n];
    let mut dp = vec![i64::MAX; 1 + n];
    dp[0] = 0;
    for right in 1..=n {
        for left in 0..right {
            let cost_sum = prefc[right] - prefc[left];
            let curr = prefn[right] * cost_sum + k * (ctotal - prefc[left]);
            dp[right] = dp[right].min(dp[left] + curr);
        }
    }
    dp[n]
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
        assert_eq!(minimum_cost(&[3, 1, 4], &[4, 6, 6], 1), 110);
        assert_eq!(
            minimum_cost(
                &[4, 8, 5, 1, 14, 2, 2, 12, 1],
                &[7, 2, 8, 4, 2, 2, 1, 1, 2],
                7
            ),
            985
        );

        assert_eq!(bottom_up(&[3, 1, 4], &[4, 6, 6], 1), 110);
        assert_eq!(
            bottom_up(
                &[4, 8, 5, 1, 14, 2, 2, 12, 1],
                &[7, 2, 8, 4, 2, 2, 1, 1, 2],
                7
            ),
            985
        );
    }

    #[test]
    fn test() {}
}
