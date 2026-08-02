mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::{cmp::Reverse, collections::HashMap};

// even/odd <= a/b
// even*b <= odd*a
pub fn count_ratio_subarrays(nums: Vec<i32>, a: i32, b: i32) -> i64 {
    let prefix = nums.iter().fold(vec![0], |mut acc, v| {
        let curr = if v & 1 == 0 { b } else { -a };
        acc.push(i64::from(curr) + acc.last().unwrap_or(&0));
        acc
    });
    let map = prefix.iter().sorted_unstable_by_key(|&&v| Reverse(v)).fold(
        HashMap::new(),
        |mut acc, &v| {
            let i = acc.len();
            acc.entry(v).or_insert(i);
            acc
        },
    );
    let n = map.len();
    let mut ft = FenwickTree::new(1 + n);
    let mut res = 0;
    for v in prefix.iter() {
        let i = map[v];
        res += ft.query(1 + i);
        ft.update(1 + i, 1);
    }
    res
}

struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
            n,
        }
    }

    fn update(&mut self, mut idx: usize, val: i64) {
        while idx <= self.n {
            self.tree[idx] += val;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut res = 0;
        while idx > 0 {
            res += self.tree[idx];
            idx -= idx & idx.wrapping_neg();
        }
        res
    }
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
