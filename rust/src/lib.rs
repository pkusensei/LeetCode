mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    backtrack(&nums, k, 0, &mut HashSet::new())
}

fn backtrack(nums: &[i32], k: i32, idx: usize, set: &mut HashSet<i32>) -> i32 {
    if idx >= nums.len() {
        return i32::from(!set.is_empty());
    }
    let mut res = backtrack(nums, k, 1 + idx, set);
    if !set.contains(&(nums[idx] - k)) {
        set.insert(nums[idx]);
        res += backtrack(nums, k, 1 + idx, set);
        set.remove(&nums[idx]);
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
        assert_eq!(beautiful_subsets(vec![2, 4, 6], 2), 4);
        assert_eq!(beautiful_subsets(vec![1], 1), 1);
    }

    #[test]
    fn test() {}
}
