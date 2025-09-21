mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_value(nums: &[i32], k: i32) -> i64 {
    use std::collections::BinaryHeap;
    let n = nums.len();
    let mintree = SegTree::new(nums, std::cmp::min, i32::MAX);
    let maxtree = SegTree::new(nums, std::cmp::max, i32::MIN);
    // (diff, left, right)
    let mut heap = BinaryHeap::with_capacity(n);
    for left in 0..n {
        let diff = maxtree.query(left, n - 1) - mintree.query(left, n - 1);
        heap.push((diff, left, n - 1));
    }
    let mut res = 0;
    for _ in 0..k {
        let Some((diff, left, right)) = heap.pop() else {
            break;
        };
        res += i64::from(diff);
        if left < right {
            let diff = maxtree.query(left, right - 1) - mintree.query(left, right - 1);
            heap.push((diff, left, right - 1));
        }
    }
    res
}

struct SegTree {
    tree: Vec<i32>,
    n: usize,
    f: fn(i32, i32) -> i32,
    discard: i32,
}

impl SegTree {
    fn new(nums: &[i32], f: fn(i32, i32) -> i32, discard: i32) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            n,
            f,
            discard,
        };
        s.build(1, 0, n - 1, nums);
        s
    }

    fn build(&mut self, node: usize, left: usize, right: usize, nums: &[i32]) {
        if left == right {
            self.tree[node] = nums[left];
            return;
        }
        let mid = left.midpoint(right);
        self.build(2 * node, left, mid, nums);
        self.build(2 * node + 1, 1 + mid, right, nums);
        self.tree[node] = (self.f)(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    fn query(&self, ql: usize, qr: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if qr < left || right < ql {
            return self.discard;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        (self.f)(
            self._query(2 * node, left, mid, ql, qr),
            self._query(2 * node + 1, 1 + mid, right, ql, qr),
        )
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
    fn basics() {
        assert_eq!(max_total_value(&[1, 3, 2], 2), 4);
        assert_eq!(max_total_value(&[4, 2, 5, 1], 3), 12);
    }

    #[test]
    fn test() {
        assert_eq!(max_total_value(&[11, 8], 2), 3);
    }
}
