mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
    let k = threshold as usize;
    let mut dsu = DSU::new(1 + k);
    for &num in &nums {
        let num = num as usize;
        for v in (num..=k).step_by(num) {
            dsu.union(num, v);
        }
    }
    let mut seen = std::collections::HashSet::new();
    let mut res = 0;
    for &num in &nums {
        let num = num as usize;
        if num <= k {
            seen.insert(dsu.find(num as usize));
        } else {
            res += 1
        }
    }
    seen.len() as i32 + res
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
            self.parent[v] = self.find(self.parent[v])
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
    fn basics() {}

    #[test]
    fn test() {}
}
