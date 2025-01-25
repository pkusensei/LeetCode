mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn lexicographically_smallest_array(nums: &[i32], limit: i32) -> Vec<i32> {
    let n = nums.len();
    let mut nums: Vec<_> = nums.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    nums.sort_unstable_by_key(|&(_, v)| v);
    let mut dsu = DSU::new(n);
    for w in nums.windows(2) {
        let (x, v1) = w[0];
        let (y, v2) = w[1];
        if v2 - v1 <= limit {
            dsu.union(x, y);
        }
    }
    let mut groups = nums.into_iter().map(|(i, v)| (i, v)).rev().fold(
        HashMap::<_, Vec<_>>::new(),
        |mut acc, (i, v)| {
            acc.entry(dsu.find(i)).or_default().push(v);
            acc
        },
    );
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        let curr = groups.get_mut(&dsu.find(i)).unwrap();
        res.push(curr.pop().unwrap());
    }
    res
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i16>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
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
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            lexicographically_smallest_array(&[1, 5, 3, 9, 8], 2),
            [1, 3, 5, 8, 9]
        );
        assert_eq!(
            lexicographically_smallest_array(&[1, 7, 6, 18, 2, 1], 3),
            [1, 6, 7, 18, 1, 2]
        );
        assert_eq!(
            lexicographically_smallest_array(&[1, 7, 28, 19, 10], 3),
            [1, 7, 28, 19, 10]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
