mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::cmp::Reverse;

#[allow(unused_imports)]
use helper::*;

use itertools::{Itertools, chain, izip};

pub fn max_value(nums1: &[i32], nums0: &[i32]) -> i32 {
    let mut ones = 0;
    let mut sorted = vec![];
    for (&f1, &f0) in izip!(nums1.iter(), nums0.iter()) {
        if f0 == 0 {
            ones += f1
        } else {
            sorted.push([f1, f0]);
        }
    }
    sorted.sort_unstable_by_key(|&[f1, f0]| (Reverse(f1), f0));
    let mut res = (mod_pow(2, ones.into()) - 1).rem_euclid(M);
    for [f1, f0] in sorted {
        let p1 = mod_pow(2, f1.into());
        let p2 = mod_pow(2, f0.into());
        res = res * p1 % M;
        res = (res + p1 - 1).rem_euclid(M); // geometric seq
        res = res * p2 % M;
    }
    res as i32
}

fn concat(a: &[u8], b: &[u8]) -> Vec<u8> {
    chain!(a.iter().copied(), b.iter().copied()).collect()
}

const M: i64 = 1_000_000_007;

const fn mod_pow(b: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % M, exp >> 1)
    } else {
        mod_pow(b * b % M, exp >> 1) * b % M
    }
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
