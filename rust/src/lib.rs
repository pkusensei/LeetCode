mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

// negate = sum + 2(-x)
// sum+(-2x).rem_euclid(k) = prev_sum%k
pub fn longest_subarray(nums: &[i32], k: i32) -> i32 {
    let mut prev_sum = HashMap::from([(0, -1)]);
    let mut prev_nums = HashMap::new();
    let mut sum = 0;
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        sum = (sum + num).rem_euclid(k);
        prev_nums.insert((-2 * num).rem_euclid(k), idx as i32);
        if let Some(&prev) = prev_sum.get(&sum) {
            res = res.max(idx as i32 - prev);
        }
        for (rem, &v) in prev_nums.iter() {
            if let Some(&prev) = prev_sum.get(&((sum + rem) % k))
                && prev < v
            {
                res = res.max(idx as i32 - prev)
            }
        }
        prev_sum.entry(sum).or_insert(idx as i32);
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
        assert_eq!(longest_subarray(&[9, -12], 4), 1);
    }
}
