mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::Itertools;
use std::{collections::HashSet, iter, sync::LazyLock};

pub fn special_palindrome(n: i64) -> i64 {
    let i = NUMS.partition_point(|&v| v <= n);
    NUMS[i]
}

static NUMS: LazyLock<Vec<i64>> = LazyLock::new(preprocess);

fn preprocess() -> Vec<i64> {
    let mut res = vec![];
    for len in 1..=16 {
        let ds = find_digits(len, 0, 1);
        let curr = ds.into_iter().flat_map(build).sorted_unstable();
        res.extend(curr);
    }
    res
}

fn find_digits(len: i32, mask: i32, lower: i32) -> Vec<Vec<i32>> {
    if len == 0 {
        return vec![vec![]];
    }
    let mut res = vec![];
    for d in lower..=9.min(len) {
        if (mask >> d) & 1 == 1 || (len & 1 == 0 && d & 1 == 1) {
            continue;
        }
        let tail = find_digits(len - d, mask | (1 << d), d);
        res.extend(tail.into_iter().map(|mut v| {
            v.push(d);
            v
        }));
    }
    res
}

fn build(ds: Vec<i32>) -> Vec<i64> {
    let mut perm = vec![];
    let mut single = None::<u8>;
    for d in ds {
        if d & 1 == 1 {
            single = Some(d as u8 + b'0');
        }
        perm.extend(iter::repeat_n(d as u8 + b'0', d as usize / 2));
    }
    let len = perm.len();
    let mut res = HashSet::<i64>::new();
    for mut p in perm.into_iter().permutations(len) {
        let mut mid = len;
        if let Some(v) = single {
            p.push(v);
            mid += 1;
        }
        p.extend_from_within(..len);
        p[mid..].reverse();
        res.insert(
            String::from_utf8(p)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap(),
        );
    }
    res.into_iter().collect()
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
    fn basics() {
        assert_eq!(special_palindrome(2), 22);
        assert_eq!(special_palindrome(33), 212);
    }

    #[test]
    fn test() {
        assert_eq!(special_palindrome(4774), 23332);
    }
}
