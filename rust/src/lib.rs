mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn merge_stones(stones: &[i32], k: i32) -> i32 {
    let n = stones.len();
    let k = k as usize;
    // To merge `n` into `1`
    // `1` has to merge with ( `n-1` merge into `k-1` )
    if (n - 1) % (k - 1) > 0 {
        return -1;
    }
    let prefix = stones.iter().fold(vec![], |mut acc, v| {
        acc.push(v + acc.last().unwrap_or(&0));
        acc
    });
    tabulation(&prefix, k)
    // let mut memo = vec![vec![vec![-1; 1 + k]; n]; n];
    // dfs(&prefix, k, 0, n - 1, 1, &mut memo)
}

fn dfs(
    prefix: &[i32],
    k: usize,
    left: usize,
    right: usize,
    target: usize,
    memo: &mut [Vec<Vec<i32>>],
) -> i32 {
    if left == right {
        return if target == 1 { 0 } else { i32::MAX >> 2 };
    }
    if memo[left][right][target] > -1 {
        return memo[left][right][target];
    }
    let res = if target == 1 {
        dfs(prefix, k, left, right, k, memo)
            + (prefix[right] - if left > 0 { prefix[left - 1] } else { 0 })
    } else {
        let mut res = i32::MAX;
        for mid in left..right {
            res = res.min(
                dfs(prefix, k, left, mid, 1, memo)
                    + dfs(prefix, k, 1 + mid, right, target - 1, memo),
            )
        }
        res
    };
    memo[left][right][target] = res;
    res
}

fn tabulation(prefix: &[i32], k: usize) -> i32 {
    const INF: i32 = i32::MAX >> 2;
    let n = prefix.len();
    let mut dp = vec![vec![vec![INF; 1 + k]; n]; n];
    for i in 0..n {
        dp[i][i][1] = 0;
    }
    for len in 2..=n {
        for left in 0..=n - len {
            for target in (1..=k).rev() {
                let right = left + len - 1;
                dp[left][right][target] = if target == 1 {
                    let cost = prefix[right] - if left > 0 { prefix[left - 1] } else { 0 };
                    (dp[left][right][k] + cost).min(INF) // prevents overflow
                } else {
                    let mut curr = INF;
                    for mid in left..right {
                        curr = curr.min(dp[left][mid][1] + dp[1 + mid][right][target - 1]);
                    }
                    curr
                };
            }
        }
    }
    dp[0][n - 1][1]
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
