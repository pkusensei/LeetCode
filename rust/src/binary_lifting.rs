pub struct BinaryLifting {
    pub up: Vec<Vec<usize>>,
    pub depth: Vec<i32>,
    n: usize,
    max_log: usize,
}

#[allow(dead_code)]
impl BinaryLifting {
    pub fn new(n: usize, max_log: usize) -> Self {
        Self {
            up: vec![vec![0; max_log]; n],
            depth: vec![0; n],
            n,
            max_log,
        }
    }

    pub fn build(&mut self) {
        for i2 in 1..self.max_log {
            for i1 in 0..self.n {
                self.up[i1][i2] = self.up[self.up[i1][i2 - 1]][i2 - 1];
            }
        }
    }

    pub fn lca(&self, mut node1: usize, mut node2: usize) -> usize {
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
