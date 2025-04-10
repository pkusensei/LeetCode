mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_equal_subarray(nums: &[i32], k: i32) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::<_, Vec<_>>::new();
    for (idx, &num) in nums.iter().enumerate() {
        map.entry(num).or_default().push(idx as i32);
    }
    let mut res = 0;
    for ids in map.values() {
        let mut left = 0;
        let mut curr = 0;
        for (right, &val) in ids.iter().enumerate() {
            while val - ids[left] - (right - left) as i32 > k {
                left += 1;
            }
            curr = curr.max(right + 1 - left);
        }
        res = res.max(curr as i32);
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
        assert_eq!(longest_equal_subarray(&[1, 3, 2, 3, 1, 3], 3), 3);
        assert_eq!(longest_equal_subarray(&[1, 1, 2, 2, 1, 1], 2), 4);
    }

    #[test]
    fn test() {}
}
