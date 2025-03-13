mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    if edges.is_empty(){return (n*(n-1)/2) as i64}
    let mut dsu = DSU::new(n);
    for e in edges.iter() {
        dsu.union(e[0] as usize, e[1] as usize);
    }
    let map = (0..n).fold(std::collections::HashMap::new(), |mut acc, v| {
        let root = dsu.find(v);
        let size = dsu.size[root];
        acc.entry(root).or_insert(size);
        acc
    });
    let mut res = 0;
    for (i, a) in map.values().enumerate() {
        for b in map.values().skip(1 + i) {
            res += a * b;
        }
    }
    res
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i64>,
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
            self.size[ry] += self.size[rx];
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
