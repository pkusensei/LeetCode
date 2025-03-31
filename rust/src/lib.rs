mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::{cmp::Reverse, collections::BinaryHeap};

struct Graph {
    adj: Vec<Vec<i32>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut adj = vec![vec![0; n]; n];
        for e in edges {
            let [a, b, c] = e[..] else { unreachable!() };
            adj[a as usize][b as usize] = c;
        }
        Self { adj }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let [a, b, c] = edge[..] else { unreachable!() };
        self.adj[a as usize][b as usize] = c;
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut dists = vec![i32::MAX; self.adj.len()];
        let [n1, n2] = [node1, node2].map(|v| v as usize);
        let mut heap = BinaryHeap::from([(Reverse(0), n1)]);
        while let Some((Reverse(dist), node)) = heap.pop() {
            if node == n2 {
                return dist;
            }
            if dists[node] < dist {
                continue;
            }
            for (next, &val) in self.adj[node].iter().enumerate() {
                if val > 0 {
                    let nd = dist + val;
                    if nd < dists[next] {
                        dists[next] = nd;
                        heap.push((Reverse(nd), next));
                    }
                }
            }
        }
        -1
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
        let mut g = Graph::new(
            4,
            vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]],
        );
        assert_eq!(g.shortest_path(3, 2), 6);
        assert_eq!(g.shortest_path(0, 3), -1);
        g.add_edge(vec![1, 3, 4]);
        assert_eq!(g.shortest_path(0, 3), 6);
    }

    #[test]
    fn test() {}
}
