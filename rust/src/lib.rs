mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_length(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut res = 1 + nums.len();
    let mut curr = 0;
    let mut freq = HashMap::new();
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        let f = freq.entry(num).or_insert(0);
        if *f == 0 {
            curr += num;
        }
        *f += 1;
        while curr >= k {
            res = res.min(right + 1 - left);
            let prev = freq.entry(nums[left]).or_insert(0);
            if *prev == 1 {
                curr -= nums[left];
            }
            *prev -= 1;
            left += 1;
        }
    }
    if res > nums.len() { -1 } else { res as i32 }
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
