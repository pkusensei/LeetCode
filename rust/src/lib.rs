mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn largest_component_size(nums: Vec<i32>) -> i32 {
    let mut groups = vec![None; 1 + MAX_P];
    let mut dsu = DSU::new(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        for &p in P.iter() {
            if (num as usize) % p == 0 {
                let v = groups[p].get_or_insert(i);
                dsu.union(*v, i);
            }
            if p >= num as usize {
                break;
            }
        }
    }
    dsu.max
}

const MAX_P: usize = 100_000;

static P: LazyLock<Vec<usize>> = LazyLock::new(|| {
    let mut sieve = vec![true; 1 + MAX_P];
    sieve[..2].fill(false);
    for p in 2..=MAX_P {
        if sieve[p] {
            for v in (p * p..=MAX_P).step_by(p) {
                sieve[v] = false;
            }
        }
    }
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(p, b)| if b { Some(p) } else { None })
        .collect()
});

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
    max: i32,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            max: 1,
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
        if rx == ry {
            return;
        }
        if self.size[rx] < self.size[ry] {
            self.size[ry] += self.size[rx];
            self.parent[rx] = ry;
            self.max = self.max.max(self.size[ry])
        } else {
            self.size[rx] += self.size[ry];
            self.parent[ry] = rx;
            self.max = self.max.max(self.size[rx])
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
