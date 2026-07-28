mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct MajorityChecker {
    arr: Vec<i32>,
    map: HashMap<i32, Vec<usize>>,
    tree: Vec<Option<i32>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let map = arr
            .iter()
            .enumerate()
            .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &v)| {
                acc.entry(v).or_default().push(i);
                acc
            });
        let n = arr.len();
        let tree = vec![None; 4 * n];
        let mut s = Self { arr, map, tree };
        s.build(1, 0, n - 1);
        s
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let n = self.arr.len();
        if let Some([num, f]) = self._query(1, 0, n - 1, left as usize, right as usize)
            && f >= threshold
        {
            num
        } else {
            -1
        }
    }

    // [num, freq]
    fn _query(
        &self,
        node: usize,
        left: usize,
        right: usize,
        ql: usize,
        qr: usize,
    ) -> Option<[i32; 2]> {
        if qr < left || right < ql {
            return None;
        }
        if ql <= left && right <= qr {
            let num = self.tree[node]?;
            let f = self.count(num, ql, qr);
            return if f * 2 > (1 + qr - ql) as i32 {
                Some([num, f])
            } else {
                None
            };
        }
        let mid = left.midpoint(right);
        self._query(2 * node, left, mid, ql, qr)
            .or_else(|| self._query(1 + 2 * node, 1 + mid, right, ql, qr))
    }

    fn build(&mut self, node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = Some(self.arr[left]);
            return;
        }
        let mid = left.midpoint(right);
        self.build(2 * node, left, mid);
        self.build(1 + 2 * node, 1 + mid, right);
        if let Some(v) = self.tree[2 * node]
            && self.count(v, left, right) * 2 > (right + 1 - left) as i32
        {
            self.tree[node] = Some(v)
        } else if let Some(v) = self.tree[1 + 2 * node]
            && self.count(v, left, right) * 2 > (right + 1 - left) as i32
        {
            self.tree[node] = Some(v)
        }
    }

    fn count(&self, num: i32, left: usize, right: usize) -> i32 {
        let Some(arr) = self.map.get(&num) else {
            return 0;
        };
        let a = arr.partition_point(|&v| v < left);
        let b = arr.partition_point(|&v| v <= right);
        (b - a) as i32
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
    fn test() {
        let m = MajorityChecker::new(vec![2, 2, 1, 2, 1, 2, 2, 1, 1, 2]);
        assert_eq!(-1, m.query(0, 5, 6));
    }
}
