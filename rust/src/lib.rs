mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_alternating_subarray(nums: &[i32], threshold: i32) -> i32 {
    let n = nums.len();
    let mut left = 0;
    let mut res = 0;
    while left < n {
        while nums.get(left).is_some_and(|&v| v & 1 == 1 || v > threshold) {
            left += 1;
        }
        if left >= n {
            break;
        }
        let mut right = left;
        while nums
            .get(1 + right)
            .is_some_and(|&v| v <= threshold && v & 1 != nums[right] & 1)
        {
            right += 1;
        }
        res = res.max(right + 1 - left);
        left += 1;
    }
    res as i32
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
        assert_eq!(longest_alternating_subarray(&[3, 2, 5, 4], 5), 3);
        assert_eq!(longest_alternating_subarray(&[1, 2], 2), 1);
        assert_eq!(longest_alternating_subarray(&[2, 3, 4, 5], 4), 3);
    }

    #[test]
    fn test() {}
}
