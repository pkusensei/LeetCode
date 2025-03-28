mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn handle_query(nums1: &[i32], nums2: &[i32], queries: &[[i32; 3]]) -> Vec<i64> {
    let n = nums1.len();
    let mut tree = SegmentTree::build(nums1);
    let mut sum: i64 = nums2.iter().map(|&v| i64::from(v)).sum();
    let mut res = vec![];
    for q in queries.iter() {
        match q[0] {
            1 => tree.update(1, 0, n - 1, [q[1] as usize, q[2] as usize]),
            2 => {
                let count = tree.query(1, 0, n - 1, [0, n - 1]);
                sum += i64::from(q[1]) * count;
            }
            _ => res.push(sum),
        }
    }
    res
}

struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    fn build(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
        };
        for (i, &v) in nums.iter().enumerate() {
            s.add(1, i, v.into(), 0, n - 1);
        }
        s
    }

    fn add(&mut self, u: usize, idx: usize, val: i64, left: usize, right: usize) {
        if left == right {
            self.tree[u] = val;
            return;
        }
        let mid = left + (right - left) / 2;
        if idx <= mid {
            self.add(2 * u, idx, val, left, mid);
        } else {
            self.add(2 * u + 1, idx, val, 1 + mid, right);
        }
        self.tree[u] = self.tree[2 * u] + self.tree[2 * u + 1];
    }

    fn update(&mut self, u: usize, left: usize, right: usize, range: [usize; 2]) {
        if self.lazy[u] != 0 {
            self.tree[u] = (right + 1 - left) as i64 - self.tree[u];
            if left != right {
                self.lazy[2 * u] ^= 1;
                self.lazy[2 * u + 1] ^= 1;
            }
            self.lazy[u] ^= 1;
        }
        if left > right || range[1] < left || right < range[0] {
            return;
        }
        if range[0] <= left && right <= range[1] {
            self.tree[u] = (right + 1 - left) as i64 - self.tree[u];
            if left != right {
                self.lazy[2 * u] ^= 1;
                self.lazy[2 * u + 1] ^= 1;
            }
            return;
        }
        let mid = left + (right - left) / 2;
        self.update(2 * u, left, mid, range);
        self.update(2 * u + 1, 1 + mid, right, range);
        self.tree[u] = self.tree[2 * u] + self.tree[2 * u + 1];
    }

    fn query(&mut self, u: usize, left: usize, right: usize, range: [usize; 2]) -> i64 {
        if self.lazy[u] != 0 {
            self.tree[u] = (right + 1 - left) as i64 - self.tree[u];
            if left != right {
                self.lazy[2 * u] ^= 1;
                self.lazy[2 * u + 1] ^= 1;
            }
            self.lazy[u] ^= 1;
        }
        if left > right || range[1] < left || right < range[0] {
            return 0;
        }
        if range[0] <= left && right <= range[1] {
            return self.tree[u];
        }
        let mid = left + (right - left) / 2;
        self.query(2 * u, left, mid, range) + self.query(2 * u + 1, 1 + mid, mid, range)
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
        assert_eq!(
            handle_query(&[1, 0, 1], &[0, 0, 0], &[[1, 1, 1], [2, 1, 0], [3, 0, 0]]),
            [3]
        );
        assert_eq!(handle_query(&[1], &[5], &[[2, 0, 0], [3, 0, 0]]), [5]);
    }

    #[test]
    fn test() {}
}
