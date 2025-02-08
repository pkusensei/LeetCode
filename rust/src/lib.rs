mod dsu;
mod helper;
mod trie;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;

pub fn min_absolute_sum_diff(nums1: &[i32], nums2: &[i32]) -> i32 {
    let set: BTreeSet<_> = nums1.iter().copied().collect();
    let sum: u64 = nums1
        .iter()
        .zip(nums2.iter())
        .map(|(a, b)| u64::from(a.abs_diff(*b)))
        .sum();
    let mut res = sum;
    for (&a, &b) in nums1.iter().zip(nums2.iter()) {
        if a == b {
            continue;
        }
        if let Some(&upper) = set.range(b..).next() {
            res = res.min(sum + u64::from(upper.abs_diff(b)) - u64::from(a.abs_diff(b)));
        }
        if let Some(&lower) = set.range(..b).next_back() {
            res = res.min(sum + u64::from(lower.abs_diff(b)) - u64::from(a.abs_diff(b)));
        }
    }
    (res % 1_000_000_007) as _
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
    fn test() {}
}
