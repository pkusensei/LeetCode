#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
    size: Vec<i32>, // size of component at this root
    count: usize,   // number of component
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

    pub fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let [rx, ry] = [x, y].map(|v| self.find(v));
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

    pub fn get_size(&mut self, v: usize) -> i32 {
        let root = self.find(v);
        self.size[root]
    }
}
