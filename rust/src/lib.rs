mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_hamming_distance(
    source: Vec<i32>,
    target: Vec<i32>,
    allowed_swaps: Vec<Vec<i32>>,
) -> i32 {
    let n = source.len();
    let mut dsu = DSU::new(n);
    for s in allowed_swaps.iter() {
        dsu.union(s[0] as usize, s[1] as usize);
    }
    let mut map = HashMap::<_, HashMap<_, i32>>::new();
    for (i, &num) in source.iter().enumerate() {
        let root = dsu.find(i);
        *map.entry(root).or_default().entry(num).or_insert(0) += 1;
    }
    for (i, &num) in target.iter().enumerate() {
        let root = dsu.find(i);
        *map.entry(root).or_default().entry(num).or_insert(0) -= 1;
    }
    map.iter()
        .flat_map(|(_, m)| m.values().map(|v| v.abs()))
        .sum::<i32>()
        / 2
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
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        if self.size[rx] < self.size[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx]
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
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
    fn basics() {}

    #[test]
    fn test() {}
}
