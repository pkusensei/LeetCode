mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn palindrome_path(n: i32, edges: &[[i32; 2]], s: &str, queries: &[&str]) -> Vec<bool> {
    use itertools::Itertools;
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut hld = HLD::new(adj);
    hld.build(0);
    let masks = s
        .bytes()
        .enumerate()
        .map(|(i, b)| (hld.pos[i], 1 << (b - b'a')))
        .sorted_unstable()
        .map(|(_, v)| v)
        .collect_vec();
    let mut st = SegTree::new(&masks);
    let mut res = vec![];
    for q in queries.iter() {
        if let Some(qq) = q.strip_prefix("query ") {
            let (u, v) = qq.split_once(' ').unwrap();
            let [a, b] = [u, v].map(|v| v.parse::<usize>().unwrap());
            let mask = hld.query(a, b, &st);
            res.push(mask.count_ones() <= 1);
        } else if let Some(qu) = q.strip_prefix("update ") {
            let (u, c) = qu.split_once(' ').unwrap();
            let u = u.parse::<usize>().unwrap();
            let nmask = 1 << (c.as_bytes()[0] - b'a');
            let idx = hld.pos[u];
            st.update(idx, nmask);
        }
    }
    res
}

// Heavy Light Decomposition
struct HLD {
    adj: Vec<Vec<usize>>,
    parent: Vec<usize>,
    depth: Vec<usize>,
    heavy: Vec<usize>,
    head: Vec<usize>,
    pos: Vec<usize>,
    size: Vec<usize>,
    cur_pos: usize,
    n: usize,
}

impl HLD {
    fn new(adj: Vec<Vec<usize>>) -> Self {
        let n = adj.len();
        Self {
            n,
            adj,
            parent: vec![0; n],
            depth: vec![0; n],
            heavy: vec![n; n],
            head: vec![0; n],
            pos: vec![0; n],
            size: vec![0; n],
            cur_pos: 0,
        }
    }

    // First DFS: compute subtree sizes + heavy child
    fn dfs(&mut self, node: usize, p: usize) {
        self.size[node] = 1;
        self.parent[node] = p;
        self.heavy[node] = self.n;

        let mut max_subtree = 0;
        for next in self.adj[node].clone() {
            if next == p {
                continue;
            }
            self.depth[next] = self.depth[node] + 1;
            self.dfs(next, node);
            self.size[node] += self.size[next];
            if self.size[next] > max_subtree {
                max_subtree = self.size[next];
                self.heavy[node] = next;
            }
        }
    }

    // Second DFS: decompose into chains
    fn decompose(&mut self, node: usize, curr_head: usize) {
        self.head[node] = curr_head;
        self.pos[node] = self.cur_pos;
        self.cur_pos += 1;

        // Continue heavy chain
        if self.heavy[node] < self.n {
            self.decompose(self.heavy[node], curr_head);
        }
        for next in self.adj[node].clone() {
            if next != self.parent[node] && next != self.heavy[node] {
                // New chain
                self.decompose(next, next);
            }
        }
    }

    fn build(&mut self, root: usize) {
        self.depth[root] = 0;
        self.dfs(root, self.n);
        self.decompose(root, root);
    }

    fn query(&self, mut a: usize, mut b: usize, st: &SegTree) -> i32 {
        use std::mem::swap;
        let mut res = 0;
        while self.head[a] != self.head[b] {
            if self.depth[self.head[a]] < self.depth[self.head[b]] {
                swap(&mut a, &mut b);
            }
            let head_a = self.head[a];
            res ^= st.query(self.pos[head_a], self.pos[a]);
            a = self.parent[head_a];
        }
        if self.depth[a] > self.depth[b] {
            swap(&mut a, &mut b);
        }
        res ^= st.query(self.pos[a], self.pos[b]);
        res
    }
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
        self.tree[node] = self.tree[2 * node] ^ self.tree[2 * node + 1];
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
        self.tree[node] = self.tree[2 * node] ^ self.tree[2 * node + 1]
    }

    fn query(&self, ql: usize, qr: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if qr < left || right < ql {
            return 0;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        self._query(2 * node, left, mid, ql, qr) ^ self._query(2 * node + 1, 1 + mid, right, ql, qr)
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
            palindrome_path(
                3,
                &[[0, 1], [1, 2]],
                "aac",
                &["query 0 2", "update 1 b", "query 0 2"]
            ),
            [true, false]
        );
        assert_eq!(
            palindrome_path(
                4,
                &[[0, 1], [0, 2], [0, 3]],
                "abca",
                &[
                    "query 1 2",
                    "update 0 b",
                    "query 2 3",
                    "update 3 a",
                    "query 1 3"
                ],
            ),
            [false, false, true]
        )
    }

    #[test]
    fn test() {
        assert_eq!(
            palindrome_path(3, &[[0, 2], [0, 1]], "onm", &["query 0 1"]),
            [false]
        );
        assert_eq!(
            palindrome_path(
                4,
                &[[2, 3], [0, 1], [0, 2]],
                "tqtt",
                &["query 3 1", "update 1 d"]
            ),
            [false]
        );
    }
}
