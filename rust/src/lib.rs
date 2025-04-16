mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_sum(nums1: &[i32], nums2: &[i32]) -> i64 {
    let [mut zero1, mut sum1] = f(&nums1);
    let [mut zero2, mut sum2] = f(&nums2);
    if zero1 < zero2 {
        std::mem::swap(&mut zero1, &mut zero2);
        std::mem::swap(&mut sum1, &mut sum2);
    }
    // zero1 >= zero2
    if zero1 == 0 {
        if sum1 == sum2 { sum1 } else { -1 }
    } else if zero2 == 0 {
        if sum1 + zero1 <= sum2 { sum2 } else { -1 }
    } else {
        (sum1 + zero1).max(sum2 + zero2)
    }
}

fn f(nums: &[i32]) -> [i64; 2] {
    let mut zeros = 0;
    let mut sum = 0;
    for &num in nums {
        sum += i64::from(num);
        zeros += i64::from(num == 0);
    }
    [zeros, sum]
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
        assert_eq!(min_sum(&[3, 2, 0, 1, 0], &[6, 5, 0]), 12);
    }

    #[test]
    fn test() {}
}
