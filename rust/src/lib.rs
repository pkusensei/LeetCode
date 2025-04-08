mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let n = nums.len();
    let count = nums.iter().unique().count();
    let mut map = HashMap::new();
    let mut left = 0;
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        *map.entry(num).or_insert(0) += 1;
        while map.len() == count {
            res += n - right;
            *map.entry(nums[left]).or_insert(0) -= 1;
            if map[&nums[left]] == 0 {
                map.remove(&nums[left]);
            }
            left += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
