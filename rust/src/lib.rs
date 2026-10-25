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

// (1+55)*55/2 = 1540
pub fn count_staircase(nums: &[i32]) -> i32 {
    let n = nums.len();
    (0..n).map(|i| f(&nums[i..])).sum()
}

const LENGTHS: [usize; 55] = {
    let mut v = [0; 55];
    let mut idx = 0;
    while idx < 55 {
        v[idx] = (1 + idx) * (2 + idx) / 2;
        idx += 1;
    }
    v
};

fn f(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut order = vec![];
    let mut seen = vec![false; 100_001];
    let mut freq = HashMap::new();
    let mut idx = 0;
    'out: for len in LENGTHS {
        if len > n {
            break;
        }
        while idx < len {
            let val = nums[idx];
            if !seen[val as usize] {
                seen[val as usize] = true;
                order.push(val);
            }
            *freq.entry(val).or_insert(0) += 1;
            idx += 1;
        }
        for (i, val) in order.iter().enumerate() {
            if freq.get(val).is_none_or(|&f| f != 1 + i) {
                continue 'out;
            }
        }
        res += 1;
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
        assert_eq!(count_staircase(&[5, 8, 8, 2, 2, 2]), 9);
    }

    #[test]
    fn test() {}
}
