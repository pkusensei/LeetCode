mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::{
    collections::{BTreeSet, HashMap},
    sync::LazyLock,
};

#[allow(unused_imports)]
use helper::*;

pub fn maximum_count(mut nums: Vec<i32>, queries: &[[i32; 2]]) -> Vec<i32> {
    let mut p_ids = HashMap::<_, BTreeSet<_>>::new();
    for (i, &num) in nums.iter().enumerate() {
        if S[num as usize] {
            p_ids.entry(num).or_default().insert(i);
        }
    }
    let n = nums.len();
    let mut delta = vec![0; 1 + n];
    for set in p_ids.values() {
        if set.len() >= 2 {
            delta[1 + *set.first().unwrap()] += 1;
            delta[1 + *set.last().unwrap()] -= 1;
        }
    }
    for i in 1..=n {
        delta[i] += delta[i - 1];
    }
    let mut st = SegTree::new(n, &delta); // NOTE len(delta)==1+n
    let mut res = vec![];
    for q in queries.iter() {
        let idx = q[0] as usize;
        let prev = nums[idx];
        nums[idx] = q[1];
        if let Some(set) = p_ids.get_mut(&prev) {
            if set.len() >= 2 {
                let [left, right] = get_bounds(set);
                if idx == left - 1 || idx == right {
                    st.update(left, right, -1);
                    set.remove(&idx);
                    if set.len() >= 2 {
                        let [left, right] = get_bounds(set);
                        st.update(left, right, 1);
                    }
                } else {
                    set.remove(&idx);
                }
            } else {
                p_ids.remove(&prev);
            }
        }
        if S[q[1] as usize] {
            if let Some(set) = p_ids.get_mut(&q[1]) {
                let [left, right] = get_bounds(set);
                if (left - 1..=right).contains(&idx) {
                    set.insert(idx);
                } else {
                    if set.len() >= 2 {
                        st.update(left, right, -1);
                    }
                    set.insert(idx);
                    let [left, right] = get_bounds(set);
                    st.update(left, right, 1);
                }
            } else {
                p_ids.entry(q[1]).or_default().insert(idx);
            }
        }
        res.push(p_ids.len() as i32 + st.query());
    }
    res
}

const N: usize = 100_001;
static S: LazyLock<[bool; N]> = LazyLock::new(|| {
    let mut res = [true; N];
    res[..2].fill(false);
    for p in 2..=N.isqrt() {
        if res[p] {
            for val in (p * p..N).step_by(p) {
                res[val] = false;
            }
        }
    }
    res
});

fn get_bounds(set: &BTreeSet<usize>) -> [usize; 2] {
    debug_assert!(!set.is_empty());
    [1 + set.first().unwrap(), *set.last().unwrap()]
}

struct SegTree {
    tree: Vec<i32>,
    lazy: Vec<i32>,
    n: usize,
}

impl SegTree {
    fn new(n: usize, nums: &[i32]) -> SegTree {
        let mut s = SegTree {
            tree: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
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
        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
    }

    fn update(&mut self, ql: usize, qr: usize, val: i32) {
        self._update(1, 0, self.n - 1, ql, qr, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, ql: usize, qr: usize, val: i32) {
        self.push(node, left, right);
        if right < ql || qr < left {
            return;
        }
        if ql <= left && right <= qr {
            self.lazy[node] += val;
            self.push(node, left, right);
            return;
        }
        let mid = left.midpoint(right);
        self._update(2 * node, left, mid, ql, qr, val);
        self._update(2 * node + 1, 1 + mid, right, ql, qr, val);
        self.tree[node] = self.tree[2 * node].max(self.tree[2 * node + 1]);
    }

    fn query(&mut self) -> i32 {
        self.push(1, 0, self.n - 1);
        self.tree[1]
    }

    fn push(&mut self, node: usize, left: usize, right: usize) {
        if self.lazy[node] != 0 {
            self.tree[node] += self.lazy[node];
            if left != right {
                self.lazy[2 * node] += self.lazy[node];
                self.lazy[2 * node + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
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
            maximum_count(vec![2, 1, 3, 1, 2], &[[1, 2], [3, 3]]),
            [3, 4]
        );
        assert_eq!(maximum_count(vec![2, 1, 4], &[[0, 1]]), [0]);
    }

    #[test]
    fn test() {
        assert_eq!(
            maximum_count(
                vec![7, 44, 5, 13, 3, 3, 13],
                &[[2, 48], [3, 98], [3, 91], [0, 11], [6, 21], [5, 2], [2, 13]]
            ),
            [5, 4, 4, 4, 3, 3, 4]
        );
        assert_eq!(
            maximum_count(
                vec![92, 43, 11, 17, 2, 2, 81, 11, 31],
                &[
                    [7, 27],
                    [5, 13],
                    [3, 18],
                    [6, 5],
                    [2, 3],
                    [6, 5],
                    [4, 34],
                    [5, 3],
                    [8, 16]
                ]
            ),
            [6, 6, 5, 6, 6, 6, 5, 5, 4]
        );
    }
}
