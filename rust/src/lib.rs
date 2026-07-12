mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_consistent_columns(grid: Vec<Vec<i32>>, limit: i32) -> i32 {
    let cols = grid[0].len();
    let mut dp = vec![1; cols];
    let mut res = 1;
    for right in 1..cols {
        for left in 0..right {
            if grid
                .iter()
                .all(|r| r[right].abs_diff(r[left]) <= limit as u32)
            {
                dp[right] = dp[right].max(1 + dp[left]);
                res = res.max(dp[right])
            }
        }
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
