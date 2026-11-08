mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::cmp::Reverse;

#[allow(unused_imports)]
use helper::*;

// 1 3 2 4
// 1 2 3 4
pub fn count_suffix(mut nums: Vec<i32>, target: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return i32::from(nums[0] == target[0]);
    }
    let mut start = 0;
    while start < n && nums[start] == target[start] {
        start += 1;
    }
    nums[start..].sort_unstable_by_key(|&v| Reverse(v));
    let mut res = 0;
    if nums == target {
        res += 1;
        let right = start.min(n - 1);
        for w in nums[..=right].windows(2).rev() {
            if w[0] >= w[1] {
                res += 1
            } else {
                break;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(count_suffix(vec![1, 5, 6, 4, 6], vec![1, 5, 6, 6, 4]), 2);
        assert_eq!(count_suffix(vec![1, 2, 3], vec![3, 2, 1]), 1);
    }

    #[test]
    fn test() {
        assert_eq!(count_suffix(vec![3, 1, 5, 1], vec![3, 5, 1, 1]), 1);
        assert_eq!(count_suffix(vec![3, 1, 2], vec![1, 3, 2]), 0);
    }
}
