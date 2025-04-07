mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_non_decreasing_length(nums1: &[i32], nums2: &[i32]) -> i32 {
    let n = nums1.len();
    let [mut end1, mut end2] = [1, 1];
    let mut res = 1;
    for idx in 1..n {
        let [mut curr1, mut curr2] = [1, 1];
        if nums1[idx] >= nums1[idx - 1] {
            curr1 = curr1.max(1 + end1);
        }
        if nums1[idx] >= nums2[idx - 1] {
            curr1 = curr1.max(1 + end2);
        }
        if nums2[idx] >= nums1[idx - 1] {
            curr2 = curr2.max(1 + end1);
        }
        if nums2[idx] >= nums2[idx - 1] {
            curr2 = curr2.max(1 + end2);
        }
        res = res.max(curr1).max(curr2);
        end1 = curr1;
        end2 = curr2;
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
        assert_eq!(max_non_decreasing_length(&[2, 3, 1], &[1, 2, 1]), 2);
        assert_eq!(max_non_decreasing_length(&[1, 3, 2, 1], &[2, 2, 3, 4],), 4);
        assert_eq!(max_non_decreasing_length(&[1, 1], &[2, 2]), 2);
    }

    #[test]
    fn test() {
        assert_eq!(
            max_non_decreasing_length(&[3, 19, 13, 19], &[20, 18, 7, 14]),
            2
        );
    }
}
