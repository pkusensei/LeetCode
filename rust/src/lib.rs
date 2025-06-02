mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_subsequence(nums: &[i32]) -> i32 {
    const N: usize = 301;
    let n = nums.len();
    let mut dp = vec![[0; N]; n];
    let mut subs = 1;
    for idx in 0..n {
        let mut suf_max = [0; 1 + N];
        for d in (0..N).rev() {
            suf_max[d] = suf_max[1 + d].max(dp[idx][d]);
        }
        for right in 1 + idx..n {
            let diff = nums[idx].abs_diff(nums[right]) as usize;
            dp[right][diff] = dp[right][diff].max(1 + suf_max[diff]);
            subs = subs.max(dp[right][diff]);
        }
    }
    1 + subs
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
        assert_eq!(longest_subsequence(&[16, 6, 3]), 3);
        assert_eq!(longest_subsequence(&[6, 5, 3, 4, 2, 1]), 4);
        assert_eq!(longest_subsequence(&[10, 20, 10, 19, 10, 20]), 5);
    }

    #[test]
    fn test() {}
}
