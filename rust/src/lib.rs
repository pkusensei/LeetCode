mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn can_traverse_all_pairs(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return true;
    }
    let mut set = HashSet::new();
    let mut max = 0;
    for &num in nums.iter() {
        if num == 1 {
            return false;
        }
        set.insert(num);
        max = max.max(num);
    }
    let mut sieve = vec![true; 1 + max as usize];
    sieve[0..2].copy_from_slice(&[false; 2]);
    let mut dsu = DSU::new(1 + max as usize);
    for p in 2..=max as usize {
        if sieve[p] {
            for val in (p + p..=max as usize).step_by(p) {
                sieve[val] = false;
                if set.contains(&(val as i32)) {
                    dsu.union(p, val);
                }
            }
        }
    }
    let root = dsu.find(nums[0] as usize);
    for &num in nums.iter() {
        if dsu.find(num as usize) != root {
            return false;
        }
    }
    true
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
                self.parent[ry] = rx;
                self.rank[rx] += 1;
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
        assert!(can_traverse_all_pairs(&[2, 3, 6]));
        assert!(!can_traverse_all_pairs(&[3, 9, 5]));
        assert!(can_traverse_all_pairs(&[4, 3, 12, 8]));
    }

    #[test]
    fn test() {}
}
