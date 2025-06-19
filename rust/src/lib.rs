mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn result_array(nums: &[i32], k: i32, queries: &[[i32; 4]]) -> Vec<i32> {
    let mut st = SegTree::new(nums, k);
    let mut res = vec![];
    for q in queries.iter() {
        let [idx, val, start, x] = q[..] else {
            unreachable!();
        };
        st.update(idx as usize, val);
        res.push(st.query(start as usize, x));
    }
    res
}

#[derive(Clone, Copy)]
struct Node {
    freq: [i32; 5],
    prod: i32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            freq: [0; 5],
            prod: 1,
        }
    }
}

impl Node {
    fn new(rem: i32) -> Self {
        let mut freq = [0; 5];
        freq[rem as usize] = 1;
        Self { freq, prod: rem }
    }
}

struct SegTree {
    tree: Vec<Node>,
    n: usize,
    k: i32,
}

impl SegTree {
    fn new(nums: &[i32], k: i32) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![Node::default(); 4 * n],
            n,
            k,
        };
        s.build(nums, 1, 0, n - 1);
        s
    }

    fn build(&mut self, nums: &[i32], node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = Node::new(nums[left] % self.k);
            return;
        }
        let mid = left.midpoint(right);
        self.build(nums, 2 * node, left, mid);
        self.build(nums, 2 * node + 1, 1 + mid, right);
        self.tree[node] = self.merge(&self.tree[2 * node], &self.tree[2 * node + 1]);
    }

    fn merge(&self, n1: &Node, n2: &Node) -> Node {
        let prod = n1.prod * n2.prod % self.k;
        let mut freq = n1.freq;
        for rem in 0..self.k {
            freq[(rem * n1.prod % self.k) as usize] += n2.freq[rem as usize];
        }
        Node { freq, prod }
    }

    fn update(&mut self, idx: usize, val: i32) {
        self._update(1, 0, self.n - 1, idx, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i32) {
        if left == right {
            self.tree[node] = Node::new(val % self.k);
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, val);
        } else {
            self._update(2 * node + 1, 1 + mid, right, idx, val);
        }
        self.tree[node] = self.merge(&self.tree[2 * node], &self.tree[2 * node + 1]);
    }

    fn query(&self, start: usize, x: i32) -> i32 {
        let node = self._query(1, 0, self.n - 1, start, self.n - 1);
        node.freq[x as usize]
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> Node {
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        if qr < left || right < ql {
            return Node::default();
        }
        let mid = left.midpoint(right);
        let n1 = self._query(2 * node, left, mid, ql, qr);
        let n2 = self._query(2 * node + 1, 1 + mid, right, ql, qr);
        self.merge(&n1, &n2)
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
            result_array(
                &[1, 2, 3, 4, 5],
                3,
                &[[2, 2, 0, 2], [3, 3, 3, 0], [0, 1, 0, 1]]
            ),
            [2, 2, 2]
        );
        assert_eq!(
            result_array(&[1, 2, 4, 8, 16, 32], 4, &[[0, 2, 0, 2], [0, 2, 0, 1]]),
            [1, 0]
        );
        assert_eq!(result_array(&[1, 1, 2, 1, 1], 2, &[[2, 1, 0, 1]]), [5]);
    }

    #[test]
    fn test() {}
}
