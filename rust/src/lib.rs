mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct BookMyShow {
    n: usize,
    m: usize,
    tree: Vec<(i32, i64)>, // (max_len, sum_len)
}

impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        fn build(u: usize, left: usize, right: usize, m: usize, tree: &mut [(i32, i64)]) {
            if left == right {
                tree[u] = (m as _, m as _);
                return;
            }
            let mid = left + (right - left) / 2;
            build(2 * u, left, mid, m, tree);
            build(2 * u + 1, 1 + mid, right, m, tree);
            let max = tree[2 * u].0.max(tree[2 * u + 1].0);
            let sum = tree[2 * u].1 + tree[2 * u + 1].1;
            tree[u] = (max, sum);
        }

        let [n, m] = [n, m].map(|v| v as usize);
        let mut tree = vec![(0, 0); 4 * n];
        build(1, 0, n - 1, m, &mut tree);
        Self { n, m, tree }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        self.gather_impl(1, 0, self.n - 1, k, max_row as _)
    }

    fn gather_impl(
        &mut self,
        u: usize,
        left: usize,
        right: usize,
        k: i32,
        max_row: usize,
    ) -> Vec<i32> {
        if left > max_row || self.tree[u].0 < k {
            return vec![];
        }
        if left == right {
            self.tree[u].0 -= k;
            self.tree[u].1 -= i64::from(k);
            return vec![left as i32, self.m as i32 - self.tree[u].0 - k];
        }
        let mid = left + (right - left) / 2;
        let res = if self.tree[2 * u].0 >= k {
            self.gather_impl(2 * u, left, mid, k, max_row)
        } else {
            self.gather_impl(2 * u + 1, 1 + mid, right, k, max_row)
        };
        self.tree[u].0 = self.tree[2 * u].0.max(self.tree[2 * u + 1].0);
        self.tree[u].1 = self.tree[2 * u].1 + self.tree[2 * u + 1].1;
        res
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        if self.check(1, 0, self.n - 1, k, max_row as _) {
            self.scatter_impl(1, 0, self.n - 1, k);
            true
        } else {
            false
        }
    }

    fn check(&self, u: usize, left: usize, right: usize, k: i32, max_row: usize) -> bool {
        if left > max_row || self.tree[u].1 < i64::from(k) {
            return false;
        }
        if left == right {
            return true;
        }
        let mid = left + (right - left) / 2;
        if 1 + mid > max_row {
            return self.check(2 * u, left, mid, k, max_row);
        }
        if i64::from(k) <= self.tree[2 * u].1 {
            return true;
        }
        self.check(
            2 * u + 1,
            1 + mid,
            right,
            k - self.tree[2 * u].1 as i32,
            max_row,
        )
    }

    fn scatter_impl(&mut self, u: usize, left: usize, right: usize, k: i32) {
        if left == right {
            self.tree[u].0 -= k;
            self.tree[u].1 -= i64::from(k);
            return;
        }
        let mid = left + (right - left) / 2;
        let remain = i64::from(k) - self.tree[2 * u].1;
        self.scatter_impl(2 * u, left, mid, (k as i64).min(self.tree[2 * u].1) as i32);
        if remain > 0 {
            self.scatter_impl(2 * u + 1, 1 + mid, right, remain as i32);
        }
        self.tree[u].0 = self.tree[2 * u].0.max(self.tree[2 * u + 1].0);
        self.tree[u].1 = self.tree[2 * u].1 + self.tree[2 * u + 1].1;
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
        let mut bms = BookMyShow::new(2, 5); // There are 2 rows with 5 seats each
        assert_eq!(bms.gather(4, 0), [0, 0]); // return [0, 0]
                                              // The group books seats [0, 3] of row 0.
        assert!(bms.gather(2, 0).is_empty()); // return []
                                              // There is only 1 seat left in row 0,
                                              // so it is not possible to book 2 consecutive seats.
        assert!(bms.scatter(5, 1)); // return True
                                    // The group books seat 4 of row 0 and seats [0, 3] of row 1.
        assert!(!bms.scatter(5, 1));
    }

    #[test]
    fn test() {}
}
