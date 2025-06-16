mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_active_sections_after_trade(s: &str, queries: &[[i32; 2]]) -> Vec<i32> {
    let mut total = 0;
    let mut segs = vec![];
    let mut idx = 0;
    for ch in s.as_bytes().chunk_by(|a, b| a == b) {
        idx += ch.len() as i32;
        segs.push((i32::from(ch[0] - b'0'), ch.len() as i32, idx - 1)); // (byte, length, end_idx)
        if ch[0] == b'1' {
            total += ch.len() as i32;
        }
    }
    let mut st = SegTree::new(segs);
    let mut res = vec![];
    for q in queries.iter() {
        let [left, right] = q[..] else { unreachable!() };
        if left == right {
            res.push(total);
            continue;
        }
        let ql = st.segs.partition_point(|v| v.2 < left);
        let qr = st.segs.partition_point(|v| v.2 < right);
        let curr_l = st.segs[ql].1;
        let curr_r = st.segs[qr].1;
        let new_l = curr_l - (left - if ql > 0 { 1 + st.segs[ql - 1].2 } else { 0 });
        let new_r = curr_r - (st.segs[qr].2 - right);
        st.update(ql, new_l);
        st.update(qr, new_r);
        let curr = st.query(ql, qr);
        res.push(total + curr);
        st.update(ql, curr_l);
        st.update(qr, curr_r);
    }
    res
}

struct SegTree {
    segs: Vec<(i32, i32, i32)>,
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(segs: Vec<(i32, i32, i32)>) -> Self {
        let n = segs.len();
        let mut s = Self {
            segs,
            tree: vec![-1; 4 * n],
            n,
        };
        s.build(1, 0, n - 1);
        s
    }

    fn build(&mut self, node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = 0;
            return;
        }
        let mid = left.midpoint(right);
        self.build(2 * node, left, mid);
        self.build(2 * node + 1, 1 + mid, right);
        self.merge(node, left, right, mid);
    }

    fn update(&mut self, idx: usize, val: i32) {
        self._update(1, 0, self.n - 1, idx, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i32) {
        if left == right {
            self.segs[left].1 = val;
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, val);
        } else {
            self._update(2 * node + 1, 1 + mid, right, idx, val);
        }
        self.merge(node, left, right, mid);
    }

    fn query(&self, ql: usize, qr: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        if right < ql || qr < left {
            return 0;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        let v1 = self._query(2 * node, left, mid, ql, qr);
        let v2 = self._query(2 * node + 1, 1 + mid, right, ql, qr);
        let mut max = v1.max(v2);
        if self.segs[mid].0 == 0 {
            if Self::check(mid, left, right, ql, qr) && Self::check(2 + mid, left, right, ql, qr) {
                max = max.max(self.segs[mid].1 + self.segs[2 + mid].1);
            }
        } else if Self::check(mid.wrapping_sub(1), left, right, ql, qr)
            && Self::check(1 + mid, left, right, ql, qr)
        {
            max = max.max(self.segs[1 + mid].1 + self.segs[mid - 1].1);
        }
        max
    }

    fn merge(&mut self, node: usize, left: usize, right: usize, mid: usize) {
        let mut max = self.tree[2 * node].max(self.tree[2 * node + 1]);
        if self.segs[mid].0 == 0 {
            if mid + 2 <= right {
                max = max.max(self.segs[mid].1 + self.segs[2 + mid].1);
            }
        } else if mid > left {
            max = max.max(self.segs[1 + mid].1 + self.segs[mid - 1].1);
        }
        self.tree[node] = max;
    }

    fn check(idx: usize, left: usize, right: usize, ql: usize, qr: usize) -> bool {
        (left.max(ql)..=right.min(qr)).contains(&idx)
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
        assert_eq!(max_active_sections_after_trade("01", &[[0, 1]]), [1]);
        assert_eq!(
            max_active_sections_after_trade("0100", &[[0, 3], [0, 2], [1, 3], [2, 3]]),
            [4, 3, 1, 1]
        );
        assert_eq!(
            max_active_sections_after_trade("1000100", &[[1, 5], [0, 6], [0, 4]]),
            [6, 7, 2]
        );
        assert_eq!(
            max_active_sections_after_trade("01010", &[[0, 3], [1, 4], [1, 3]]),
            [4, 4, 2]
        );
    }

    #[test]
    fn test() {}
}
