mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn count_local_maximums(matrix: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(&matrix);
    let trees = matrix.iter().map(|row| SegTree::new(row)).collect_vec();
    let mut res = 0;
    for (r, row) in matrix.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            if val > 0
                && val
                    == trees[r].query(
                        c.saturating_sub(val as usize),
                        (c + val as usize).min(cols - 1),
                    )
            {
                let mut flag = true;
                for d in 0..val {
                    let ql = if d == val - 1 {
                        c.saturating_sub(val as usize - 1)
                    } else {
                        c.saturating_sub(val as usize)
                    };
                    let qr = if d == val - 1 {
                        (c + val as usize - 1).min(cols - 1)
                    } else {
                        (c + val as usize).min(cols - 1)
                    };
                    if r >= 1 + d as usize && val < trees[r - 1 - d as usize].query(ql, qr) {
                        flag = false;
                        break;
                    }
                    if (r + 1 + d as usize) < rows && val < trees[r + 1 + d as usize].query(ql, qr)
                    {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    res += 1;
                }
            }
        }
    }
    res
}

struct SegTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(row: &[i32]) -> Self {
        let n = row.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            n,
        };
        s.build(1, 0, n - 1, row);
        s
    }

    fn build(&mut self, node: usize, left: usize, right: usize, row: &[i32]) {
        if left == right {
            self.tree[node] = row[left];
            return;
        }
        let mid = left.midpoint(right);
        self.build(2 * node, left, mid, row);
        self.build(2 * node + 1, 1 + mid, right, row);
        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1])
    }

    fn query(&self, ql: usize, qr: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
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
        assert_eq!(
            count_local_maximums(&[
                &[0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 2, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0]
            ]),
            1
        );
    }

    #[test]
    fn test() {
        assert_eq!(count_local_maximums(&[&[0, 2, 1], &[1, 1, 0]]), 2)
    }
}
