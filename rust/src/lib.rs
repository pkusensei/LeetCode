mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let st = RMQ::new(&nums);
    let mut res = 0;
    let mut heap: BinaryHeap<_> = (0..n)
        .map(|left| (st.query(left, n - 1), left, n - 1))
        .collect();
    for _ in 0..k {
        let Some((val, left, right)) = heap.pop() else {
            break;
        };
        res += i64::from(val);
        if left < right {
            heap.push((st.query(left, right - 1), left, right - 1));
        }
    }
    res
}

type RMQ = SparseTable;

struct SparseTable {
    st_max: Vec<Vec<i32>>,
    st_min: Vec<Vec<i32>>,
}

impl SparseTable {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let logn = 32 - (n as u32).leading_zeros() as usize;
        let mut st_max = vec![vec![0; logn]; n];
        let mut st_min = vec![vec![0; logn]; n];
        for (i, &num) in nums.iter().enumerate() {
            st_max[i][0] = num;
            st_min[i][0] = num;
        }
        for col in 1..logn {
            for row in 0..n {
                if row + (1 << col) > n {
                    break;
                }
                st_max[row][col] =
                    st_max[row][col - 1].max(st_max[row + (1 << (col - 1))][col - 1]);
                st_min[row][col] =
                    st_min[row][col - 1].min(st_min[row + (1 << (col - 1))][col - 1]);
            }
        }
        Self { st_max, st_min }
    }

    fn query(&self, left: usize, right: usize) -> i32 {
        let col = 31 - ((right + 1 - left) as u32).leading_zeros() as usize;
        let max = self.st_max[left][col].max(self.st_max[right - (1 << col) + 1][col]);
        let min = self.st_min[left][col].min(self.st_min[right - (1 << col) + 1][col]);
        max - min
    }
}

struct SegTree {
    max_tree: Vec<i32>,
    min_tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut s = Self {
            max_tree: vec![0; 4 * n],
            min_tree: vec![0; 4 * n],
            n,
        };
        s.build(nums, 1, 0, n - 1);
        s
    }

    fn build(&mut self, nums: &[i32], node: usize, left: usize, right: usize) -> [i32; 2] {
        if left == right {
            self.max_tree[node] = nums[left];
            self.min_tree[node] = nums[left];
            return [nums[left]; 2];
        }
        let mid = left.midpoint(right);
        let a = self.build(nums, 2 * node, left, mid);
        let b = self.build(nums, 2 * node + 1, 1 + mid, right);
        self.max_tree[node] = a[0].max(b[0]);
        self.min_tree[node] = a[1].min(b[1]);
        [self.max_tree[node], self.min_tree[node]]
    }

    fn query(&self, ql: usize, qr: usize) -> i32 {
        let [max, min] = self._query(1, 0, self.n - 1, ql, qr);
        max - min
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> [i32; 2] {
        if right < ql || qr < left {
            return [i32::MIN, i32::MAX];
        }
        if ql <= left && right <= qr {
            return [self.max_tree[node], self.min_tree[node]];
        }
        let mid = left.midpoint(right);
        let a = self._query(2 * node, left, mid, ql, qr);
        let b = self._query(2 * node + 1, 1 + mid, right, ql, qr);
        [a[0].max(b[0]), a[1].min(b[1])]
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
