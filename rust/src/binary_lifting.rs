pub struct BinaryLifting {
    pub up: Vec<Vec<usize>>,
    pub depth: Vec<i32>,
    n: usize,
    max_log: usize,
}

#[allow(dead_code)]
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

    pub fn lca(&self, mut node1: usize, mut node2: usize) -> usize {
        if self.depth[node1] > self.depth[node2] {
            std::mem::swap(&mut node1, &mut node2);
        }
        for i in 0..self.max_log {
            if ((self.depth[node2] - self.depth[node1]) >> i) & 1 == 1 {
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
