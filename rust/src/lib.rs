mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    let groups = meetings
        .iter()
        .fold(BTreeMap::<_, HashSet<_>>::new(), |mut acc, me| {
            let [a, b, t] = me[..] else { unreachable!() };
            acc.entry(t).or_default().insert([a, b]);
            acc
        });
    let mut dsu = DSU::new(n as usize);
    dsu.union(0, first_person as usize);
    for group in groups.values() {
        for &[a, b] in group.iter() {
            dsu.union(a as usize, b as usize);
        }
        for &[a, b] in group.iter() {
            if dsu.find(a as usize) != dsu.find(0) && dsu.find(a as usize) != dsu.find(0) {
                dsu.reset(a as usize);
                dsu.reset(b as usize);
            }
        }
    }
    (0..n)
        .filter(|&v| dsu.find(v as usize) == dsu.find(0))
        .collect()
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
            std::cmp::Ordering::Less => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
    }

    fn reset(&mut self, v: usize) {
        self.parent[v] = v;
        self.rank[v] = 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
