mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rectangle_area(rectangles: &[[i32; 4]]) -> i32 {
    let n = rectangles.len();
    let mut xs = Vec::with_capacity(2 * n);
    let mut y_items = Vec::with_capacity(2 * n);
    let mut min_y = i32::MAX;
    for rect in rectangles.iter() {
        let [x1, y1, x2, y2] = rect[..] else {
            unreachable!()
        };
        xs.extend([x1, x2]);
        min_y = min_y.min(y1);
        y_items.push([y1, x1, x2, 1]);
        y_items.push([y2, x1, x2, -1]);
    }
    xs.sort_unstable();
    xs.dedup();
    y_items.sort_unstable();
    let mut st = SegTree::new(xs);
    let mut res = 0;
    for item in y_items {
        let [y, x1, x2, v] = item;
        res = (res + i64::from(y - min_y) * i64::from(st.query())) % 1_000_000_007;
        min_y = y;
        st.update(x1, x2, v);
    }
    res as i32
}

struct SegTree {
    n: usize,
    xs: Vec<i32>,
    covered: Vec<i32>,
    count: Vec<i32>,
}

impl SegTree {
    fn new(xs: Vec<i32>) -> Self {
        let n = xs.len() - 1;
        Self {
            n,
            xs,
            covered: vec![0; 4 * n],
            count: vec![0; 4 * n],
        }
    }

    fn query(&self) -> i32 {
        self.covered[1]
    }

    fn update(&mut self, x1: i32, x2: i32, val: i32) {
        self._update(1, 0, self.n - 1, x1, x2, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, ql: i32, qr: i32, val: i32) {
        if self.xs[1 + right] <= ql || qr <= self.xs[left] {
            return;
        }
        if ql <= self.xs[left] && self.xs[1 + right] <= qr {
            self.count[node] += val;
        } else {
            let mid = left.midpoint(right);
            self._update(2 * node, left, mid, ql, qr, val);
            self._update(2 * node + 1, 1 + mid, right, ql, qr, val);
        }
        if self.count[node] > 0 {
            self.covered[node] = self.xs[1 + right] - self.xs[left];
        } else if left == right {
            self.covered[node] = 0;
        } else {
            self.covered[node] = self.covered[2 * node] + self.covered[2 * node + 1];
        }
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
            rectangle_area(&[[0, 0, 2, 2], [1, 0, 2, 3], [1, 0, 3, 1]]),
            6
        );
    }

    #[test]
    fn test() {}
}
