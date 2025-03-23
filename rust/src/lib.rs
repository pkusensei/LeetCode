mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut dsu = DSU::new(n);
    for r in roads.iter() {
        let [a, b] = [0, 1].map(|i| r[i] as usize - 1);
        dsu.union(a, b, r[2]);
    }
    let root = dsu.find(0);
    dsu.edge[root]
}

struct DSU {
    parent: Vec<usize>,
    edge: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            edge: vec![i32::MAX; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize, edge: i32) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            self.edge[rx] = self.edge[rx].min(edge);
            return;
        }
        if self.edge[rx] <= self.edge[ry] {
            self.parent[ry] = rx;
            self.edge[rx] = self.edge[rx].min(edge);
        } else {
            self.parent[rx] = ry;
            self.edge[ry] = self.edge[ry].min(edge);
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
