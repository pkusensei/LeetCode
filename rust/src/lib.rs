mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_alternating_sum(nums: &[i32], swaps: &[[i32; 2]]) -> i64 {
    use std::collections::HashMap;
    let n = nums.len();
    let mut dsu = DSU::new(n);
    for s in swaps {
        dsu.union(s[0] as usize, s[1] as usize);
    }
    let mut groups = HashMap::<_, Pack>::new();
    for (idx, &num) in nums.iter().enumerate() {
        let root = dsu.find(idx);
        let pack = groups.entry(root).or_default();
        pack.odd += idx & 1;
        pack.nums.push(num.into());
    }
    let mut res = 0;
    for mut pack in groups.into_values() {
        pack.nums.sort_unstable();
        res +=
            pack.nums[pack.odd..].iter().sum::<i64>() - pack.nums[..pack.odd].iter().sum::<i64>();
    }
    res
}

#[derive(Default)]
struct Pack {
    nums: Vec<i64>,
    odd: usize,
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
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
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
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
    fn basics() {
        assert_eq!(max_alternating_sum(&[1, 2, 3], &[[0, 2], [1, 2]]), 4);
        assert_eq!(max_alternating_sum(&[1, 2, 3], &[[1, 2]]), 2);
        assert_eq!(
            max_alternating_sum(&[1, 1000000000, 1, 1000000000, 1, 1000000000], &[]),
            -2999999997
        );
    }

    #[test]
    fn test() {}
}
