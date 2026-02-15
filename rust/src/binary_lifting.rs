pub struct BinaryLifting {
    pub up: Vec<Vec<usize>>,
    pub depth: Vec<i32>,
}

#[allow(dead_code)]
impl BinaryLifting {
    // Usually root = 0
    fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len(); // Could be stored
        let max_log = if n <= 1 { 1 } else { 1 + n.ilog2() as usize }; // Could be stored
        let mut s = Self {
            up: vec![vec![n; n]; max_log],
            depth: vec![0; n],
        };
        s.dfs(adj, root, n);
        // Explicit: root has no parent
        // s.up[0][root] = n;
        // Build binary lifting table
        for i1 in 1..max_log {
            for i2 in 0..n {
                let mid = s.up[i1 - 1][i2];
                s.up[i1][i2] = if mid == n { n } else { s.up[i1 - 1][mid] };
            }
        }
        s
    }

    fn dfs(&mut self, adj: &[Vec<usize>], node: usize, prev: usize) {
        for &next in &adj[node] {
            if next != prev {
                self.up[0][next] = node;
                self.depth[next] = 1 + self.depth[node];
                self.dfs(adj, next, node);
            }
        }
    }

    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.depth.len();
        // Ensure a is not deeper than b
        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        let max_log = 1 + n.ilog2() as usize;
        // Bring b to same depth as a
        for i in 0..max_log {
            if ((self.depth[b] - self.depth[a]) >> i) & 1 == 1 {
                b = self.up[i][b];
            }
        }
        if a == b {
            return a;
        }
        // Binary search for lca
        for i in (0..max_log).rev() {
            if self.up[i][a] != n && self.up[i][a] != self.up[i][b] {
                a = self.up[i][a];
                b = self.up[i][b];
            }
        }
        self.up[0][a]
    }

    pub fn kth_ancestor(&self, mut node: usize, k: i32) -> Option<usize> {
        let n = self.depth.len();
        let max_log = 1 + n.ilog2() as usize;
        for i in 0..max_log {
            if (k >> i) & 1 == 1 {
                node = self.up[i][node];
                if node == n {
                    return None;
                }
            }
        }
        Some(node)
    }

    pub fn distance(&self, a: usize, b: usize) -> i32 {
        let lca = self.lca(a, b);
        self.depth[a] + self.depth[b] - 2 * self.depth[lca]
    }
}
