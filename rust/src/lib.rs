mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_complete_components(n: i32, edges: &[[i32; 2]]) -> i32 {
    use std::collections::HashSet;
    let n = n as usize;
    let mut dsu = DSU::new(n);
    for e in edges.iter() {
        dsu.union(e[0] as _, e[1] as _);
    }
    (0..n)
        .map(|v| dsu.find(v))
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|&v| {
            let e = dsu.edges[v];
            let size = dsu.size[v];
            size * (size - 1) / 2 == e
        })
        .count() as _
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
    edges: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            edges: vec![0; n],
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
            self.edges[rx] += 1;
            return;
        }
        if self.size[rx] < self.size[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];
            self.edges[ry] += 1 + self.edges[rx];
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
            self.edges[rx] += 1 + self.edges[ry];
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
    fn basics() {
        assert_eq!(
            count_complete_components(6, &[[0, 1], [0, 2], [1, 2], [3, 4]]),
            3
        );
        assert_eq!(
            count_complete_components(6, &[[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]]),
            1
        );
    }

    #[test]
    fn test() {}
}
