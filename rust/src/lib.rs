mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut st = SegTree::new(&nums, k);
    let mut res = vec![];
    for q in queries {
        let [idx, val, start, x] = q[..] else {
            unreachable!()
        };
        st.update(k, idx, val);
        res.push(st.query(k, start, x));
    }
    res
}

#[derive(Clone)]
struct Node {
    freq: Vec<i32>,
    rem: usize,
}

impl Node {
    fn new(rem: usize, k: usize) -> Self {
        let mut freq = vec![0; k];
        freq[rem] = 1;
        Self { freq, rem }
    }

    fn merge(&self, other: &Self, k: usize) -> Self {
        let mut freq = if !self.freq.is_empty() {
            self.freq.to_vec()
        } else {
            vec![0; k]
        };
        let rem = self.rem * other.rem % k;
        for i in 0..k {
            freq[self.rem * i % k] += other.freq[i];
        }
        Self { freq, rem }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            freq: vec![],
            rem: 1,
        }
    }
}

struct SegTree {
    tree: Vec<Node>,
    n: usize,
}

impl SegTree {
    fn new(nums: &[i32], k: i32) -> Self {
        let n = nums.len();
        let k = k as usize;
        let mut s = Self {
            tree: vec![Node::default(); 4 * n],
            n,
        };
        s.build(nums, k, 1, 0, n - 1);
        s
    }

    fn build(&mut self, nums: &[i32], k: usize, node: usize, left: usize, right: usize) {
        if left == right {
            let rem = nums[left] as usize % k;
            self.tree[node] = Node::new(rem, k);
            return;
        }
        let mid = left.midpoint(right);
        self.build(nums, k, 2 * node, left, mid);
        self.build(nums, k, 1 + 2 * node, 1 + mid, right);
        self.tree[node] = self.tree[2 * node].merge(&self.tree[1 + 2 * node], k);
    }

    fn update(&mut self, k: i32, idx: i32, val: i32) {
        self._update(
            k as usize,
            1,
            0,
            self.n - 1,
            idx as usize,
            (val % k) as usize,
        );
    }

    fn _update(
        &mut self,
        k: usize,
        node: usize,
        left: usize,
        right: usize,
        idx: usize,
        val: usize,
    ) {
        if left == right {
            self.tree[node] = Node::new(val, k);
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(k, 2 * node, left, mid, idx, val);
        } else {
            self._update(k, 1 + 2 * node, 1 + mid, right, idx, val);
        }
        self.tree[node] = self.tree[2 * node].merge(&self.tree[1 + 2 * node], k);
    }

    fn query(&self, k: i32, start: i32, x: i32) -> i32 {
        let node = self._query(k as usize, 1, 0, self.n - 1, start as usize, self.n - 1);
        *node.freq.get(x as usize).unwrap_or(&0)
    }

    fn _query(
        &self,
        k: usize,
        node: usize,
        left: usize,
        right: usize,
        ql: usize,
        qr: usize,
    ) -> Node {
        if qr < left || right < ql {
            return Node::default();
        }
        if ql <= left && right <= qr {
            return self.tree[node].clone();
        }
        let mid = left.midpoint(right);
        let a = self._query(k, 2 * node, left, mid, ql, qr);
        let b = self._query(k, 1 + 2 * node, 1 + mid, right, ql, qr);
        a.merge(&b, k)
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
