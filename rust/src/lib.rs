mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn assign_edge_weights(edges: &[[i32; 2]], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let jump = BinaryLifting::new(&adj);
    queries
        .iter()
        .map(|q| {
            let [a, b] = [0, 1].map(|i| q[i] as usize - 1);
            if a == b {
                return 0;
            }
            let lca_ = jump.lca(a, b);
            let d = jump.depth[a] + jump.depth[b] - 2 * jump.depth[lca_];
            mod_pow(2, d - 1) as i32
        })
        .collect()
}

const M: i64 = 1_000_000_007;

const fn mod_pow(b: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % M, exp >> 1)
    } else {
        mod_pow(b * b % M, exp >> 1) * b % M
    }
}

struct BinaryLifting {
    up: Vec<Vec<usize>>,
    depth: Vec<i64>,
    n: usize,
    max_log: usize,
}

impl BinaryLifting {
    fn new(adj: &[Vec<usize>]) -> Self {
        let n = adj.len();
        let max_log = 1 + n.ilog2() as usize;
        let mut s = Self {
            up: vec![vec![0; max_log]; n],
            depth: vec![0; n],
            n,
            max_log,
        };
        s.dfs(adj, 0, n);
        for i2 in 1..max_log {
            for i1 in 0..n {
                s.up[i1][i2] = s.up[s.up[i1][i2 - 1]][i2 - 1];
            }
        }
        s
    }

    fn dfs(&mut self, adj: &[Vec<usize>], node: usize, prev: usize) {
        for &next in &adj[node] {
            if next != prev {
                self.up[next][0] = node;
                self.depth[next] = 1 + self.depth[node];
                self.dfs(adj, next, node);
            }
        }
    }

    fn lca(&self, mut n1: usize, mut n2: usize) -> usize {
        if self.depth[n1] > self.depth[n2] {
            std::mem::swap(&mut n1, &mut n2);
        }
        for i in 0..self.max_log {
            if ((self.depth[n2] - self.depth[n1]) >> i) & 1 == 1 {
                n2 = self.up[n2][i];
            }
        }
        if n1 == n2 {
            return n1;
        }
        for i in (0..self.max_log).rev() {
            if self.up[n1][i] != self.up[n2][i] {
                n1 = self.up[n1][i];
                n2 = self.up[n2][i];
            }
        }
        self.up[n1][0]
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
        assert_eq!(assign_edge_weights(&[[1, 2]], &[[1, 1], [1, 2]]), [0, 1]);
        assert_eq!(
            assign_edge_weights(&[[1, 2], [1, 3], [3, 4], [3, 5]], &[[1, 4], [3, 4], [2, 5]]),
            [2, 1, 4]
        );
    }

    #[test]
    fn test() {}
}
