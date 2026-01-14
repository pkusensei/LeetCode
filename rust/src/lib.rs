mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_area(squares: &[[i32; 3]]) -> i32 {
    let (y_items, xs, min_y) = preprocess(squares);
    let mut st = SegTree::new(&xs);
    let mut total = 0;
    let mut prev_y = min_y;
    for item in &y_items {
        let &[y, x1, x2, type_] = item;
        total += (y - prev_y) * (st.query());
        st.update(x1, x2, type_);
        prev_y = y;
    }
    total
}

pub fn separate_squares(squares: &[[i32; 3]]) -> f64 {
    let n = squares.len();
    let (y_items, xs, min_y) = preprocess(squares);
    let mut st = SegTree::new(&xs);
    let mut total = 0;
    let mut prev_y = min_y;
    let mut prefix = Vec::with_capacity(n);
    let mut x_lens = Vec::with_capacity(n);
    for item in &y_items {
        let &[y, x1, x2, type_] = item;
        total += i64::from(y - prev_y) * i64::from(st.query());
        st.update(x1, x2, type_);
        prev_y = y;
        prefix.push(total);
        x_lens.push(st.query());
    }
    split_with_binary_search(&y_items, total, &prefix, &x_lens)
    // mid_split(&y_items, &xs, min_y, total)
}

fn split_with_binary_search(
    y_items: &[[i32; 4]],
    total: i64,
    prefix: &[i64],
    x_lens: &[i32],
) -> f64 {
    let i = prefix.partition_point(|&v| v < (1 + total) / 2) - 1;
    let y = f64::from(y_items[i][0]);
    let x = f64::from(x_lens[i]);
    y + (total as f64 - prefix[i] as f64 * 2.0) / (x * 2.0)
}

fn mid_split(y_items: &[[i32; 4]], xs: &[i32], min_y: i32, total: i64) -> f64 {
    let mut st = SegTree::new(&xs);
    let mut curr = total as f64 / 2.0;
    let mut prev_y = min_y;
    for item in y_items {
        let &[y, x1, x2, type_] = item;
        let len = f64::from(st.query());
        let temp = len * f64::from(y - prev_y);
        if curr - temp <= 0.0 {
            return f64::from(prev_y) + curr / len;
        }
        curr -= temp;
        st.update(x1, x2, type_);
        prev_y = y;
    }
    unreachable!()
}

fn preprocess(squares: &[[i32; 3]]) -> (Vec<[i32; 4]>, Vec<i32>, i32) {
    let n = squares.len();
    let mut y_items = Vec::with_capacity(2 * n);
    let mut xs = Vec::with_capacity(2 * n);
    let mut min_y = i32::MAX;
    for sq in squares.iter() {
        let [x, y, side] = sq[..] else { unreachable!() };
        min_y = min_y.min(y);
        xs.extend([x, x + side]);
        y_items.push([y, x, x + side, 1]); // line sweep enters square
        y_items.push([y + side, x, x + side, -1]); // line sweep leaves square
    }
    y_items.sort_unstable();
    xs.sort_unstable();
    xs.dedup(); // unique x-vals
    (y_items, xs, min_y)
}

struct SegTree<'a> {
    xs: &'a [i32],
    count: Vec<i32>,   // accumulated count from line sweep
    covered: Vec<i32>, // covered length of x-chunks
    n: usize,
}

impl<'a> SegTree<'a> {
    fn new(xs: &'a [i32]) -> Self {
        let n = xs.len() - 1;
        Self {
            xs,
            count: vec![0; 4 * n],
            covered: vec![0; 4 * n],
            n,
        }
    }

    // Total length of all "covered"(count>0) chunks on x-axis
    // avoid using segment to prevent confusion
    fn query(&self) -> i32 {
        self.covered[1]
    }

    fn update(&mut self, ql: i32, qr: i32, val: i32) {
        self._update(1, 0, self.n - 1, ql, qr, val);
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
            // This x-chunk is covered
            self.covered[node] = self.xs[1 + right] - self.xs[left]
        } else if left == right {
            self.covered[node] = 0; // empty node
        } else {
            // propagate changes
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
        float_eq!(separate_squares(&[[0, 0, 1], [2, 2, 1]]), 1.0);
        float_eq!(separate_squares(&[[0, 0, 2], [1, 1, 1]]), 1.0);

        assert_eq!(total_area(&[[0, 0, 1], [2, 2, 1]]), 2);
        assert_eq!(total_area(&[[0, 0, 2], [1, 1, 1]]), 4);
    }

    #[test]
    fn test() {
        float_eq!(separate_squares(&[[10, 30, 3], [17, 27, 4]]), 30.07143);

        assert_eq!(total_area(&[[10, 30, 3], [17, 27, 4]]), 25);
    }
}
