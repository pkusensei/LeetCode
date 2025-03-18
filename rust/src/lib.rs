mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_nice_subarray(nums: &[i32]) -> i32 {
    let mut res = 0;
    let mut all_bits = 0;
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        while all_bits & num > 0 {
            all_bits ^= nums[left];
            left += 1;
        }
        all_bits |= num;
        res = res.max(right + 1 - left);
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
        assert_eq!(longest_nice_subarray(&[1, 3, 8, 48, 10]), 3);
        assert_eq!(longest_nice_subarray(&[3, 1, 5, 11, 13]), 1);
    }

    #[test]
    fn test() {}
}
