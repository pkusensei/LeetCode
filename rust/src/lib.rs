mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::{Itertools, izip};
use std::collections::HashMap;

pub fn min_operations(nums: &[i32], k: i32, queries: &[[i32; 2]]) -> Vec<i64> {
    let n = nums.len();
    let mut roots = vec![n; n];
    let mut left = 0;
    for ch in nums.chunk_by(|a, b| a % k == b % k) {
        roots[left..left + ch.len()].fill(left);
        left += ch.len();
    }
    let pst = PersistentSegTree::new(nums);
    let mut res = vec![];
    for q in queries {
        let [ql, qr] = [0, 1].map(|i| q[i] as usize);
        if roots[ql] != roots[qr] {
            res.push(-1);
            continue;
        }
        let med = pst.median(ql, qr);
        let low = pst.range(ql, qr, 0, med - 1);
        let high = pst.range(ql, qr, 1 + med, i32::MAX);
        let diff = i64::from(med) * low[0] - low[1] + high[1] - i64::from(med) * high[0];
        res.push(diff / i64::from(k));
    }
    res
}

struct PersistentSegTree {
    n: usize,
    sorted: Vec<i32>,
    tree: Vec<usize>,
    count: Vec<i64>,
    sum: Vec<i64>,
    left: Vec<usize>,
    right: Vec<usize>,
}

impl PersistentSegTree {
    fn new(nums: &[i32]) -> Self {
        let sorted = nums.iter().copied().sorted_unstable().dedup().collect_vec();
        let n = sorted.len();
        let mut st = Self {
            n,
            sorted,
            tree: vec![0; nums.len()],
            count: vec![0],
            sum: vec![0],
            left: vec![0],
            right: vec![0],
        };
        let mut prev = 0;
        for (i, &num) in nums.iter().enumerate() {
            let id = st.sorted.binary_search(&num).unwrap();
            prev = st.update(prev, id, num, 0, n - 1);
            st.tree[i] = prev;
        }
        st
    }

    fn range(&self, ql: usize, qr: usize, low: i32, high: i32) -> [i64; 2] {
        let start = self.sorted.partition_point(|&v| v < low);
        let end = self.sorted.partition_point(|&v| v <= high).checked_sub(1);
        if end.is_none_or(|v| start > v) {
            return [0, 0];
        }
        let left_tree = if ql == 0 { 0 } else { self.tree[ql - 1] };
        let right_tree = self.tree[qr];
        self._range(left_tree, right_tree, start, end.unwrap(), 0, self.n - 1)
    }

    fn _range(
        &self,
        left_tree: usize,
        right_tree: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
    ) -> [i64; 2] {
        if end < left || right < start {
            return [0, 0];
        }
        if start <= left && right <= end {
            return [
                self.count[right_tree] - self.count[left_tree],
                self.sum[right_tree] - self.sum[left_tree],
            ];
        }
        let mid = left.midpoint(right);
        let a = self._range(
            self.left[left_tree],
            self.left[right_tree],
            start,
            end,
            left,
            mid,
        );
        let b = self._range(
            self.right[left_tree],
            self.right[right_tree],
            start,
            end,
            1 + mid,
            right,
        );
        [a[0] + b[0], a[1] + b[1]]
    }

    fn median(&self, ql: usize, qr: usize) -> i32 {
        let k = (qr - ql + 1 + 1) >> 1;
        let left_tree = if ql == 0 { 0 } else { self.tree[ql - 1] };
        let right_tree = self.tree[qr];
        self.kth(left_tree, right_tree, k as i64, 0, self.n - 1)
    }

    fn kth(&self, left_tree: usize, right_tree: usize, k: i64, left: usize, right: usize) -> i32 {
        if self.count[right_tree] - self.count[left_tree] < k {
            return i32::MIN;
        }
        if left == right {
            return self.sorted[left];
        }
        let mid = left.midpoint(right);
        let left_count = self.count[self.left[right_tree]] - self.count[self.left[left_tree]];
        if left_count >= k {
            self.kth(self.left[left_tree], self.left[right_tree], k, left, mid)
        } else {
            self.kth(
                self.right[left_tree],
                self.right[right_tree],
                k - left_count,
                1 + mid,
                right,
            )
        }
    }

    fn new_node(&mut self) -> usize {
        self.count.push(0);
        self.sum.push(0);
        self.left.push(0);
        self.right.push(0);
        self.count.len() - 1
    }

