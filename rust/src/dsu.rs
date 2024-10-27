#[derive(Debug, Clone)]
pub struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
    count: usize,
}

impl DSU {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            count: size,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
        self.count -= 1;
        true
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
