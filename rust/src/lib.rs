mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_steps(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut dp = vec![0; n];
    let mut stack = vec![];
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate().rev() {
        while stack.last().is_some_and(|&i| nums[i] < num) {
            let i = stack.pop().unwrap();
            dp[idx] = (1 + dp[idx]).max(dp[i]);
            res = res.max(dp[idx])
        }
        stack.push(idx);
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
    fn basics() {}

    #[test]
    fn test() {
        assert_eq!(total_steps(&[5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]), 3);
        assert_eq!(total_steps(&[4, 5, 7, 7, 13]), 0);
    }
}
