mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::Itertools;
use std::collections::HashMap;

pub fn median_of_uniqueness_array(nums: &[i32]) -> i32 {
    let n = nums.len();
    let size = n * (1 + n) / 2;
    let median = (size + 1) / 2;
    let mut left = 1;
    let mut right = nums.iter().unique().count();
    while left < right {
        let mid = left.midpoint(right);
        if count(nums, mid) < median {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as i32
}

fn count(nums: &[i32], unique: usize) -> usize {
    let mut res = 0;
    let mut left = 0;
    let mut map = HashMap::new();
    for (right, &num) in nums.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        while map.len() > unique {
            let v = map.entry(nums[left]).or_insert(0);
            if *v == 1 {
                map.remove(&nums[left]);
            } else {
                *v -= 1;
            }
            left += 1;
        }
        res += right + 1 - left
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
        assert_eq!(median_of_uniqueness_array(&[1, 2, 3]), 1);
        assert_eq!(median_of_uniqueness_array(&[3, 4, 3, 4, 5]), 2);
        assert_eq!(median_of_uniqueness_array(&[4, 3, 5, 4]), 2);
    }

    #[test]
    fn test() {}
}
