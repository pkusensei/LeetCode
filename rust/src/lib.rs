mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: &[[i32; 2]]) -> i32 {
    const M: i64 = 1_000_000_007;
    let mut res = 0;
    let mut tree = SegmentTree::build(&nums);
    for q in queries {
        res += tree.insert(0, 0, nums.len(), q[0] as usize, q[1])[3];
    }
    (res % M) as i32
}

struct SegmentTree {
    tree: Vec<[i64; 4]>,
}

impl SegmentTree {
    fn build(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![[0; 4]; 4 * n],
        };
        for (i, &num) in nums.iter().enumerate() {
            s.insert(0, 0, n, i, num);
        }
        s
    }

    fn insert(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i32) -> [i64; 4] {
        if left == right {
            self.tree[node][3] = val.into();
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        self.tree[node] = if idx <= mid {
            let v = self.insert(2 * node + 1, left, mid, idx, val);
            Self::merge(v, self.tree[2 * node + 2])
        } else {
            let v = self.insert(2 * node + 2, 1 + mid, right, idx, val);
            Self::merge(self.tree[2 * node + 1], v)
        };
        self.tree[node]
    }

    fn merge(a: [i64; 4], b: [i64; 4]) -> [i64; 4] {
        [
            (a[1] + b[0]).max(a[0] + b[2]), // _XX + _X_, _X_ + XX_
            (a[1] + b[1]).max(a[0] + b[3]), // _XX + _XX, _X_ + XXX
            (a[3] + b[0]).max(a[2] + b[2]), // XXX + _XX, XX_ + XX_
            (a[3] + b[1]).max(a[2] + b[3]), // XXX + _XX, XX_ + XXX
        ]
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
            maximum_sum_subsequence(vec![3, 5, 9], &[[1, -2], [0, -3]]),
            21
        );
        assert_eq!(maximum_sum_subsequence(vec![0, -1], &[[0, -5]]), 0);
    }

    #[test]
    fn test() {}
}
