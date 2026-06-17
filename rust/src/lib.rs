mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut dp = vec![vec![i32::MAX; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
        if 1 + i < n {
            dp[i][1 + i] = 0;
        }
    }
    for left in (0..n).rev() {
        for right in 2 + left..n {
            for mid in 1 + left..right {
                let curr = values[left] * values[right] * values[mid];
                dp[left][right] = dp[left][right].min(curr + dp[left][mid] + dp[mid][right]);
            }
        }
    }
    dp[0][n - 1]
    // dfs(&values, 0, n - 1)
}

fn dfs(nums: &[i32], left: usize, right: usize) -> i32 {
    if left + 2 > right {
        return 0;
    }
    let mut res = i32::MAX;
    for mid in 1 + left..right {
        let curr =
            nums[left] * nums[right] * nums[mid] + dfs(nums, left, mid) + dfs(nums, mid, right);
        res = res.min(curr)
    }
    res
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
