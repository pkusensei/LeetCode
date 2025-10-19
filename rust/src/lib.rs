mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_balanced(nums: &[i32]) -> i32 {
    use std::collections::HashMap;
    let n = nums.len();
    let mut prev = HashMap::with_capacity(n);
    let mut st = SegTree::new(n);
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        let sign = if num & 1 == 1 { 1 } else { -1 };
        if let Some(&left) = prev.get(&num) {
            // remove old impact to ensure distinct
            st.update(0, left, -sign);
        }
        st.update(0, right, sign);
        prev.insert(num, right);
        // With this number counted in, there is a balanced left point
        if let Some(left) = st.query()
            && left <= right
        {
            res = res.max(right + 1 - left);
        }
    }
    res as i32
}

struct SegTree {
    min_tree: Vec<i32>,
    max_tree: Vec<i32>,
    lazy: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            min_tree: vec![0; 4 * n],
            max_tree: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
            n,
        }
    }

    /// find leftmost zero
    fn query(&mut self) -> Option<usize> {
        self._query(1, 0, self.n - 1)
    }

    fn _query(&mut self, node: usize, left: usize, right: usize) -> Option<usize> {
        self.push(node, left, right); // push any pending updates
        if self.min_tree[node] > 0 || self.max_tree[node] < 0 {
            return None;
        }
        if left == right {
            return if self.min_tree[node] == 0 {
                Some(left)
            } else {
                None
            };
        }
        let mid = left.midpoint(right);
        self._query(2 * node, left, mid)
            .or_else(|| self._query(2 * node + 1, 1 + mid, right))
    }

    fn update(&mut self, ql: usize, qr: usize, val: i32) {
        self._update(1, 0, self.n - 1, ql, qr, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, ql: usize, qr: usize, val: i32) {
        self.push(node, left, right); // push any pending updates
        if left > right || qr < left || right < ql {
            return;
        }
        if ql <= left && right <= qr {
            self.lazy[node] += val; // cache local update on this range
            self.push(node, left, right);
            return; // recursion stops here!
        }
        let mid = left.midpoint(right);
        self._update(2 * node, left, mid, ql, qr, val);
        self._update(2 * node + 1, 1 + mid, right, ql, qr, val);
        self.min_tree[node] = self.min_tree[2 * node].min(self.min_tree[2 * node + 1]);
        self.max_tree[node] = self.max_tree[2 * node].max(self.max_tree[2 * node + 1]);
    }

    fn push(&mut self, node: usize, left: usize, right: usize) {
        if self.lazy[node] != 0 {
            self.min_tree[node] += self.lazy[node];
            self.max_tree[node] += self.lazy[node];
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
        assert_eq!(longest_balanced(&[2, 5, 4, 3]), 4);
        assert_eq!(longest_balanced(&[3, 2, 2, 5, 4]), 5);
        assert_eq!(longest_balanced(&[1, 2, 3, 2]), 3);
    }

    #[test]
    fn test() {}
}
