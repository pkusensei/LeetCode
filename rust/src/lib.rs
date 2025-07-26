mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct NumArrayBIT {
    tree: Vec<i64>,
    nums: Vec<i32>,
    n: usize,
}

impl NumArrayBIT {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 1 + n],
            nums: vec![],
            n,
        };
        for (i, &num) in nums.iter().enumerate() {
            s._update(1 + i, num.into());
        }
        s.nums = nums;
        s
    }

    fn _update(&mut self, mut idx: usize, val: i64) {
        while idx <= self.n {
            self.tree[idx] += val;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn update(&mut self, idx: i32, val: i32) {
        let i = idx as usize;
        let delta = i64::from(val - self.nums[i]);
        self.nums[i] = val;
        self._update(1 + i, delta);
    }

    fn query(&self, idx: usize) -> i64 {
        let mut i = idx;
        let mut res = 0;
        while i > 0 {
            res += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        res
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        (self.query(1 + right as usize) - self.query(left as _)) as i32
    }
}

struct NumArrayST {
    tree: Vec<i32>,
    n: usize,
}

impl NumArrayST {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            n,
        };
        s.build(1, 0, n - 1, &nums);
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
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
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
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    fn update(&mut self, idx: i32, val: i32) {
        self._update(1, 0, self.n - 1, idx as usize, val);
    }

    fn query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if qr < left || right < ql {
            return 0;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        self.query(2 * node, left, mid, ql, qr) + self.query(2 * node + 1, 1 + mid, right, ql, qr)
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.query(1, 0, self.n - 1, left as _, right as _)
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
        let mut na = NumArrayBIT::new(vec![1, 3, 5]);
        assert_eq!(na.sum_range(0, 2), 9);
        na.update(1, 2); // [1, 2, 5]
        assert_eq!(na.sum_range(0, 2), 8);

        let mut st = NumArrayST::new(vec![1, 3, 5]);
        assert_eq!(st.sum_range(0, 2), 9);
        st.update(1, 2); // [1, 2, 5]
        assert_eq!(st.sum_range(0, 2), 8);
    }

    #[test]
    fn test() {}
}
