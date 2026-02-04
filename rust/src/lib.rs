mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_trionic(nums: &[i32]) -> i64 {
    let mut res = i64::MIN;
    // [start, inc, inc_dec, dec_inc]
    let mut dp = [i64::MIN >> 1; 4];
    dp[0] = i64::from(nums[0]);
    for w in nums.windows(2) {
        let mut curr = [i64::MIN >> 1; 4];
        let num = i64::from(w[1]);
        curr[0] = num; // Attempt to start here
        match w[0].cmp(&w[1]) {
            std::cmp::Ordering::Less => {
                curr[1] = num + dp[1].max(dp[0]);
                curr[3] = num + dp[3].max(dp[2]);
                curr[0] = num.max(num + dp[0]); // Sneak in Kadane's algo
            }
            std::cmp::Ordering::Equal => (), // reset streak
            std::cmp::Ordering::Greater => curr[2] = num + dp[2].max(dp[1]),
        }
        dp = curr;
        res = res.max(dp[3]);
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
    fn test() {
        assert_eq!(
            max_sum_trionic(&[-754, 167, -172, 202, 735, -941, 992]),
            988
        );
    }
}
