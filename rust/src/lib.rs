mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut dsu = DSU::new(n);
    for e in edges {
        dsu.union(e[0] as usize, e[1] as usize);
    }
    let mut seen = 0_i64;
    let mut res = 0;
    for i in 0..n {
        let root = dsu.find(i);
        if seen & (1 << root) == 0 {
            seen |= 1 << root;
            let a = dsu.size[root];
            let b = dsu.edge[root];
            res += i32::from(a * (a - 1) / 2 == b);
        }
    }
    res
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
    edge: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            edge: vec![0; n],
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
            self.edge[rx] += 1;
            return;
        }
        if self.size[rx] < self.size[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];
            self.edge[ry] += 1 + self.edge[rx]
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
            self.edge[rx] += 1 + self.edge[ry];
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
