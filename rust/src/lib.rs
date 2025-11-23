mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_and_multiply(s: &str, queries: &[[i32; 2]]) -> Vec<i32> {
    let n = s.len();
    let mut pref_sum = Vec::with_capacity(n);
    let mut pref_x = Vec::with_capacity(n);
    let mut pref_pow = Vec::with_capacity(n);
    let mut curr_val = 0;
    for b in s.bytes() {
        let d = i64::from(b - b'0');
        pref_sum.push(d + pref_sum.last().unwrap_or(&0));
        pref_pow.push(i64::from(d > 0) + pref_pow.last().unwrap_or(&0));
        curr_val *= if d == 0 { 1 } else { 10 };
        curr_val += d;
        curr_val %= M;
        pref_x.push(curr_val);
    }
    let mut res = vec![];
    for q in queries {
        let [left, right] = [0, 1].map(|i| q[i] as usize);
        let dsum = pref_sum[right] - if left > 0 { pref_sum[left - 1] } else { 0 };
        if dsum == 0 {
            res.push(0);
            continue;
        }
        let x = pref_x[right]
            - if left > 0 {
                pref_x[left - 1] * mod_pow(10, pref_pow[right] - pref_pow[left - 1], M) % M
            } else {
                0
            };
        res.push((x.rem_euclid(M) * dsum % M) as i32);
    }
    res
}

pub fn with_segtree(s: &str, queries: &[[i32; 2]]) -> Vec<i32> {
    let st = SegTree::new(s.as_bytes());
    let mut res = vec![];
    for q in queries {
        let [x, dsum, _] = st.query(q[0] as usize, q[1] as usize);
        res.push((x * dsum % M) as i32);
    }
    res
}

const M: i64 = 1_000_000_007;

struct SegTree {
    tree: Vec<[i64; 3]>, // [range_x, sum, pow10]
    n: usize,
}

impl SegTree {
    fn new(s: &[u8]) -> Self {
        let n = s.len();
        let mut tree = Self {
            tree: vec![[0; 3]; 4 * n],
            n,
        };
        tree.build(s, 1, 0, n - 1);
        tree
    }

    fn build(&mut self, s: &[u8], node: usize, left: usize, right: usize) -> [i64; 3] {
        if left == right {
            let d = i64::from(s[left] - b'0');
            let pow10 = i64::from(d > 0);
            self.tree[node] = [d, d, pow10];
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        let a = self.build(s, 2 * node, left, mid);
        let b = self.build(s, 2 * node + 1, 1 + mid, right);
        self.tree[node] = self.merge(a, b);
        self.tree[node]
    }

    fn query(&self, ql: usize, qr: usize) -> [i64; 3] {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> [i64; 3] {
        if qr < left || right < ql {
            return [0; 3];
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        let a = self._query(2 * node, left, mid, ql, qr);
        let b = self._query(2 * node + 1, 1 + mid, right, ql, qr);
        self.merge(a, b)
    }

    fn merge(&self, left: [i64; 3], right: [i64; 3]) -> [i64; 3] {
        let [left_x, left_sum, left_pow] = left;
        let [right_x, right_sum, right_pow] = right;
        let x = (right_x + left_x * mod_pow(10, right_pow, M) % M) % M;
        let sum = left_sum + right_sum;
        let pow = left_pow + right_pow;
        [x, sum, pow]
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
            sum_and_multiply("10203004", &[[0, 7], [1, 3], [4, 6]]),
            [12340, 4, 9]
        );
        assert_eq!(sum_and_multiply("1000", &[[0, 3], [1, 1]]), [1, 0]);
        assert_eq!(sum_and_multiply("9876543210", &[[0, 9]]), [444444137]);

        assert_eq!(
            with_segtree("10203004", &[[0, 7], [1, 3], [4, 6]]),
            [12340, 4, 9]
        );
        assert_eq!(with_segtree("1000", &[[0, 3], [1, 1]]), [1, 0]);
        assert_eq!(with_segtree("9876543210", &[[0, 9]]), [444444137]);
    }

    #[test]
    fn test() {}
}
