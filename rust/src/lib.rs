mod dsu;
mod helper;
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
    for e in allowed_swaps.iter() {
        dsu.union(e[0] as _, e[1] as _);
    }
    let mut s_groups = HashMap::<_, HashMap<_, _>>::new();
    for (i, &num) in source.iter().enumerate() {
        *s_groups
            .entry(dsu.find(i))
            .or_default()
            .entry(num)
            .or_insert(0) += 1;
    }
    let mut t_groups = HashMap::<_, HashMap<_, _>>::new();
    for (i, &num) in target.iter().enumerate() {
        *t_groups
            .entry(dsu.find(i))
            .or_default()
            .entry(num)
            .or_insert(0) += 1;
    }
    let mut common = 0;
    for (group, s_counts) in s_groups.into_iter() {
        let Some(t_counts) = t_groups.remove(&group) else {
            break;
        };
        for (num, count) in s_counts {
            if let Some(&v) = t_counts.get(&num) {
                common += count.min(v);
            }
        }
    }
    n as i32 - common
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
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
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
