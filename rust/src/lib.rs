mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_activated(points: &[[i32; 2]]) -> i32 {
    use itertools::Itertools;
    use std::{cmp::Reverse, collections::HashMap};

    let n = points.len();
    let mut dsu = DSU::new(n);
    let mut x_maps = HashMap::new();
    let mut y_maps = HashMap::new();
    for (i, p) in points.iter().enumerate() {
        let [x, y] = p[..] else { unreachable!() };
        let vx = x_maps.entry(x).or_insert(vec![]);
        vx.push(i);
        dsu.union(vx[0], i);
        let vy = y_maps.entry(y).or_insert(vec![]);
        vy.push(i);
        dsu.union(vy[0], i);
    }
    let mut sizes = HashMap::new();
    for i in 0..n {
        let root = dsu.find(i);
        let size = dsu.size[root];
        sizes.insert(root, size);
    }
    if sizes.len() < 2 {
        1 + n as i32
    } else {
        let mut v = sizes.into_values().collect_vec();
        v.select_nth_unstable_by_key(1, |&v| Reverse(v));
        1 + v[..2].iter().sum::<i32>()
    }
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v])
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        if self.size[rx] < self.size[ry] {
            self.size[ry] += self.size[rx];
            self.parent[rx] = ry;
        } else {
            self.size[rx] += self.size[ry];
            self.parent[ry] = rx;
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
        assert_eq!(max_activated(&[[2, 3], [2, 2], [1, 1], [4, 5]]), 4);
    }

    #[test]
    fn test() {}
}
