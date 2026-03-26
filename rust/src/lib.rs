mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

//   0
// 1 x 2
//   3
pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
    use itertools::Itertools;
    let n = grid.len();
    let mut dsu = DSU::new(4 * n * n);
    for (r, s) in grid.iter().enumerate() {
        for (c, b) in s.bytes().enumerate() {
            let i = (r * n + c) * 4;
            match b {
                b'/' => {
                    dsu.union(i, i + 1);
                    dsu.union(i + 2, i + 3);
                }
                b'\\' => {
                    dsu.union(i, i + 2);
                    dsu.union(i + 1, i + 3);
                }
                _ => {
                    dsu.union(i, i + 1);
                    dsu.union(i, i + 2);
                    dsu.union(i, i + 3);
                }
            }
            if r > 0 {
                dsu.union(((r - 1) * n + c) * 4 + 3, i);
            }
            if c > 0 {
                dsu.union((r * n + c - 1) * 4 + 2, i + 1);
            }
        }
    }
    (0..4 * n * n).map(|v| dsu.find(v)).unique().count() as i32
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
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry
        } else {
            self.rank[rx] += 1;
            self.parent[ry] = rx;
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
        assert_eq!(tallest_billboard(&[3, 3]), 3);
    }

    #[test]
    fn test() {}
}
