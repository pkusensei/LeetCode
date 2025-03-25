mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums1: &[i32], nums2: &[i32], k: i32) -> i64 {
    if k == 0 {
        return if nums1 == nums2 { 0 } else { -1 };
    }
    if nums1.iter().map(|&v| i64::from(v)).sum::<i64>()
        != nums2.iter().map(|&v| i64::from(v)).sum::<i64>()
    {
        return -1;
    }
    let mut res = 0;
    for (a, b) in nums1.iter().zip(nums2.iter()) {
        if a % k != b % k {
            return -1;
        }
        res += i64::from((a - b) / k).abs();
    }
    res / 2
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
        assert_eq!(min_operations(&[4, 3, 1, 4], &[1, 3, 7, 1], 3), 2);
        assert_eq!(min_operations(&[3, 8, 5, 2], &[2, 4, 1, 6], 1), -1);
    }

    #[test]
    fn test() {}
}
