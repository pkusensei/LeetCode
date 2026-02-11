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

pub fn longest_balanced(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut pos = HashMap::new();
    let mut st = SegTree::new(n);
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        let val = if num & 1 == 1 { 1 } else { -1 };
        if let Some(left) = pos.insert(num, right) {
            st.update(0, left, -val);
        }
        st.update(0, right, val);
        if let Some(left) = st.query()
            && left < right
        {
            res = res.max(right - left + 1);
        }
    }
    res as i32
}

struct SegTree {
    mint: Vec<i32>,
    maxt: Vec<i32>,
    lazy: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            mint: vec![0; 4 * n],
            maxt: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
            n,
        }
    }

    fn update(&mut self, ql: usize, qr: usize, val: i32) {
        self._update(1, 0, self.n - 1, ql, qr, val);
    }

    fn query(&mut self) -> Option<usize> {
        self._query(1, 0, self.n - 1)
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, ql: usize, qr: usize, val: i32) {
        self.push(node, left, right);
        if qr < left || right < ql {
            return;
        }
        if ql <= left && right <= qr {
            self.lazy[node] += val;
            self.push(node, left, right);
            return;
        }
        let mid = left.midpoint(right);
        self._update(2 * node, left, mid, ql, qr, val);
        self._update(2 * node + 1, 1 + mid, right, ql, qr, val);
        self.mint[node] = self.mint[2 * node].min(self.mint[2 * node + 1]);
        self.maxt[node] = self.maxt[2 * node].max(self.maxt[2 * node + 1]);
    }

    fn _query(&mut self, node: usize, left: usize, right: usize) -> Option<usize> {
        self.push(node, left, right);
        if self.mint[node] > 0 || self.maxt[node] < 0 {
            return None;
        }
        if left == right {
            return if self.mint[node] == 0 {
                Some(left)
            } else {
                None
            };
        }
        let mid = left.midpoint(right);
        self._query(2 * node, left, mid)
            .or_else(|| self._query(2 * node + 1, 1 + mid, right))
    }

    fn push(&mut self, node: usize, left: usize, right: usize) {
        if self.lazy[node] != 0 {
            self.mint[node] += self.lazy[node];
            self.maxt[node] += self.lazy[node];
            if left < right {
                self.lazy[2 * node] += self.lazy[node];
                self.lazy[2 * node + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
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
        assert_eq!(longest_balanced(&[1, 2, 3, 2]), 3);
    }

    #[test]
    fn test() {}
}
