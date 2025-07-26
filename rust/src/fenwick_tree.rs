pub struct FenwickTree {
    pub tree: Vec<i32>,
    pub size: usize,
}

#[allow(dead_code)]
impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![0; 1 + n],
            size: n,
        }
    }

    // !! 1-based index !!
    pub fn update(&mut self, mut idx: usize, val: i32) {
        while idx <= self.size {
            self.tree[idx] += val;
            idx += idx & (!idx + 1); // same as wrapping_neg
        }
    }

    pub fn query(&self, mut idx: usize) -> i32 {
        let mut res = 0;
        while idx > 0 {
            res += self.tree[idx];
            idx -= idx & idx.wrapping_neg();
        }
        res
    }
}
