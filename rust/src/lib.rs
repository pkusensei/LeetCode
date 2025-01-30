mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distance_limited_paths_exist(
    n: i32,
    edge_list: &mut [[i32; 3]],
    queries: &[[i32; 3]],
) -> Vec<bool> {
    let n = n as usize;
    edge_list.sort_unstable_by_key(|e| e[2]);
    let mut query_indices: Vec<_> = (0..queries.len()).collect();
    query_indices.sort_unstable_by_key(|&i| queries[i][2]);
    let mut res = vec![false; queries.len()];
    let mut dsu = DSU::new(n);
    let mut ei = 0;
    for qi in query_indices {
        let [a, b, limit] = [0, 1, 2].map(|i| queries[qi][i]);
        while edge_list.get(ei).is_some_and(|e| e[2] < limit) {
            let [x, y] = [0, 1].map(|i| edge_list[ei][i] as usize);
            dsu.union(x, y);
            ei += 1;
        }
        res[qi] = dsu.is_connected(a as _, b as _);
    }
    res
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

    fn union(&mut self, x: usize, y: usize) -> bool {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
        true
    }

    fn is_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
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
            distance_limited_paths_exist(
                3,
                &mut [[0, 1, 2], [1, 2, 4], [2, 0, 8], [1, 0, 16]],
                &[[0, 1, 2], [0, 2, 5]]
            ),
            [false, true]
        );
        assert_eq!(
            distance_limited_paths_exist(
                5,
                &mut [[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]],
                &[[0, 4, 14], [1, 4, 13]]
            ),
            [true, false]
        )
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
