mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

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
        if check(&adj, source as usize, target as usize, k, mid) {
            possible = true;
            right = mid
        } else {
            left = 1 + mid
        }
    }
    if possible { left } else { -1 }
}

fn check(adj: &[Vec<(usize, i32)>], source: usize, target: usize, k: i32, thr: i32) -> bool {
    let n = adj.len();
    let mut dists = vec![i32::MAX >> 1; n];
    dists[source] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), source)]);
    while let Some((Reverse(dist), node)) = heap.pop() {
        if node == target {
            return dists[node] <= k;
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
    fn test() {
        assert_eq!(
            minimum_threshold(
                10,
                &[
                    [0, 5, 37],
                    [4, 6, 80],
                    [5, 8, 42],
                    [0, 1, 57],
                    [4, 8, 73],
                    [0, 3, 20],
                    [0, 6, 77],
                    [2, 9, 50],
                    [1, 9, 30]
                ],
                4,
                1,
                3
            ),
            0
        );
        assert_eq!(minimum_threshold(2, &[], 1, 0, 0), -1);
        assert_eq!(minimum_threshold(2, &[], 0, 1, 0), -1);
    }
}
