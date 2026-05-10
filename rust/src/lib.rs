mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut dp = vec![-1; n];
    dp[0] = 0;
    for right in 1..n {
        for left in 0..right {
            if nums[right].abs_diff(nums[left]) <= target as u32 && dp[left] > -1 {
                dp[right] = dp[right].max(1 + dp[left]);
            }
        }
    }
    dp[n - 1]
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
