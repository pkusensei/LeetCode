mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{iter, sync::LazyLock};

#[allow(unused_imports)]
use helper::*;
use itertools::{Itertools, chain};

pub fn min_operations(nums: Vec<i32>) -> i64 {
    let mut res = 0;
    for &num in nums.iter() {
        if num < 10 {
            continue;
        }
        let num = i64::from(num);
        let vals = if num & 1 == 1 { &NUMS.0 } else { &NUMS.1 };
        let i = vals.partition_point(|&v| v < num);
        let mut curr = (vals[i] - num).abs() / 2;
        if i > 0 {
            curr = curr.min((vals[i - 1] - num).abs() / 2)
        }
        res += curr;
    }
    res
}

static NUMS: LazyLock<(Vec<i64>, Vec<i64>)> = LazyLock::new(|| {
    let mut odds = vec![];
    for b in (b'1'..=b'9').step_by(2) {
        odds.push(i64::from(b - b'0'));
        odds.extend(precompute(&mut vec![b]));
    }
    let mut evens = vec![];
    for b in (b'2'..=b'8').step_by(2) {
        evens.push(i64::from(b - b'0'));
        evens.extend(precompute(&mut vec![b]));
    }
    odds.sort_unstable();
    evens.sort_unstable();
    (odds, evens)
});

fn precompute(left: &mut Vec<u8>) -> Vec<i64> {
    if left.len() > 6 {
        return vec![];
    }
    let mut res = vec![];
    let right: Vec<_> = left.iter().copied().rev().collect();
    let s: Vec<_> = left.iter().chain(&right).copied().collect();
    res.push(String::from_utf8(s).unwrap().parse().unwrap());
    if left.len() < 5 {
        for b in b'0'..=b'9' {
            let s =
                chain!(left.iter().copied(), iter::once(b), right.iter().copied()).collect_vec();
            res.push(String::from_utf8(s).unwrap().parse().unwrap());
        }
    }
    for b in b'0'..=b'9' {
        left.push(b);
        res.extend(precompute(left));
        left.pop();
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
    fn basics() {}

    #[test]
    fn test() {}
}
