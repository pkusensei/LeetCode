mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_weight(n: i32, edges: &[[i32; 3]], src1: i32, src2: i32, dest: i32) -> i64 {
    let n = n as usize;
    let [mut adj, mut adj2] = [0, 1].map(|_| vec![vec![]; n]);
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push((b, e[2]));
        adj2[b].push((a, e[2]));
    }
    let dists1 = dijkstra(&adj, src1 as _);
    let dists2 = dijkstra(&adj, src2 as _);
    let dists3 = dijkstra(&adj2, dest as _);
    dists1
        .into_iter()
        .zip(dists2)
        .zip(dists3)
        .filter_map(|((a, b), c)| a.checked_add(b).and_then(|v| v.checked_add(c)))
        .min()
        .unwrap_or(-1)
}

fn dijkstra(adj: &[Vec<(usize, i32)>], node: usize) -> Vec<i64> {
    let n = adj.len();
    let mut dists = vec![i64::MAX; n];
    dists[node] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), node)]);
    while let Some((Reverse(cost), node)) = heap.pop() {
        if cost > dists[node] {
            continue;
        }
        for &(next, weight) in adj[node].iter() {
            let nc = cost + i64::from(weight);
            if nc < dists[next] {
                dists[next] = nc;
                heap.push((Reverse(nc), next));
            }
        }
    }
    dists
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
            minimum_weight(
                6,
                &[
                    [0, 2, 2],
                    [0, 5, 6],
                    [1, 0, 3],
                    [1, 4, 5],
                    [2, 1, 1],
                    [2, 3, 3],
                    [2, 3, 4],
                    [3, 4, 2],
                    [4, 5, 1]
                ],
                0,
                1,
                5
            ),
            9
        );
        assert_eq!(minimum_weight(3, &[[0, 1, 1], [2, 1, 1]], 0, 1, 2), -1);
    }

    #[test]
    fn test() {}
}
