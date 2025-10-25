mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums1: &[i32], nums2: &[i32]) -> i64 {
    let append = *nums2.last().unwrap();
    let mut res = 1;
    let mut ops = u32::MAX;
    for (&v1, &v2) in nums1.iter().zip(nums2.iter()) {
        res += i64::from(v1.abs_diff(v2));
        let min = v1.min(v2);
        let max = v1.max(v2);
        if (min..=max).contains(&append) {
            ops = 0;
        } else if ops > 0 {
            ops = ops.min(v1.abs_diff(append).min(v2.abs_diff(append)));
        }
    }
    res + i64::from(ops)
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
        assert_eq!(min_operations(&[2, 8], &[1, 7, 3]), 4);
        assert_eq!(min_operations(&[1, 3, 6], &[2, 4, 5, 3]), 4);
    }

    #[test]
    fn test() {}
}
