mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarray_length(nums: &[i32], k: i32) -> i32 {
    let mut res = 0;
    let mut map = std::collections::HashMap::new();
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        while map[&num] > k {
            map.entry(nums[left]).and_modify(|v| *v -= 1);
            left += 1;
        }
        res = res.max(right - left + 1);
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
        assert_eq!(max_subarray_length(&[1, 2, 3, 1, 2, 3, 1, 2], 2), 6);
        assert_eq!(max_subarray_length(&[1, 2, 1, 2, 1, 2, 1, 2], 1), 2);
        assert_eq!(max_subarray_length(&[5, 5, 5, 5, 5, 5, 5], 4), 4);
    }

    #[test]
    fn test() {}
}
