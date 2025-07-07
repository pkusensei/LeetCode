mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_stable(nums: &[i32], max_c: i32) -> i32 {
    let st = SegTree::new(&nums);
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left.midpoint(right);
        if check(&st, max_c, mid) {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left as i32
}

fn check(st: &SegTree, mut max_c: i32, mid: usize) -> bool {
    let len = 1 + mid;
    if len > st.n {
        return false;
    }
    let mut i = 0;
    while i <= st.n - len {
        if st.query(i, i + len - 1) >= 2 {
            max_c -= 1;
            if max_c < 0 {
                return false;
            }
            i += len;
        } else {
            i += 1
        }
    }
    max_c >= 0
}

struct SegTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            n,
        };
        s.build(1, 0, n - 1, nums);
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
        self.tree[node] = gcd(self.tree[2 * node], self.tree[2 * node + 1])
    }

    fn query(&self, ql: usize, qr: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if right < ql || qr < left {
            return 0; // ??
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        gcd(
            self._query(2 * node, left, mid, ql, qr),
            self._query(2 * node + 1, 1 + mid, right, ql, qr),
        )
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let [a, b] = [a.min(b), a.max(b)];
    if a == 0 { b } else { gcd(b % a, a) }
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
        assert_eq!(min_stable(&[3, 5, 10], 1), 1);
        assert_eq!(min_stable(&[2, 6, 8], 2), 1);
        assert_eq!(min_stable(&[2, 4, 9, 6], 1), 2);
    }

    #[test]
    fn test() {
        assert_eq!(min_stable(&[6, 5], 2), 0);
        assert_eq!(min_stable(&[2, 2], 0), 2)
    }
}
