mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_jumps(nums: &[i32], target: i32) -> i32 {
    let n = nums.len();
    let mut dp = vec![-1; n];
    dp[0] = 0;
    for end in 0..n {
        for prev in 0..end {
            if nums[end].abs_diff(nums[prev]) <= target as u32 && dp[prev] >= 0 {
                dp[end] = dp[end].max(1 + dp[prev]);
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(maximum_jumps(&[1, 3, 6, 4, 1, 2], 2), 3);
        assert_eq!(maximum_jumps(&[1, 3, 6, 4, 1, 2], 3), 5);
        assert_eq!(maximum_jumps(&[1, 3, 6, 4, 1, 2], 0), -1);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_jumps(&[0, 2, 1, 3], 1), -1);
    }
}
