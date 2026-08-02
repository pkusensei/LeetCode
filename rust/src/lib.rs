mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game(piles: Vec<i32>) -> bool {
    let n = piles.len();
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = piles[i];
    }
    for len in 2..=n {
        for left in 0..=n - len {
            let right = left + len - 1;
            dp[left][right] =
                (piles[left] - dp[1 + left][right]).max(piles[right] - dp[left][right - 1]);
        }
    }
    dp[0][n - 1] > 0
    // dfs(&piles, 0, n - 1) > 0
}

fn dfs(nums: &[i32], left: usize, right: usize) -> i32 {
    if left == right {
        return nums[left];
    }
    let a = nums[left] - dfs(nums, 1 + left, right);
    let b = nums[right] - dfs(nums, left, right - 1);
    a.max(b)
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
