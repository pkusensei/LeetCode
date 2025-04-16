mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_increment_operations(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let mut dp = vec![0; n];
    for (i, &num) in nums.iter().enumerate() {
        if i < 3 {
            dp[i] = i64::from(k - num).max(0);
        } else {
            dp[i] = i64::from(k - num).max(0) + dp[i - 3..i].iter().min().unwrap_or(&0);
        }
    }
    *dp[n - 3..].iter().min().unwrap()
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
        assert_eq!(min_increment_operations(&[2, 3, 0, 0, 2], 4), 3);
        assert_eq!(min_increment_operations(&[0, 1, 3, 3], 5), 2);
        assert_eq!(min_increment_operations(&[1, 1, 2], 1), 0);
    }

    #[test]
    fn test() {}
}