    fn update(&mut self, prev: usize, idx: usize, val: i32, left: usize, right: usize) -> usize {
        let curr = self.new_node();
        self.count[curr] = self.count[prev];
        self.sum[curr] = self.sum[prev];
        self.left[curr] = self.left[prev];
        self.right[curr] = self.right[prev];
        if left == right {
            self.count[curr] += 1;
            self.sum[curr] += i64::from(val);
            return curr;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self.left[curr] = self.update(self.left[prev], idx, val, left, mid);
        } else {
            self.right[curr] = self.update(self.right[prev], idx, val, 1 + mid, right);
        }
        self.count[curr] = self.count[self.left[curr]] + self.count[self.right[curr]];
        self.sum[curr] = self.sum[self.left[curr]] + self.sum[self.right[curr]];
        curr
    }
}

pub fn with_seg_tree(nums: &[i32], k: i32, queries: &[[i32; 2]]) -> Vec<i64> {
    let n = nums.len();
    let mut roots = vec![n; n];
    let mut left = 0;
    for ch in nums.chunk_by(|a, b| a % k == b % k) {
        roots[left..left + ch.len()].fill(left);
        left += ch.len();
    }
    let st = SegTree::new(nums);
    let mut res = vec![];
    for q in queries {
        let [ql, qr] = [0, 1].map(|i| q[i] as usize);
        if roots[ql] != roots[qr] {
            res.push(-1);
            continue;
        }
        res.push(st.query(k, ql, qr));
    }
    res
}

struct SegTree {
    tree: Vec<Vec<i32>>,
    sorted: Vec<i32>,
    val_idx: HashMap<i32, usize>,
    n: usize,
    f_len: usize,
}

impl SegTree {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let sorted = nums.iter().copied().sorted_unstable().dedup().collect_vec();
        let val_idx = sorted
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect::<HashMap<_, _>>();
        let f_len = sorted.len();
        let mut st = Self {
            tree: vec![vec![]; 4 * n],
            sorted,
            val_idx,
            n,
            f_len,
        };
        st.build(&nums, 1, 0, n - 1);
        st
    }

    fn build(&mut self, nums: &[i32], node: usize, left: usize, right: usize) -> Vec<i32> {
        if left == right {
            let mut f = vec![0; self.f_len];
            f[self.val_idx[&nums[left]]] = 1;
            self.tree[node] = f;
            return self.tree[node].clone();
        }
        let mid = left.midpoint(right);
        let a = self.build(nums, 2 * node, left, mid);
        let b = self.build(nums, 2 * node + 1, 1 + mid, right);
        let f = izip!(a, b).map(|(x, y)| x + y).collect();
        self.tree[node] = f;
        self.tree[node].clone()
    }

    // 1 2 4 5
    // 2 1+2+3
    // 3 2+1+1+2
    fn query(&self, k: i32, ql: usize, qr: usize) -> i64 {
        let freq = self._query(1, 0, self.n - 1, ql, qr);
        let sum = freq.iter().sum::<i32>();
        if sum == 0 {
            return 0;
        }
        let mut curr = 0;
        for (i, &f) in freq.iter().enumerate() {
            curr += f;
            if curr > sum / 2 {
                let med = self.sorted[i];
                return izip!(freq, &self.sorted)
                    .map(|(f, val)| i64::from(f) * i64::from(med / k - val / k).abs())
                    .sum();
            }
        }
        -1
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> Vec<i32> {
        if qr < left || right < ql {
            return vec![0; self.f_len];
        }
        if ql <= left && right <= qr {
            return self.tree[node].clone();
        }
        let mid = left.midpoint(right);
        let a = self._query(2 * node, left, mid, ql, qr);
        let b = self._query(2 * node + 1, 1 + mid, right, ql, qr);
        izip!(a, b).map(|(x, y)| x + y).collect()
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
        assert_eq!(min_operations(&[1, 4, 7], 3, &[[0, 1], [0, 2]]), [1, 2]);
        assert_eq!(
            min_operations(&[1, 2, 4], 2, &[[0, 2], [0, 0], [1, 2]]),
            [-1, 0, 1]
        );

        assert_eq!(with_seg_tree(&[1, 4, 7], 3, &[[0, 1], [0, 2]]), [1, 2]);
        assert_eq!(
            with_seg_tree(&[1, 2, 4], 2, &[[0, 2], [0, 0], [1, 2]]),
            [-1, 0, 1]
        );
    }

    #[test]
    fn test() {}
}
