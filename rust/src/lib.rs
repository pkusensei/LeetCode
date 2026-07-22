mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut arr = vec![Node {
        val: 1,
        len: 0,
        end_idx: -1,
    }];
    let mut total_one = 0;
    let mut idx = -1;
    for ch in s.as_bytes().chunk_by(|a, b| a == b) {
        idx += ch.len() as i32;
        arr.push(Node {
            val: ch[0] - b'0',
            len: ch.len() as i32,
            end_idx: idx,
        });
        total_one += i32::from(ch[0] - b'0') * ch.len() as i32;
    }

    let mut st = SegTree::new(arr);
    let mut res = vec![];
    for q in queries.iter() {
        if q[0] == q[1] {
            res.push(total_one);
            continue;
        }
        let ql = st.arr.partition_point(|v| v.end_idx < q[0]);
        let qr = st.arr.partition_point(|v| v.end_idx < q[1]);
        let [templ, tempr] = [ql, qr].map(|i| st.arr[i].len);
        let lenl = templ - (q[0] - st.arr[ql - 1].end_idx - 1);
        let lenr = tempr - (st.arr[qr].end_idx - q[1]);
        st.update(ql, lenl);
        st.update(qr, lenr);
        res.push(total_one + st.query(ql, qr));
        st.update(ql, templ);
        st.update(qr, tempr);
    }
    res
}

#[derive(Clone, Copy)]
struct Node {
    val: u8,
    len: i32,
    end_idx: i32,
}

struct SegTree {
    arr: Vec<Node>,
    tree: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(arr: Vec<Node>) -> Self {
        let n = arr.len();
        let mut s = Self {
            arr,
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
        self.merge(node, left, mid, right);
    }

    fn merge(&mut self, node: usize, left: usize, mid: usize, right: usize) {
        let mut maxv = self.tree[2 * node].max(self.tree[1 + 2 * node]);
        if self.arr[mid].val == 0 {
            if 2 + mid <= right {
                maxv = maxv.max(self.arr[mid].len + self.arr[2 + mid].len);
            }
        } else {
            if left < mid {
                maxv = maxv.max(self.arr[mid - 1].len + self.arr[1 + mid].len);
            }
        }
        self.tree[node] = maxv;
    }

    fn update(&mut self, idx: usize, len: i32) {
        self._update(1, 0, self.n - 1, idx, len);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, len: i32) {
        if left == right {
            self.arr[left].len = len;
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, len);
        } else {
            self._update(1 + 2 * node, 1 + mid, right, idx, len);
        }
        self.merge(node, left, mid, right);
    }

    fn query(&self, ql: usize, qr: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        fn check(l1: usize, l2: usize, r1: usize, r2: usize, v1: usize, v2: usize) -> bool {
            (l1.max(l2)..=r1.min(r2)).contains(&v1) && (l1.max(l2)..=r1.min(r2)).contains(&v2)
        }

        if right < ql || qr < left {
            return 0;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        let a = self._query(2 * node, left, mid, ql, qr);
        let b = self._query(1 + 2 * node, 1 + mid, right, ql, qr);
        let mut res = a.max(b);
        if self.arr[mid].val == 0 {
            if check(left, ql, right, qr, mid, 2 + mid) {
                res = res.max(self.arr[mid].len + self.arr[2 + mid].len);
            }
        } else {
            if check(left, ql, right, qr, mid - 1, 1 + mid) {
                res = res.max(self.arr[mid - 1].len + self.arr[1 + mid].len)
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused)]
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
    fn basics() {}

    #[test]
    fn test() {}
}
