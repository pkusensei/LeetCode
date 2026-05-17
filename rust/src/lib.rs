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

pub fn smallest_unique_subarray(nums: &[i32]) -> i32 {
    let mut left = 1;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if check(&nums, mid) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left as i32
}

fn check(nums: &[i32], mid: usize) -> bool {
    const M: i64 = 1_000_000_007;
    const BASE: i64 = 215215;
    let mut freq = HashMap::new();
    let mut val = 0;
    let mut pow = 1;
    for &num in &nums[..mid] {
        val = (val * BASE + i64::from(num)) % M;
        pow = pow * BASE % M;
    }
    freq.insert(val, 1);
    for i in mid..nums.len() {
        val = (val * BASE + i64::from(nums[i]) - i64::from(nums[i - mid]) * pow % M).rem_euclid(M);
        *freq.entry(val).or_insert(0) += 1;
    }
    freq.into_values().any(|f| f == 1)
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
    fn basics() {
        assert_eq!(smallest_unique_subarray(&[3, 3, 3]), 3);
        assert_eq!(smallest_unique_subarray(&[1, 1, 2, 2, 1]), 2);
    }

    #[test]
    fn test() {}
}
