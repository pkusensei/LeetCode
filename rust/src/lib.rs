mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_partition(nums: &[i32]) -> bool {
    let n = nums.len();
    dfs(nums, &mut vec![None; 1 + n])
}

fn dfs(nums: &[i32], memo: &mut [Option<bool>]) -> bool {
    match nums {
        [] => true,
        [_] => false,
        [a, b] => a == b,
        [a, b, c, tail @ ..] => {
            let n = nums.len();
            if let Some(v) = memo[n] {
                return v;
            }
            let res = (a == b && dfs(&nums[2..], memo))
                || (a == b && b == c && dfs(tail, memo))
                || (a + 1 == *b && b + 1 == *c && dfs(tail, memo));
            memo[n] = Some(res);
            res
        }
    }
}

pub fn with_dp(nums: &[i32]) -> bool {
    let n = nums.len();
    if n == 1 {
        return false;
    }
    let mut dp = [true, false, if n > 1 { nums[0] == nums[1] } else { false }];
    for idx in 2..n {
        let curr = (nums[idx] == nums[idx - 1] && dp[1])
            || (nums[idx] == nums[idx - 1] && nums[idx - 1] == nums[idx - 2] && dp[0])
            || (nums[idx] == 1 + nums[idx - 1] && nums[idx - 1] == 1 + nums[idx - 2] && dp[0]);
        dp = [dp[0], dp[1], curr];
    }
    dp[2]
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
        assert!(valid_partition(&[4, 4, 4, 5, 6]));
        assert!(!valid_partition(&[1, 1, 1, 2]));

        assert!(with_dp(&[4, 4, 4, 5, 6]));
        assert!(!with_dp(&[1, 1, 1, 2]));
    }

    #[test]
    fn test() {}
}
