mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

// nums1[i] - nums1[j] <= nums2[i] - nums2[j] + diff
// nums1[i] - nums2[i] <= nums1[j] - nums2[j] + diff
pub fn number_of_pairs(nums1: &[i32], nums2: &[i32], diff: i32) -> i64 {
    let mut res = 0;
    let mut sorted = Vec::with_capacity(nums1.len());
    for (&a, &b) in nums1.iter().zip(nums2.iter()) {
        res += sorted.partition_point(|&v| v <= a - b + diff) as i64;
        let i = sorted.partition_point(|&v| v < a - b);
        if i == sorted.len() {
            sorted.push(a - b);
        } else {
            sorted.insert(i, a - b);
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
        assert_eq!(number_of_pairs(&[3, 2, 5], &[2, 2, 1], 1), 3);
        assert_eq!(number_of_pairs(&[3, -1], &[-2, 2], -1), 0);
    }

    #[test]
    fn test() {}
}
