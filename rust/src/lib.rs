mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(nums1: &[i32], nums2: &[i32]) -> i32 {
    let mut res = 0;
    for (i, &n1) in nums1.iter().enumerate() {
        let j = nums2.partition_point(|&v| v >= n1);
        if j.checked_sub(1).is_some_and(|j| j >= i) {
            res = res.max(j - 1 - i);
        }
    }
    res as _
}

pub fn with_two_ptrs(nums1: &[i32], nums2: &[i32]) -> i32 {
    let mut res = 0;
    let (n1, n2) = (nums1.len(), nums2.len());
    let [mut i, mut j] = [0, 0];
    while i < n1 && j < n2 {
        if nums1[i] > nums2[j] {
            i += 1;
        } else {
            res = res.max(j.saturating_sub(i));
            j += 1;
        }
    }
    res as _
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
        assert_eq!(max_distance(&[55, 30, 5, 4, 2], &[100, 20, 10, 10, 5]), 2);
        assert_eq!(max_distance(&[2, 2, 2], &[10, 10, 1]), 1);
        assert_eq!(max_distance(&[30, 29, 19, 5], &[25, 25, 25, 25, 25]), 2);

        assert_eq!(with_two_ptrs(&[55, 30, 5, 4, 2], &[100, 20, 10, 10, 5]), 2);
        assert_eq!(with_two_ptrs(&[2, 2, 2], &[10, 10, 1]), 1);
        assert_eq!(with_two_ptrs(&[30, 29, 19, 5], &[25, 25, 25, 25, 25]), 2);
    }

    #[test]
    fn test() {}
}
