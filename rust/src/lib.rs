mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_counts(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = nums.len();
    let mut seen = HashMap::new();
    let mut st = SegTree::new(n);
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let ql = seen.insert(num, idx).map(|v| 1 + v).unwrap_or(0);
        st.insert(ql, idx);
        res = (res + st.sq[1]) % M;
    }
    res as i32
}

const M: i64 = 1_000_000_007;
struct SegTree {
    n: usize,
    sum: Vec<i64>,
    lazy: Vec<i64>,
    sq: Vec<i64>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            sum: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
            sq: vec![0; 4 * n],
        }
    }

    fn insert(&mut self, ql: usize, qr: usize) {
        self._insert(1, 0, self.n - 1, ql, qr);
    }

    fn _insert(&mut self, node: usize, left: usize, right: usize, ql: usize, qr: usize) {
        self.update(node, left, right);
        if qr < left || right < ql {
            return;
        }
        if ql <= left && right <= qr {
            self.lazy[node] = 1;
            self.update(node, left, right);
            return;
        }
        let mid = left.midpoint(right);
        self._insert(2 * node, left, mid, ql, qr);
        self._insert(1 + 2 * node, 1 + mid, right, ql, qr);
        self.sum[node] = (self.sum[2 * node] + self.sum[1 + 2 * node]) % M;
        self.sq[node] = (self.sq[2 * node] + self.sq[1 + 2 * node]) % M;
    }

    fn update(&mut self, node: usize, left: usize, right: usize) {
        if left != right {
            self.lazy[2 * node] += self.lazy[node];
            self.lazy[1 + 2 * node] += self.lazy[node];
        }
        let len = (1 + right - left) as i64;
        let sum = (self.sum[node] + len * self.lazy[node] % M) % M;
        let sq = (self.sq[node]
            + 2 * self.lazy[node] * self.sum[node] % M
            + self.lazy[node].pow(2) * len % M)
            % M;
        self.sum[node] = sum;
        self.sq[node] = sq;
        self.lazy[node] = 0;
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
