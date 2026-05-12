mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_threshold(n: i32, edges: &[[i32; 3]], source: i32, target: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    for e in edges {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push((b, e[2]));
        adj[b].push((a, e[2]));
    }
    let mut left = 0;
    let mut right = 1_000_000_001;
    let mut possible = false;
    while left < right {
        let mid = left + (right - left) / 2;
        if bfs(&adj, source as usize, target as usize, k, mid) {
            possible = true;
            right = mid
        } else {
            left = 1 + mid
        }
    }
    if possible { left } else { -1 }
}

pub fn bfs_01(adj: &[Vec<(usize, i32)>], start: usize, goal: usize) -> i32 {
    let n = adj.len();
    let mut queue = VecDeque::from([(start, 0)]);
    let mut dists = vec![i32::MAX >> 1; n];
    dists[start] = 0;
    while let Some((node, dist)) = queue.pop_front() {
        if node == goal {
            return dist;
        }
        for &(next, d) in &adj[node] {
            let nd = dist + d;
            if dists[next] > nd {
                dists[next] = nd;
                if d == 0 {
                    queue.push_front((next, nd));
                } else {
                    queue.push_back((next, nd));
                }
            }
        }
    }
    // This could be sentinel value
    dists[goal]
}

fn bfs(adj: &[Vec<(usize, i32)>], source: usize, target: usize, k: i32, thr: i32) -> bool {
    let n = adj.len();
    let mut queue = VecDeque::from([(source, 0)]);
    let mut costs = vec![1 + k; n];
    costs[source] = 0;
    while let Some((node, cost)) = queue.pop_front() {
        if node == target {
            return cost <= k;
        }
        for &(next, w) in &adj[node] {
            let nc = cost + i32::from(w > thr);
            if costs[next] > nc {
                costs[next] = nc;
                if w > thr {
                    queue.push_back((next, nc));
                } else {
                    queue.push_front((next, nc));
                }
            }
        }
    }
    costs[target] <= k
}

fn dijkstra(adj: &[Vec<(usize, i32)>], source: usize, target: usize, k: i32, thr: i32) -> bool {
    let n = adj.len();
    let mut dists = vec![i32::MAX >> 1; n];
    dists[source] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), source)]);
    while let Some((Reverse(dist), node)) = heap.pop() {
        if node == target {
            return dist <= k;
        }
        if dist > dists[node] {
            continue;
        }
        for &(next, w) in adj[node].iter() {
            let nd = dist + i32::from(w > thr);
            if dists[next] > nd {
                dists[next] = nd;
                heap.push((Reverse(nd), next));
            }
        }
    }
    dists[target] <= k
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
        assert_eq!(
            minimum_threshold(
                6,
                &[[0, 1, 5], [1, 2, 3], [3, 4, 4], [4, 5, 1], [1, 4, 2]],
                0,
                3,
                1
            ),
            4
        );
    }

    #[test]
    fn test() {}
}
