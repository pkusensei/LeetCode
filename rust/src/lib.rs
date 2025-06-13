mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_unplaced_fruits(fruits: &[i32], baskets: &[i32]) -> i32 {
    let mut tree = SegTree::build(&baskets);
    let mut res = 0;
    for &f in fruits.iter() {
        if !tree.query(f) {
            res += 1;
        }
    }
    res
}

struct SegTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn build(nums: &[i32]) -> Self {
        let n = nums.len();
        let tree = vec![0; 4 * n];
        let mut s = Self { tree, n };
        for (idx, &val) in nums.iter().enumerate() {
            s.update(idx, val);
        }
        s
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
        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
    }

    fn query(&mut self, val: i32) -> bool {
        self._query(1, 0, self.n - 1, val)
    }

    fn _query(&mut self, node: usize, left: usize, right: usize, val: i32) -> bool {
        if self.tree[node] < val {
            return false;
        }
        if left == right {
            if self.tree[node] >= val {
                self.tree[node] = 0;
                return true;
            }
            return false;
        }
        let mid = left.midpoint(right);
        let res =
            self._query(2 * node, left, mid, val) || self._query(2 * node + 1, 1 + mid, right, val);
        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
        res
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
        assert_eq!(num_of_unplaced_fruits(&[4, 2, 5], &[3, 5, 4]), 1);
        assert_eq!(num_of_unplaced_fruits(&[3, 6, 1], &[6, 4, 7]), 0);
    }

    #[test]
    fn test() {}
}
