mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn lexicographically_smallest_array(nums: &[i32], limit: i32) -> Vec<i32> {
    let n = nums.len();
    let sorted = (0..n).sorted_by_key(|&i| nums[i]).collect_vec();
    let mut dsu = DSU::new(n);
    for w in sorted.windows(2) {
        let [a, b] = w[..] else { unreachable!() };
        if nums[b] - nums[a] <= limit {
            dsu.union(a, b);
        }
    }
    let mut groups = HashMap::<usize, Vec<_>>::new();
    for i in sorted {
        groups.entry(dsu.find(i)).or_default().push(i);
    }
    let mut res = vec![0; n];
    for i in (0..n).rev() {
        let root = dsu.find(i);
        let group = groups.get_mut(&root).unwrap();
        res[i] = nums[group.pop().unwrap()];
    }
    res
}

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v])
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        self.parent[ry] = rx;
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
        assert_eq!(
            lexicographically_smallest_array(&[1, 5, 3, 9, 8], 2),
            [1, 3, 5, 8, 9]
        );
    }

    #[test]
    fn test() {}
}
