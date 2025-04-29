mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn find_answer(n: i32, edges: &[[i32; 3]]) -> Vec<bool> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, e[2]));
        acc[b].push((a, e[2]));
        acc
    });
    let dists = dijkstra(n, &adj);
    if dists[n - 1] == i32::MAX {
        return vec![false; edges.len()];
    }
    let mut path = vec![false; n];
    dfs(&adj, &dists, n - 1, n, &mut path);
    edges
        .iter()
        .map(|e| {
            let [a, b] = [0, 1].map(|i| e[i] as usize);
            path[a] && path[b] && (dists[a] - dists[b]).abs() == e[2]
        })
        .collect()
}

fn dijkstra(n: usize, adj: &[Vec<(usize, i32)>]) -> Vec<i32> {
    let mut dists = vec![i32::MAX; n];
    dists[0] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(dist), node)) = heap.pop() {
        if dists[node] < dist {
            continue;
        }
        for &(next, d) in &*adj[node] {
            let nd = d + dist;
            if nd < dists[next] {
                dists[next] = nd;
                heap.push((Reverse(nd), next));
            }
        }
    }
    dists
}

fn dfs(
    adj: &[Vec<(usize, i32)>],
    dists: &[i32],
    node: usize,
    prev: usize,
    path: &mut [bool],
) -> bool {
    // memoization
    if node == 0 || path[node] {
        path[node] = true;
        return true;
    }
    let mut res = false;
    for &(next, d) in &adj[node] {
        if next != prev && dists[next] + d == dists[node] {
            res |= dfs(adj, dists, next, node, path);
        }
    }
    path[node] = res;
    res
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
            find_answer(
                6,
                &[
                    [0, 1, 4],
                    [0, 2, 1],
                    [1, 3, 2],
                    [1, 4, 3],
                    [1, 5, 1],
                    [2, 3, 1],
                    [3, 5, 3],
                    [4, 5, 2]
                ]
            ),
            [true, true, true, false, true, true, true, false]
        );
        assert_eq!(
            find_answer(4, &[[2, 0, 1], [0, 1, 1], [0, 3, 4], [3, 2, 2]]),
            [true, false, false, true]
        );
    }

    #[test]
    fn test() {
        assert_eq!(find_answer(3, &[[2, 1, 6]]), [false]);
    }
}
