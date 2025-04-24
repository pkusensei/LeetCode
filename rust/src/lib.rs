mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.sort_unstable();
    // [no_increase, increase]
    let mut dp = vec![[1, 1]; n];
    let mut res = 1;
    for right in 1..n {
        for left in (0..right).rev() {
            match nums[right] - nums[left] {
                0 => {
                    // [2, 2] => interchangable
                    // Or increase current to [3]
                    dp[right][1] = dp[right][1].max(dp[left][1]).max(1 + dp[left][0]);
                    dp[right][0] = dp[right][0].max(dp[left][0]);
                }
                1 => {
                    dp[right][1] = dp[right][1].max(1 + dp[left][1]);
                    dp[right][0] = dp[right][0].max(1 + dp[left][0]);
                }
                2 => dp[right][0] = dp[right][0].max(1 + dp[left][1]), // [1, 3]
                _ => break,
            }
            res = res.max(dp[right][0]).max(dp[right][1]);
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
        assert_eq!(max_selected_elements(vec![2, 1, 5, 1, 1]), 3);
        assert_eq!(max_selected_elements(vec![1, 4, 7, 10]), 1);
    }

    #[test]
    fn test() {}
}
