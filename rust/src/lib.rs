mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_partition_factor(points: &[[i32; 2]]) -> i32 {
    let n = points.len();
        if n <= 2 {
            return 0;
        }
    let mut dists = vec![];
    for (i1, p1) in points.iter().enumerate() {
        for (i2, p2) in points.iter().enumerate().skip(1 + i1) {
            let d = (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs();
            dists.push((d, i1, i2));
        }
    }
    dists.sort_unstable_by_key(|v| v.0);
    let mut left = 0;
    let mut right = dists.last().unwrap().0;
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if check_split(n, &dists, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn check_split(n: usize, dists: &[(i32, usize, usize)], mid: i32) -> bool {
    let mut dsu = DSU::new(2 * n);
    for &(_, i1, i2) in dists.iter().take_while(|v| v.0 < mid) {
        if dsu.find(i1) == dsu.find(i2) {
            return false;
        }
        dsu.union(i1, i2 + n);
        dsu.union(i1 + n, i2);
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

    fn union(&mut self, x: usize, y: usize) -> bool {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
        true
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
        assert_eq!(max_partition_factor(&[[0, 0], [0, 2], [2, 0], [2, 2]]), 4);
        assert_eq!(max_partition_factor(&[[0, 0], [0, 1], [10, 0]]), 11);
    }

    #[test]
    fn test() {
        assert_eq!(
            max_partition_factor(&[
                [-66397, -34995],
                [-2453, -42401],
                [20537, -8704],
                [22668, -62907]
            ]),
            56687
        );
        assert_eq!(
            max_partition_factor(&[
                [7232, 86484],
                [-30449, 30516],
                [85154, 76686],
                [-41851, -41661]
            ]),
            161773
        );
    }
}
