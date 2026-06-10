mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let max_tree = SegTree::new(&nums, |a, b| a.max(b), i32::MIN >> 2);
    let min_tree = SegTree::new(&nums, |a, b| a.min(b), i32::MAX >> 2);
    let mut res = 0;
    let mut heap: BinaryHeap<_> = (0..n)
        .map(|left| {
            let val = max_tree.query(left, n - 1) - min_tree.query(left, n - 1);
            (val, left, n - 1)
        })
        .collect();
    for _ in 0..k {
        let Some((val, left, right)) = heap.pop() else {
            break;
        };
        res += i64::from(val);
        if left < right {
            let val = max_tree.query(left, right - 1) - min_tree.query(left, right - 1);
            heap.push((val, left, right - 1));
        }
    }
    res
}

struct SegTree {
    tree: Vec<i32>,
    n: usize,
    f: fn(i32, i32) -> i32,
    identity: i32,
}

impl SegTree {
    fn new(nums: &[i32], f: fn(i32, i32) -> i32, identity: i32) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            n,
            f,
            identity,
        };
        s.build(nums, 1, 0, n - 1);
        s
    }

    fn build(&mut self, nums: &[i32], node: usize, left: usize, right: usize) -> i32 {
        if left == right {
            self.tree[node] = nums[left].into();
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        let a = self.build(nums, 2 * node, left, mid);
        let b = self.build(nums, 2 * node + 1, 1 + mid, right);
        self.tree[node] = (self.f)(a, b);
        self.tree[node]
    }

    fn query(&self, ql: usize, qr: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if right < ql || qr < left {
            return self.identity;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        let a = self._query(2 * node, left, mid, ql, qr);
        let b = self._query(2 * node + 1, 1 + mid, right, ql, qr);
        (self.f)(a, b)
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
