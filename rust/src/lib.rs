mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    use itertools::Itertools;
    use std::collections::HashSet;
    let mut events = vec![];
    let mut xs = HashSet::new();
    for sq in &squares {
        let [x, y, side] = sq[..] else { unreachable!() };
        events.push(YEvent {
            y,
            x1: x,
            x2: x + side,
            type_: 1,
        });
        events.push(YEvent {
            y: y + side,
            x1: x,
            x2: x + side,
            type_: -1,
        });
        xs.extend([x, x + side]);
    }
    events.sort_unstable();
    let xs = xs.into_iter().sorted_unstable().collect_vec();

    let mut tree = SegTree::new(&xs);
    let mut total_area = 0.0;
    let mut prev_y = events[0].y;
    for &YEvent { y, x1, x2, type_ } in &events {
        total_area += f64::from(tree.query()) * f64::from(y - prev_y);
        tree.update(x1, x2, type_);
        prev_y = y;
    }

    tree = SegTree::new(&xs);
    let target = total_area / 2.0;
    let mut curr_area = 0.0;
    prev_y = events[0].y;
    for &YEvent { y, x1, x2, type_ } in &events {
        let width = f64::from(tree.query());
        if curr_area + width * f64::from(y - prev_y) >= target {
            return f64::from(prev_y) + (target - curr_area) / width;
        }
        curr_area += width * f64::from(y - prev_y);
        tree.update(x1, x2, type_);
        prev_y = y;
    }
    unreachable!()
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct YEvent {
    y: i32,
    x1: i32,
    x2: i32,
    type_: i32,
}

struct SegTree<'a> {
    xs: &'a [i32],
    count: Vec<i32>,
    covered: Vec<i32>,
    n: usize,
}

impl<'a> SegTree<'a> {
    fn new(xs: &'a [i32]) -> Self {
        let n = xs.len() - 1;
        let count = vec![0; 4 * n];
        let covered = vec![0; 4 * n];
        Self {
            xs,
            count,
            covered,
            n,
        }
    }

    fn query(&self) -> i32 {
        self.covered[1]
    }

    fn update(&mut self, ql: i32, qr: i32, val: i32) {
        self._update(1, 0, self.n - 1, ql, qr, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, ql: i32, qr: i32, val: i32) {
        if self.xs[1 + right] <= ql || self.xs[left] >= qr {
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
        float_eq!(separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]), 1.0);
        float_eq!(separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]), 1.0);
    }

    #[test]
    fn test() {}
}
