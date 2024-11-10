#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
    pub size: Vec<i32>,
    count: usize,
}

#[allow(dead_code)]
impl DSU {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            size: vec![1; size],
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
        if x == y {
            return false;
        }
        let (x, y) = (x.min(y), x.max(y));
        let (rx, ry) = (self.find(x), self.find(y));
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => {
                self.parent[rx] = ry;
                self.size[ry] += self.size[rx];
            }
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
                self.size[rx] += self.size[ry];
            }
            std::cmp::Ordering::Greater => {
                self.parent[ry] = rx;
                self.size[rx] += self.size[ry];
            }
        }
        self.count -= 1;
        true
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn get_size(&mut self, x: usize) -> i32 {
        let p = self.find(x);
        self.size[p]
    }
}
