mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n < 2 {
        return vec![*nums.first().unwrap_or(&0)];
    }
    nums.sort_unstable();
    // (length, prev)
    let mut dp = vec![(1, None::<usize>); n];
    let mut max_len = 1;
    let mut max_idx = 0;
    for right in 1..n {
        for left in 0..right {
            if nums[right] % nums[left] == 0 && nums[right] > nums[left] {
                let len = 1 + dp[left].0;
                if len > dp[right].0 {
                    dp[right].0 = len;
                    dp[right].1 = Some(left);
                }
            }
        }
        if dp[right].0 > max_len {
            max_len = dp[right].0;
            max_idx = right;
        }
    }
    let mut res = vec![nums[max_idx]];
    while let Some(prev) = dp[max_idx].1 {
        res.push(nums[prev]);
        max_idx = prev;
    }
    res.reverse();
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
