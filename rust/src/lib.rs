mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums1: &[i32], nums2: &[i32]) -> i32 {
    let n = nums1.len();
    let mut dp1 = 0;
    let mut dp2 = 0;
    let max1 = nums1[n - 1];
    let max2 = nums2[n - 1];
    for (&num1, &num2) in nums1.iter().zip(nums2.iter()) {
        if num1.min(num2) > max1.min(max2) || num1.max(num2) > max1.max(max2) {
            return -1;
        }
        if num1 > max1 || num2 > max2 {
            dp1 += 1
        }
        if num1 > max2 || num2 > max1 {
            dp2 += 1
        }
    }
    dp1.min(dp2)
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
        assert_eq!(min_operations(&[1, 2, 7], &[4, 5, 3],), 1);
        assert_eq!(min_operations(&[2, 3, 4, 5, 9], &[8, 8, 4, 4, 4]), 2);
        assert_eq!(min_operations(&[1, 5, 4], &[2, 5, 3]), -1)
    }

    #[test]
    fn test() {
        assert_eq!(min_operations(&[8, 6, 6, 6, 7, 8], &[5, 8, 8, 8, 7, 7]), 2);
    }
}
