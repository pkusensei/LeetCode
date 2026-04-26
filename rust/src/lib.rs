mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_alternating_sum(nums: &[i32], k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let max = *nums.iter().max().unwrap_or(&0) as usize;
    let mut tree_inc = SegTree::new(1 + max);
    let mut tree_dec = SegTree::new(1 + max);
    let mut dp_inc = vec![0; n];
    let mut dp_dec = vec![0; n];
    let mut res = 0;
    for idx in (0..n).rev() {
        if idx + k < n {
            tree_inc.update(nums[idx + k] as usize, dp_inc[idx + k]);
            tree_dec.update(nums[idx + k] as usize, dp_dec[idx + k]);
        }
        dp_inc[idx] = i64::from(nums[idx]) + tree_dec.query(1 + nums[idx] as usize, max);
        dp_dec[idx] = i64::from(nums[idx]) + tree_inc.query(1, nums[idx] as usize - 1);
        res = res.max(dp_inc[idx]).max(dp_dec[idx]);
    }
    res
    // let mut memo = vec![vec![[-1; 2]; 1 + n]; n];
    // dfs(&nums, k, 0, n, 0, &mut memo).max(dfs(&nums, k, 0, n, 1, &mut memo))
}

// prev - prev picked value
fn dfs(
    nums: &[i32],
    k: usize,
    idx: usize,
    prev: usize,
    dir: usize,
    memo: &mut [Vec<[i64; 2]>],
) -> i64 {
    if idx >= nums.len() {
        return 0;
    }
    if memo[idx][prev][dir] > -1 {
        return memo[idx][prev][dir];
    }
    let skip = dfs(nums, k, 1 + idx, prev, dir, memo);
    let take = if dir == 0 && nums.get(prev).is_none_or(|&v| v < nums[idx]) {
        i64::from(nums[idx]) + dfs(nums, k, k + idx, idx, 1 - dir, memo)
    } else if dir == 1 && nums.get(prev).is_none_or(|&v| v > nums[idx]) {
        i64::from(nums[idx]) + dfs(nums, k, k + idx, idx, 1 - dir, memo)
    } else {
        0
    };
    memo[idx][prev][dir] = skip.max(take);
    memo[idx][prev][dir]
}

struct SegTree {
    tree: Vec<i64>,
    n: usize,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 4 * n],
            n,
        }
    }

    fn update(&mut self, idx: usize, val: i64) {
        self._update(1, 0, self.n - 1, idx, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i64) {
        if left == right {
            self.tree[node] = self.tree[node].max(val);
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, val);
        } else {
            self._update(2 * node + 1, 1 + mid, right, idx, val);
        }
        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1])
    }

    fn query(&self, ql: usize, qr: usize) -> i64 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i64 {
        if qr < left || right < ql {
            return 0;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        self._query(2 * node, left, mid, ql, qr).max(self._query(
            2 * node + 1,
            1 + mid,
            right,
            ql,
            qr,
        ))
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
        assert_eq!(max_alternating_sum(&[5, 4, 2], 2), 7);
        assert_eq!(max_alternating_sum(&[3, 5, 4, 2, 4], 1), 14);
        assert_eq!(max_alternating_sum(&[5], 1), 5);
    }

    #[test]
    fn test() {}
}
