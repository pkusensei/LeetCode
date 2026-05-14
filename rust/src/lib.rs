mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    at_most(&nums, k) - at_most(&nums, k - 1)
}

fn at_most(nums: &[i32], k: i32) -> i32 {
    let mut freq = HashMap::new();
    let mut left = 0;
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        *freq.entry(num).or_insert(0) += 1;
        while freq.len() > k as usize {
            let Some(v) = freq.get_mut(&nums[left]) else {
                unreachable!()
            };
            *v -= 1;
            if *v == 0 {
                freq.remove(&nums[left]);
            }
            left += 1;
        }
        res += right + 1 - left;
    }
    res as i32
}

pub fn single_pass(nums: &[i32], k: i32) -> i32 {
    let k = k as usize;
    let mut freq = HashMap::new();
    let mut res = 0;
    let mut left = 0;
    let mut mid = 0;
    for (_right, &num) in nums.iter().enumerate() {
        *freq.entry(num).or_insert(0) += 1;
        while freq.len() > k {
            let v = freq.entry(nums[mid]).or_insert(0);
            *v -= 1;
            if *v == 0 {
                freq.remove(&nums[mid]);
            }
            mid += 1;
            left = mid;
        }
        if freq.len() == k {
            while freq.len() == k {
                let v = freq.entry(nums[mid]).or_insert(0);
                if *v == 1 {
                    break;
                }
                *v -= 1;
                mid += 1;
            }
            res += 1 + mid - left;
        }
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
        assert!(is_good(&[1, 3, 3, 2]));
    }

    #[test]
    fn test() {}
}
