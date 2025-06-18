mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn tree_queries(n: i32, edges: &[[i32; 3]], queries: &[&[i32]]) -> Vec<i32> {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        adj[a].push((b, e[2]));
        adj[b].push((a, e[2]));
    }
    let mut path = Vec::with_capacity(n - 1);
    let mut seg_begs = vec![0; n];
    let mut seg_ends = vec![0; n];
    let mut edge_child = HashMap::new();
    euler_dfs(
        &adj,
        0,
        n,
        0,
        &mut path,
        &mut seg_begs,
        &mut seg_ends,
        &mut edge_child,
    );
    let mut st = SegTree::new(&path);
    let mut res = vec![];
    for q in queries.iter() {
        match q {
            [1, _a, _b, w] => {
                let [a, b] = [_a, _b].map(|&v| v as usize - 1);
                let node = edge_child[&[a.min(b), a.max(b)]];
                let i_beg = seg_begs[node];
                let i_end = seg_ends[node];
                st.update(i_beg, *w);
                st.update(i_end, -w);
            }
            [2, v] => {
                let idx = seg_begs[*v as usize - 1];
                res.push(st.query(idx));
            }
            _ => (),
        }
    }
    res
}

fn euler_dfs(
    adj: &[Vec<(usize, i32)>],
    node: usize,
    prev: usize,
    weight: i32,
    path: &mut Vec<i32>,
    seg_begs: &mut [usize],
    seg_ends: &mut [usize],
    edge_child: &mut HashMap<[usize; 2], usize>,
) {
    seg_begs[node] = path.len();
    path.push(weight);
    for &(next, w) in &adj[node] {
        if next != prev {
            let a = node.min(next);
            let b = node.max(next);
            edge_child.insert([a, b], next);
            euler_dfs(adj, next, node, w, path, seg_begs, seg_ends, edge_child);
        }
    }
    seg_ends[node] = path.len();
    path.push(-weight);
}

struct SegTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            n,
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
        self.merge(node);
    }

    fn merge(&mut self, node: usize) {
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    fn update(&mut self, idx: usize, val: i32) {
        self._update(1, 0, self.n - 1, idx, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i32) {
        if left == right {
            self.tree[node] = val;
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, val);
        } else {
            self._update(2 * node + 1, 1 + mid, right, idx, val);
        }
        self.merge(node);
    }

    fn query(&self, idx: usize) -> i32 {
        self._query(1, 0, self.n - 1, 0, idx)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if ql == left && right == qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        if qr <= mid {
            self._query(2 * node, left, mid, ql, qr)
        } else if 1 + mid <= ql {
            self._query(2 * node + 1, 1 + mid, right, ql, qr)
        } else {
            self._query(2 * node, left, mid, ql, mid)
                + self._query(2 * node + 1, 1 + mid, right, 1 + mid, qr)
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
        assert_eq!(
            tree_queries(2, &[[1, 2, 7]], &[&[2, 2], &[1, 1, 2, 4], &[2, 2]]),
            [7, 4]
        );
        assert_eq!(
            tree_queries(
                3,
                &[[1, 2, 2], [1, 3, 4]],
                &[&[2, 1], &[2, 3], &[1, 1, 3, 7], &[2, 2], &[2, 3]]
            ),
            [0, 4, 2, 7]
        );
        assert_eq!(
            tree_queries(
                4,
                &[[1, 2, 2], [2, 3, 1], [3, 4, 5]],
                &[&[2, 4], &[2, 3], &[1, 2, 3, 3], &[2, 2], &[2, 3]]
            ),
            [8, 3, 2, 5]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            tree_queries(
                4,
                &[[1, 2, 2], [4, 2, 3], [1, 3, 4]],
                &[&[2, 2], &[1, 3, 1, 1], &[2, 3]]
            ),
            [2, 1]
        );
    }
}
