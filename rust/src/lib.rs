mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_balanced(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let n = nums.len();
    let mut res = 0;
    for left in 0..n {
        let mut odds = HashSet::new();
        let mut evens = HashSet::new();
        for right in left..n {
            if nums[right] & 1 == 1 {
                odds.insert(nums[right]);
            } else {
                evens.insert(nums[right]);
            }
            if odds.len() == evens.len() {
                res = res.max(right + 1 - left);
            }
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
    fn basics() {}

    #[test]
    fn test() {}
}
