mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
    use itertools::{Itertools, izip};
    use std::collections::{BTreeMap, BTreeSet, HashMap};
    let x_id = x_coord.iter().sorted().fold(HashMap::new(), |mut acc, &x| {
        let id = acc.len();
        acc.entry(x).or_insert(id);
        acc
    });
    let y_xs = izip!(x_coord.iter(), y_coord.iter()).fold(
        BTreeMap::<i32, BTreeSet<_>>::new(),
        |mut acc, (&x, &y)| {
            acc.entry(y).or_default().insert(x); // All points are unique => set
            acc
        },
    );
    let n = x_id.len();
    let mut tree = SegmentTree::new(n);
    // max prev_y on x
    let mut prev_y = HashMap::new();
    let mut res = 0;
    for (&y2, xs) in y_xs.iter() {
        // Nothing comes in between (x1, x2) on y=y2 line
        for (&x1, &x2) in izip!(xs.iter(), xs.iter().skip(1)) {
            if let Some((&y11, &y12)) = prev_y.get(&x1).zip(prev_y.get(&x2)) {
                if y11 != y12 {
                    continue;
                }
                // Found two lower corners/rectangle
                let [i1, i2] = [x1, x2].map(|x| x_id[&x]);
                // Ensures no enclosed points
                if tree.query(1, 0, n - 1, 1 + i1, i2 - 1) < y11 {
                    res = res.max(i64::from(x2 - x1) * i64::from(y2 - y11));
                }
            }
        }
        // BTreeMap ensures y2 is sorted => always updating bigger value
        for &x in xs {
            tree.update(1, 0, n - 1, x_id[&x], y2);
            prev_y
                .entry(x)
                .and_modify(|v| *v = (*v).max(y2))
                .or_insert(y2);
        }
    }
    if res > 0 { res } else { -1 }
}

struct SegmentTree {
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![-1; 4 * n],
        }
    }

    fn query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if ql > qr {
            return i32::MIN;
        }
        if left == ql && right == qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        self.query(2 * node, left, mid, ql, qr.min(mid))
            .max(self.query(2 * node + 1, 1 + mid, right, ql.max(1 + mid), qr))
    }

    fn update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i32) {
        if left == right {
            self.tree[node] = val;
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self.update(2 * node, left, mid, idx, val);
        } else {
            self.update(2 * node + 1, 1 + mid, right, idx, val);
        }
        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
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
        assert_eq!(max_rectangle_area(&[1, 1, 3, 3], &[1, 3, 1, 3]), 4);
        assert_eq!(max_rectangle_area(&[1, 1, 3, 3, 2], &[1, 3, 1, 3, 2]), -1);
        assert_eq!(
            max_rectangle_area(&[1, 1, 3, 3, 1, 3], &[1, 3, 1, 3, 2, 2]),
            2
        );
    }

    #[test]
    fn test() {
        assert_eq!(max_rectangle_area(&[87, 77, 87, 77], &[40, 37, 37, 40]), 30);
        assert_eq!(
            max_rectangle_area(
                &[
                    87, 77, 87, 77, 64, 7, 18, 51, 27, 69, 22, 68, 56, 52, 53, 56, 40, 85, 40, 76
                ],
                &[
                    40, 37, 37, 40, 66, 52, 93, 60, 5, 2, 52, 27, 66, 43, 63, 96, 31, 55, 40, 70
                ]
            ),
            30
        );
    }
}
