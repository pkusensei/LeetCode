pub struct BinaryLifting {
    pub up: Vec<Vec<usize>>,
    pub depth: Vec<i32>,
}

#[allow(dead_code)]
impl BinaryLifting {
    // Usually root = 0
    pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len(); // Could be stored
        let max_log = if n <= 1 { 1 } else { 1 + n.ilog2() as usize }; // Could be stored
        let mut s = Self {
            up: vec![vec![n; max_log]; n],
            depth: vec![0; n],
        };
        s.dfs(adj, root, n);
        // Explicit: root has no parent
        // s.up[root][0] = n;
        // Build binary lifting table
        for col in 1..max_log {
            for row in 0..n {
                let mid = s.up[row][col - 1];
                s.up[row][col] = if mid == n { n } else { s.up[mid][col - 1] };
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

    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if a == b {
            return a;
        }
        let n = self.depth.len();
        // Ensure a is not deeper than b
        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        let max_log = 1 + n.ilog2() as usize;
        // Bring b to same depth as a
        for i in 0..max_log {
            if ((self.depth[b] - self.depth[a]) >> i) & 1 == 1 {
                b = self.up[b][i];
            }
        }
        if a == b {
            return a;
        }
        // Binary search for lca
        for i in (0..max_log).rev() {
            if self.up[a][i] != n && self.up[a][i] != self.up[b][i] {
                a = self.up[a][i];
                b = self.up[b][i];
            }
        }
        self.up[a][0]
    }

    pub fn kth_ancestor(&self, mut node: usize, k: i32) -> Option<usize> {
        let n = self.depth.len();
        let max_log = 1 + n.ilog2() as usize;
        for i in 0..max_log {
            if (k >> i) & 1 == 1 {
                node = self.up[node][i];
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
