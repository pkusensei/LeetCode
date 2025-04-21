mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_set_size(nums1: &[i32], nums2: &[i32]) -> i32 {
    use std::collections::HashSet;
    let n = nums1.len();
    let [set1, set2] = [&nums1, &nums2].map(|v| v.iter().copied().collect::<HashSet<_>>());
    let common = set1.intersection(&set2).count();
    let n1 = (set1.len() - common).min(n / 2);
    let n2 = (set2.len() - common).min(n / 2);
    (n1 + n2 + common).min(n) as i32
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
        assert_eq!(maximum_set_size(&[1, 2, 1, 2], &[1, 1, 1, 1]), 2);
        assert_eq!(
            maximum_set_size(&[1, 2, 3, 4, 5, 6], &[2, 3, 2, 3, 2, 3]),
            5
        );
        assert_eq!(
            maximum_set_size(&[1, 1, 2, 2, 3, 3], &[4, 4, 5, 5, 6, 6]),
            6
        );
    }

    #[test]
    fn test() {}
}
