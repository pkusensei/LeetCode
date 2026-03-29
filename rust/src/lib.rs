mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::i32;

#[allow(unused_imports)]
use helper::*;
use itertools::{Itertools, izip};

pub fn sortable_integers(nums: &[i32]) -> i32 {
    let n = nums.len();
    let facts = get_factors(n);
    if nums.is_sorted() {
        return facts.iter().sum::<usize>() as i32;
    }
    let mut min = 0;
    let mut res = 0;
    // remove 1
    'out: for &k in &facts[1..] {
        for ch in nums.chunks(k) {
            if let Some(v) = check(ch, min) {
                min = v;
            } else {
                min = 0;
                continue 'out; // skip +k
            }
        }
        res += k;
        min = 0;
    }
    res as i32
}

fn check(nums: &[i32], min: i32) -> Option<i32> {
    let n = nums.len();
    let mut max = i32::MIN;
    let mut pivot = false;
    for (i, &num) in nums.iter().enumerate() {
        if num < min {
            return None;
        }
        max = max.max(num);
        if i > 0 && nums[i - 1] > num {
            if pivot {
                return None;
            };
            pivot = true
        }
    }
    if pivot && nums[0] < nums[n - 1] {
        None
    } else {
        Some(max)
    }
}

fn get_factors(num: usize) -> Vec<usize> {
    let mut res = vec![];
    for p in 1..=num.isqrt() {
        if num % p == 0 {
            res.push(p);
            if p * p < num {
                res.push(num / p);
            }
        }
    }
    res.sort_unstable();
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
    fn test() {
        assert_eq!(sortable_integers(&[2, 2, 1, 1]), 4);
    }
}
