mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_slide(nums: &[i32], goal: i32) -> i32 {
    at_most(nums, goal) - at_most(nums, goal - 1)
}

fn at_most(nums: &[i32], goal: i32) -> i32 {
    let mut res = 0;
    let mut left = 0;
    let mut sum = 0;
    for (right, &num) in nums.iter().enumerate() {
        sum += num;
        while left <= right && sum > goal {
            sum -= nums[left];
            left += 1;
        }
        res += 1 + right - left;
    }
    res as i32
}

pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::from([(0, 1)]);
    let mut res = 0;
    let mut sum = 0;
    for &num in nums.iter() {
        sum += num;
        if let Some(f) = map.get(&(sum - goal)) {
            res += f;
        }
        *map.entry(sum).or_insert(0) += 1;
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
