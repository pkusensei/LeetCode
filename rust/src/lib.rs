mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn popcount_depth(nums: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i32> {
    let mut tree = SegTree::new(&nums);
    let mut res = vec![];
    for q in queries {
        if q[0] == 1 {
            res.push(tree.query(q[1] as usize, q[2] as usize, q[3] as usize));
        } else {
            tree.update(q[1] as usize, q[2]);
        }
    }
    res
}

struct SegTree {
    tree: Vec<[i32; 6]>,
    n: usize,
}

impl SegTree {
    fn new(nums: &[i64]) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![[0; 6]; 4 * n],
            n,
        };
        s.build(1, 0, n - 1, nums);
        s
    }

    fn build(&mut self, node: usize, left: usize, right: usize, nums: &[i64]) {
        if left == right {
            let val = nums[left]; // bits(val)==1 but depth(val)==0
            let k = DEPTH[val.count_ones() as usize] as usize - usize::from(val == 1);
            self.tree[node][k] = 1;
            return;
        }
        let mid = left.midpoint(right);
        self.build(2 * node, left, mid, nums);
        self.build(2 * node + 1, 1 + mid, right, nums);
        self.merge(node);
    }

    fn update(&mut self, idx: usize, val: i64) {
        self._update(1, 0, self.n - 1, idx, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i64) {
        if left == right {
            let k = DEPTH[val.count_ones() as usize] as usize - usize::from(val == 1);
            self.tree[node].fill(0);
            self.tree[node][k] = 1;
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

    fn query(&self, ql: usize, qr: usize, k: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr, k)
    }

    fn _query(
        &self,
        node: usize,
        left: usize,
        right: usize,
        ql: usize,
        qr: usize,
        k: usize,
    ) -> i32 {
        if qr < left || right < ql {
            return 0;
        }
        if ql <= left && right <= qr {
            return self.tree[node][k];
        }
        let mid = left.midpoint(right);
        self._query(2 * node, left, mid, ql, qr, k)
            + self._query(2 * node + 1, 1 + mid, right, ql, qr, k)
    }

    fn merge(&mut self, node: usize) {
        let left = self.tree[2 * node];
        let right = self.tree[2 * node + 1];
        let mut curr = [0; 6];
        for (v, (a, b)) in curr.iter_mut().zip(left.iter().zip(right)) {
            *v = a + b;
        }
        self.tree[node] = curr;
    }
}

const DEPTH: [i8; 64] = {
    let mut depth = [-1; 64];
    let mut x = 0;
    while x < 64 {
        precompute(x, &mut depth);
        x += 1;
    }
    depth
};

const fn precompute(x: usize, depth: &mut [i8; 64]) -> i8 {
    if x < 2 {
        depth[x] = (x & 1) as _;
    }
    if depth[x] > -1 {
        return depth[x];
    }
    depth[x] = 1 + precompute(x.count_ones() as usize, depth);
    depth[x]
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
