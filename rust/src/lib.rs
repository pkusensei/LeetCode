mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut pref_max = nums.to_vec();
    for i in 1..n {
        pref_max[i] = pref_max[i].max(pref_max[i - 1])
    }
    let mut suf_min = nums.to_vec();
    for i in (0..n - 1).rev() {
        suf_min[i] = suf_min[i].min(suf_min[1 + i]);
    }
    let mut res = Vec::with_capacity(n);
    let mut start = 0;
    for i in 0..n - 1 {
        if pref_max[i] <= suf_min[1 + i] {
            for _ in start..=i {
                res.push(pref_max[i]);
            }
            start = 1 + i;
        }
    }
    for _ in start..n {
        res.push(pref_max[n - 1]);
    }
    res
    // let mut dsu = DSU::new(nums);
    // for i in 0..n - 1 {
    //     if pref_max[i] > suf_min[1 + i] {
    //         dsu.union(i, 1 + i);
    //     }
    // }
    // (0..n)
    //     .map(|i| {
    //         let root = dsu.find(i);
    //         dsu.nums[root]
    //     })
    //     .collect()
}

struct DSU {
    parent: Vec<usize>,
    nums: Vec<i32>,
}

impl DSU {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        Self {
            parent: (0..n).collect(),
            nums,
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
        if self.nums[rx] >= self.nums[ry] {
            self.parent[ry] = rx
        } else {
            self.parent[rx] = ry;
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}
