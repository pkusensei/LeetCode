mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn count_restricted_paths(n: i32, edges: &[[i32; 3]]) -> i32 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        let w = e[2];
        acc[a].push((b, w));
        acc[b].push((a, w));
        acc
    });
    let dists = dijkstra(&adj);
    dfs(&adj, &dists, 0, None, &mut vec![-1; n])
}

fn dfs(
    adj: &[Vec<(usize, i32)>],
    dists: &[i32],
    node: usize,
    prev: Option<usize>,
    memo: &mut [i32],
) -> i32 {
    if prev.is_some_and(|p| dists[p] <= dists[node]) {
        return 0;
    }
    let n = adj.len();
    if memo[node] > -1 {
        return memo[node];
    }
    if node == n - 1 {
        return 1;
    }
    let mut res = 0;
    for &(next, _) in adj[node].iter() {
        if prev.is_some_and(|p| p == next) {
            continue;
        }
        res += dfs(adj, dists, next, Some(node), memo);
        res %= 1_000_000_007;
    }
    memo[node] = res;
    res
}

fn dijkstra(adj: &[Vec<(usize, i32)>]) -> Vec<i32> {
    let n = adj.len();
    let mut res = vec![i32::MAX; n];
    let mut heap = BinaryHeap::from([(Reverse(0), n - 1)]);
    res[n - 1] = 0;
    while let Some((Reverse(cost), node)) = heap.pop() {
        if cost < res[node] {
            continue;
        }
        for &(next, weight) in adj[node].iter() {
            let nw = cost + weight;
            if nw < res[next] {
                res[next] = nw;
                heap.push((Reverse(nw), next));
            }
        }
    }
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
            count_restricted_paths(
                5,
                &[
                    [1, 2, 3],
                    [1, 3, 3],
                    [2, 3, 1],
                    [1, 4, 2],
                    [5, 2, 2],
                    [3, 5, 1],
                    [5, 4, 10]
                ]
            ),
            3
        );
        assert_eq!(
            count_restricted_paths(
                7,
                &[
                    [1, 3, 1],
                    [4, 1, 2],
                    [7, 3, 4],
                    [2, 5, 3],
                    [5, 6, 1],
                    [6, 7, 2],
                    [7, 5, 3],
                    [2, 6, 4]
                ]
            ),
            1
        );
    }

    #[test]
    fn test() {}
}
