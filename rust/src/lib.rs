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
    let mut ft = Fenwick::new(n);
    let mut res = 0;
    let mut curr = 0;
    for (idx, &num) in nums.iter().enumerate() {
        let left = seen.insert(num, idx).map(|v| 1 + v).unwrap_or(0);
        let v = ft.query(left, idx);
        curr = (curr + 2 * v + (1 + idx - left) as i64) % M;
        res = (res + curr) % M;
        ft.add(left, idx);
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

struct Fenwick {
    n: usize,
    tree1: Vec<i64>,
    tree2: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            tree1: vec![0; 2 + n],
            tree2: vec![0; 2 + n],
        }
    }

    fn add(&mut self, left: usize, right: usize) {
        self.update(left, 1);
        self.update(1 + right, -1);
    }

    fn update(&mut self, mut idx: usize, val: i64) {
        idx += 1;
        let raw = idx as i64;
        while idx <= self.n {
            self.tree1[idx] += val;
            self.tree2[idx] += raw * val;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, left: usize, right: usize) -> i64 {
        // left.saturating_sub(1) is not fit
        // Here we do not count anything if `left == 0`
        // But left.saturating_sub(1) would take that 0 into `_query()`
        // and increment `idx` to 1 and yield a number.
        self._query(right) - if left == 0 { 0 } else { self._query(left - 1) }
    }

    fn _query(&self, mut idx: usize) -> i64 {
        idx += 1;
        let raw = 1 + idx as i64;
        idx = idx.min(self.n);
        let mut res = 0;
        while idx > 0 {
            res += raw * self.tree1[idx] - self.tree2[idx];
            idx -= idx & idx.wrapping_neg();
        }
        res
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
