mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = nums.len();
    let mut st = SegTree::new(n);
    for i in 1..n - 1 {
        if nums[i - 1] < nums[i] && nums[i] > nums[1 + i] {
            st.update(i, 1);
        }
    }
    let mut res = vec![];
    for q in queries.iter() {
        if q[0] == 1 {
            res.push(st.query(q[1], q[2]));
        } else {
            let idx = q[1] as usize;
            nums[idx] = q[2];
            for i in 1.max(idx.saturating_sub(1))..=(idx + 1).min(n - 2) {
                let v = nums[i - 1] < nums[i] && nums[i] > nums[1 + i];
                st.update(i, v.into());
            }
        }
    }
    res
}

struct SegTree {
    tree: Vec<Node>,
    n: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![Node::default(); 4 * n],
            n,
        }
    }

    fn update(&mut self, idx: usize, val: i64) {
        self._update(1, 0, self.n - 1, idx, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i64) {
        if left == right {
            self.tree[node] = Node {
                count: val,
                left_peak: left as i64,
                right_peak: left as i64,
                peak_free: 0,
            };
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, val);
        } else {
            self._update(1 + 2 * node, 1 + mid, right, idx, val);
        }
        self.tree[node] = Self::merge(self.tree[2 * node], self.tree[1 + 2 * node]);
    }

    fn query(&self, ql: i32, qr: i32) -> i64 {
        let node = self._query(1, 0, self.n - 1, 1 + ql as usize, qr as usize - 1);
        if node.count == 0 {
            return 0;
        }
        let [ql, qr] = [ql, qr].map(i64::from);
        let len = qr - ql;
        let left = node.left_peak;
        let right = node.right_peak;
        (len.pow(2) - node.peak_free - (left - ql).pow(2) - (qr - right).pow(2)) / 2
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> Node {
        if qr < left || right < ql {
            return Node::default();
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        Self::merge(
            self._query(2 * node, left, mid, ql, qr),
            self._query(1 + 2 * node, 1 + mid, right, ql, qr),
        )
    }

    fn merge(a: Node, b: Node) -> Node {
        if a.count == 0 {
            return b;
        }
        if b.count == 0 {
            return a;
        }
        let gap = b.left_peak - a.right_peak;
        Node {
            count: a.count + b.count,
            left_peak: a.left_peak,
            right_peak: b.right_peak,
            peak_free: a.peak_free + b.peak_free + gap.pow(2),
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Node {
    count: i64,
    left_peak: i64,
    right_peak: i64,
    peak_free: i64,
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
