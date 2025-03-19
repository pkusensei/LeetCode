mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::Itertools;
use std::collections::HashMap;

pub fn number_of_good_paths(vals: &[i32], edges: &[[i32; 2]]) -> i32 {
    let n = vals.len();
    let mut count = vals.iter().map(|&v| HashMap::from([(v, 1)])).collect_vec();
    let edges = edges
        .iter()
        .map(|e| {
            let [a, b] = [0, 1].map(|i| e[i] as usize);
            (vals[a].max(vals[b]), a, b)
        })
        .sorted_unstable()
        .collect_vec();
    let mut dsu = DSU::new(n);
    let mut res = 0;
    for &(val, node1, node2) in edges.iter() {
        let [r1, r2] = [node1, node2].map(|v| dsu.find(v));
        let [c1, c2] = [r1, r2].map(|v| *count[v].get(&val).unwrap_or(&0));
        res += c1 * c2;
        dsu.parent[r2] = r1;
        count[r1] = HashMap::from([(val, c1 + c2)]);
    }
    res + n as i32
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
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
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
        assert_eq!(
            number_of_good_paths(&[1, 3, 2, 1, 3], &[[0, 1], [0, 2], [2, 3], [2, 4]]),
            6
        );
        assert_eq!(
            number_of_good_paths(&[1, 1, 2, 2, 3], &[[0, 1], [1, 2], [2, 3], [2, 4]]),
            7
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            number_of_good_paths(
                &[2, 5, 5, 1, 5, 2, 3, 5, 1, 5],
                &[
                    [0, 1],
                    [2, 1],
                    [3, 2],
                    [3, 4],
                    [3, 5],
                    [5, 6],
                    [1, 7],
                    [8, 4],
                    [9, 7]
                ]
            ),
            20
        );
    }
}
