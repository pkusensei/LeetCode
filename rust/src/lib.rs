mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    nums1.sort_unstable();
    nums2.sort_unstable();
    let mut res = i32::MAX;
    for start in 0..=2 {
        let delta = nums2[0] - nums1[start];
        let mut skip = start;
        let mut i2 = 0;
        for v1 in nums1[start..].iter() {
            if nums2.get(i2).is_some_and(|&v| v == v1 + delta) {
                i2 += 1
            } else {
                skip += 1
            }
        }
        if skip == 2 {
            res = res.min(delta);
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
        assert_eq!(
            minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10]),
            -2
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            minimum_added_integer(vec![7, 2, 6, 8, 7], vec![7, 6, 5]),
            -1
        );
    }
}
