mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_stability(n: i32, edges: &[[i32; 4]], k: i32) -> i32 {
    let n = n as usize;
    let mut dsu = DSU::new(n);
    let mut opt = vec![];
    let mut right = i32::MAX >> 1;
    for e in edges.iter() {
        let [a, b, s, m] = e[..] else { unreachable!() };
        let [a, b] = [a, b].map(|v| v as usize);
        if m == 1 {
            right = right.min(s);
            if !dsu.union(a, b) {
                return -1;
            }
        } else {
            opt.push((a, b, s));
        }
    }
    if dsu.n == 1 {
        return right;
    }
    opt.sort_unstable_by_key(|v| std::cmp::Reverse(v.2));
    let mut used = vec![];
    for &(a, b, s) in &opt {
        if dsu.union(a, b) {
            used.push(s);
        }
        if dsu.n == 1 {
            break;
        }
    }
    if dsu.n > 1 {
        return -1;
    }
    for v in used.iter_mut().rev().take(k as usize) {
        *v *= 2;
    }
    used.into_iter()
        .chain(std::iter::once(right))
        .min()
        .unwrap()
}

#[derive(Clone)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
    n: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            n,
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v])
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
        self.n -= 1;
        true
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
        assert_eq!(max_stability(3, &[[0, 1, 2, 1], [1, 2, 3, 0]], 1), 2);
        assert_eq!(
            max_stability(3, &[[0, 1, 4, 0], [1, 2, 3, 0], [0, 2, 1, 0]], 2),
            6
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            max_stability(
                5,
                &[
                    [0, 1, 96990, 0],
                    [2, 4, 48733, 1],
                    [0, 4, 78225, 0],
                    [3, 4, 858, 1],
                    [1, 4, 92483, 0]
                ],
                1
            ),
            858
        );
    }
}
