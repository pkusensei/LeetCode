mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximums_spliced_array(nums1: &[i32], nums2: &[i32]) -> i32 {
    let [mut best1, mut best2] = [i32::MIN; 2];
    let [mut curr1, mut curr2] = [0, 0];
    let [mut sum1, mut sum2] = [0, 0];
    for (v1, v2) in nums1.iter().zip(nums2) {
        sum1 += v1;
        sum2 += v2;
        curr1 = (v1 - v2).max(curr1 + v1 - v2);
        curr2 = (v2 - v1).max(curr2 + v2 - v1);
        best1 = best1.max(curr1);
        best2 = best2.max(curr2);
    }
    (sum1 + best2.max(0)).max(sum2 + best1.max(0))
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
        assert_eq!(maximums_spliced_array(&[60, 60, 60], &[10, 90, 10]), 210);
        assert_eq!(
            maximums_spliced_array(&[20, 40, 20, 70, 30], &[50, 20, 50, 40, 20]),
            220
        );
        assert_eq!(maximums_spliced_array(&[7, 11, 13], &[1, 1, 1]), 31);
    }

    #[test]
    fn test() {
        assert_eq!(
            maximums_spliced_array(&[10, 20, 50, 15, 30, 10], &[40, 20, 10, 100, 10, 10]),
            230
        );
    }
}
