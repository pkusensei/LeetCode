mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_time(n: i32, mut edges: Vec<[i32; 3]>, k: i32) -> i32 {
    edges.sort_unstable_by_key(|e| e[2]);
    let mut left = 0;
    let mut right = edges.last().map(|e| e[2]).unwrap_or(0);
    while left < right {
        let mid = left + (right - left) / 2;
        let mut dsu = DSU::new(n as usize);
        for e in edges.iter().rev().take_while(|e| e[2] > mid) {
            dsu.union(e[0] as usize, e[1] as usize);
        }
        if dsu.n >= k as usize {
            right = mid
        } else {
            left = 1 + mid;
        }
    }
    left
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
    n: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            n,
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
        self.n -= 1;
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx
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
    fn basics() {}

    #[test]
    fn test() {}
}
