mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_segment_sum(nums: &[i32], remove_queries: &[i32]) -> Vec<i64> {
    let n = nums.len();
    let mut res = vec![0; n];
    let mut dsu = DSU::new(n);
    for (idx, &q) in remove_queries.iter().enumerate().rev() {
        res[idx] = dsu.max;
        let i = q as usize;
        dsu.update(i, i64::from(nums[i]));
        if i > 0 && dsu.get(i - 1) > 0 {
            dsu.union(i, i - 1);
        }
        if i + 1 < n && dsu.get(1 + i) > 0 {
            dsu.union(i, 1 + i);
        }
    }
    res
}

struct DSU {
    parent: Vec<usize>,
    sums: Vec<i64>,
    max: i64,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            sums: vec![0; n],
            max: 0,
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        if self.sums[rx] < self.sums[ry] {
            self.sums[ry] += self.sums[rx];
            self.parent[rx] = ry;
            self.max = self.max.max(self.sums[ry]);
        } else {
            self.sums[rx] += self.sums[ry];
            self.parent[ry] = rx;
            self.max = self.max.max(self.sums[rx]);
        }
    }

    fn update(&mut self, i: usize, val: i64) {
        self.sums[i] += val;
        self.max = self.max.max(self.sums[i]);
    }

    fn get(&mut self, v: usize) -> i64 {
        let root = self.find(v);
        self.sums[root]
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
            maximum_segment_sum(&[1, 2, 5, 6, 1], &[0, 3, 2, 4, 1]),
            [14, 7, 2, 2, 0]
        );
        assert_eq!(
            maximum_segment_sum(&[3, 2, 11, 1], &[3, 2, 1, 0]),
            [16, 5, 3, 0]
        );
    }

    #[test]
    fn test() {}
}
