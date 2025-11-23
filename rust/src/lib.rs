mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    // (xor, even_odd) - idx
    let mut prefix = HashMap::from([((0, 0), -1)]);
    let mut curr_xor = 0;
    let mut curr_even_odd = 0;
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        curr_xor ^= num;
        if num & 1 == 1 {
            curr_even_odd -= 1
        } else {
            curr_even_odd += 1
        }
        if let Some(prev) = prefix.get(&(curr_xor, curr_even_odd)) {
            res = res.max(idx as i32 - prev)
        } else {
            prefix.insert((curr_xor, curr_even_odd), idx as i32);
        }
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
