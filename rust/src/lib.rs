mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_stability(n: i32, edges: Vec<[i32; 4]>, k: i32) -> i32 {
    let n = n as usize;
    let mut dsu = DSU::new(n);
    let mut mand_weights = vec![];
    let mut opt_edges = vec![];
    for e in edges {
        if e[3] == 1 {
            if !dsu.union(e[0] as usize, e[1] as usize) {
                return -1;
            }
            mand_weights.push(e[2]);
        } else {
            opt_edges.push((e[0] as usize, e[1] as usize, e[2]));
        }
    }
    opt_edges.sort_unstable_by(|a, b| b.2.cmp(&a.2));
    let mut opt_weights = vec![];
    for e in opt_edges {
        if dsu.n == 1 {
            break;
        }
        if dsu.find(e.0) != dsu.find(e.1) {
            dsu.union(e.0, e.1);
            opt_weights.push(e.2);
        }
    }
    if dsu.n > 1 {
        return -1;
    }
    opt_weights.sort_unstable();
    for v in opt_weights.iter_mut().take(k as usize) {
        *v *= 2;
    }
    mand_weights.into_iter().chain(opt_weights).min().unwrap()
}

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
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return false;
        }
        self.n -= 1;
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
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
        assert_eq!(max_stability(3, vec![[0, 1, 2, 1], [1, 2, 3, 0]], 1), 2);
        assert_eq!(
            max_stability(3, vec![[0, 1, 4, 0], [1, 2, 3, 0], [0, 2, 1, 0]], 2),
            6
        );
        assert_eq!(
            max_stability(3, vec![[0, 1, 1, 1], [1, 2, 1, 1], [2, 0, 1, 1]], 0),
            -1
        );
    }

    #[test]
    fn test() {}
}
