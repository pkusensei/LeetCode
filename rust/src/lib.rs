mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut dp = [1, 1]; // [no_inc, inc]
    let mut res = 1;
    for w in nums.windows(2) {
        let mut curr = [1, 1];
        match w[1] - w[0] {
            0 => {
                curr[0] = curr[0].max(dp[0]);
                curr[1] = curr[1].max(dp[1]).max(1 + dp[0]);
            }
            1 => {
                curr[0] = curr[0].max(1 + dp[0]);
                curr[1] = curr[1].max(1 + dp[1]);
            }
            2 => curr[0] = curr[0].max(1 + dp[1]),
            _ => (),
        }
        res = res.max(curr[0]).max(curr[1]);
        dp = curr;
    }
    res
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
    fn test() {
        {
            let nums = vec![4, 12, 8, 14, 14, 12, 11, 20, 17, 17, 4, 10, 10, 18, 1, 16];
            assert_eq!(max_selected_elements(nums), 12);
        }
    }
}
