mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_results(queries: &[&[i32]]) -> Vec<bool> {
    let max = queries.iter().map(|q| q[1] as usize).max().unwrap_or(0);
    let mut tree = SegmentTree::new(2 + max);
    let mut res = vec![];
    for q in queries.iter() {
        if q[0] == 1 {
            tree.insert(0, 0, tree.n - 1, q[1] as usize);
        } else {
            let [x, sz] = [1, 2].map(|i| q[i] as usize);
            let state = tree.query(0, 0, tree.n - 1, 0, x);
            if let Some([a, b]) = state.obs {
                let diff = state.diff.max(a).max(x.saturating_sub(b));
                res.push(sz <= diff);
            } else {
                res.push(sz <= 1 + x);
            }
        }
    }
    res
}

#[derive(Clone, Copy, Default)]
struct State {
    obs: Option<[usize; 2]>,
    diff: usize,
}

impl State {
    fn merge(self, other: State) -> Self {
        match (self.obs, other.obs) {
            (None, _) => other,
            (Some(_), None) => self,
            (Some([a1, a2]), Some([b1, b2])) => {
                let diff = self.diff.max(other.diff).max(b1 - a2);
                Self {
                    obs: Some([a1, b2]),
                    diff,
                }
            }
        }
    }
}

struct SegmentTree {
    tree: Vec<State>,
    n: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut s = Self {
            tree: vec![Default::default(); 4 * n],
            n,
        };
        s.insert(0, 0, n - 1, 0);
        s
    }

    fn insert(&mut self, node: usize, left: usize, right: usize, idx: usize) {
        if left == right {
            self.tree[node] = State {
                obs: Some([idx; 2]),
                diff: 0,
            };
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self.insert(2 * node + 1, left, mid, idx);
        } else {
            self.insert(2 * node + 2, 1 + mid, right, idx);
        }
        self.tree[node] = self.tree[2 * node + 1].merge(self.tree[2 * node + 2]);
    }

    fn query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> State {
        if qr < left || right < ql {
            return Default::default();
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        let a = self.query(2 * node + 1, left, mid, ql, qr);
        let b = self.query(2 * node + 2, 1 + mid, right, ql, qr);
        a.merge(b)
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
        assert_eq!(
            get_results(&[&[1, 2], &[2, 3, 3], &[2, 3, 1], &[2, 2, 2]]),
            [false, true, true]
        );
        assert_eq!(
            get_results(&[&[1, 7], &[2, 7, 6], &[1, 2], &[2, 7, 5], &[2, 7, 6]]),
            [true, true, false]
        );
    }

    #[test]
    fn test() {}
}
