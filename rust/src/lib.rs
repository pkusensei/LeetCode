mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
    // 1 for zero/origin, another 1 for M
    let mut tree = SegTree::new(1 + M);
    let mut res = vec![];
    for q in queries {
        if q[0] == 1 {
            tree.update(q[1] as usize);
        } else {
            if let Some(node) = tree.query(0, q[1] as usize) {
                let len = node.len.max(node.start).max(q[1] as usize - node.end);
                res.push(q[2] <= len as i32);
            } else {
                res.push(q[2] <= q[1]);
            }
        }
    }
    res
}

const M: usize = 50_000;

#[derive(Clone, Copy, Default)]
struct Node {
    start: usize,
    end: usize,
    len: usize,
}

impl Node {
    fn merge(a: Option<Self>, b: Option<Self>) -> Option<Self> {
        match [a, b] {
            [_, None] => a,
            [None, _] => b,
            [Some(a), Some(b)] => {
                let len = a.len.max(b.len).max(b.start - a.end);
                Some(Self {
                    start: a.start,
                    end: b.end,
                    len,
                })
            }
        }
    }
}

struct SegTree {
    tree: Vec<Option<Node>>,
    n: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let mut s = Self {
            tree: vec![None; 4 * n],
            n,
        };
        s._update(1, 0, n - 1, 0);
        s
    }

    fn update(&mut self, idx: usize) {
        self._update(1, 0, self.n - 1, idx);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize) {
        if left == right {
            self.tree[node] = Some(Node {
                start: idx,
                end: idx,
                len: 0,
            });
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx);
        } else {
            self._update(2 * node + 1, 1 + mid, right, idx);
        }
        self.tree[node] = Node::merge(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    fn query(&self, ql: usize, qr: usize) -> Option<Node> {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> Option<Node> {
        if qr < left || right < ql {
            return None;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        let a = self._query(2 * node, left, mid, ql, qr);
        let b = self._query(2 * node + 1, 1 + mid, right, ql, qr);
        Node::merge(a, b)
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
