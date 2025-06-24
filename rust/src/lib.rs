mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_weight(edges: &[[i32; 3]], queries: &[[i32; 3]]) -> Vec<i32> {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, e[2]));
        acc[b].push((a, e[2]));
        acc
    });
    let max_log = 1 + n.ilog2() as usize;
    let mut jump = BinaryLifting::new(n, max_log);
    let mut dist = vec![0; n];
    dfs(&adj, 0, n, &mut dist, &mut jump);
    jump.build();
    queries
        .iter()
        .map(|q| {
            let [s1, s2, d] = [0, 1, 2].map(|i| q[i] as usize);
            (distance(s1, s2, &jump, &dist)
                + distance(s1, d, &jump, &dist)
                + distance(s2, d, &jump, &dist))
                / 2
        })
        .collect()
}

fn dfs(
    adj: &[Vec<(usize, i32)>],
    node: usize,
    prev: usize,
    dist: &mut [i32],
    jump: &mut BinaryLifting,
) {
    for &(next, w) in &adj[node] {
        if next != prev {
            jump.up[next][0] = node;
            jump.depth[next] = 1 + jump.depth[node];
            dist[next] = w + dist[node];
            dfs(adj, next, node, dist, jump);
        }
    }
}

fn distance(node1: usize, node2: usize, jump: &BinaryLifting, dist: &[i32]) -> i32 {
    let lca_ = jump.lca(node1, node2);
    dist[node1] + dist[node2] - 2 * dist[lca_]
}

struct BinaryLifting {
    up: Vec<Vec<usize>>,
    depth: Vec<i32>,
    n: usize,
    max_log: usize,
}

impl BinaryLifting {
    fn new(n: usize, max_log: usize) -> Self {
        Self {
            up: vec![vec![0; max_log]; n],
            depth: vec![0; n],
            n,
            max_log,
        }
    }

    fn build(&mut self) {
        for i2 in 1..self.max_log {
            for i1 in 0..self.n {
                self.up[i1][i2] = self.up[self.up[i1][i2 - 1]][i2 - 1];
            }
        }
    }

    fn lca(&self, mut node1: usize, mut node2: usize) -> usize {
        if self.depth[node1] > self.depth[node2] {
            std::mem::swap(&mut node1, &mut node2);
        }
        for i in 0..self.max_log {
            if (self.depth[node2] - self.depth[node1]) >> i & 1 == 1 {
                node2 = self.up[node2][i];
            }
        }
        if node1 == node2 {
            return node1;
        }
        for i in (0..self.max_log).rev() {
            if self.up[node1][i] != self.up[node2][i] {
                node1 = self.up[node1][i];
                node2 = self.up[node2][i];
            }
        }
        self.up[node1][0]
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
        assert_eq!(
            minimum_weight(
                &[[0, 1, 2], [1, 2, 3], [1, 3, 5], [1, 4, 4], [2, 5, 6]],
                &[[2, 3, 4], [0, 2, 5]]
            ),
            [12, 11]
        );
        assert_eq!(minimum_weight(&[[1, 0, 8], [0, 2, 7]], &[[0, 1, 2]]), [15])
    }

    #[test]
    fn test() {
        assert_eq!(
            minimum_weight(
                &[[0, 3, 3], [1, 2, 8], [2, 3, 4]],
                &[[2, 1, 0], [2, 1, 3], [2, 3, 1], [1, 2, 0]]
            ),
            [15, 12, 12, 15]
        );
    }
}
