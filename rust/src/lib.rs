mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn find_max_path_score(edges: &[[i32; 3]], online: &[bool], k: i64) -> i32 {
    let n = online.len();
    let mut adj = vec![vec![]; n];
    let mut max_e = 0;
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        if online[a] && online[b] {
            adj[a].push((b, e[2]));
            max_e = max_e.max(e[2]);
        }
    }
    let mut left = 0;
    let mut right = max_e + 1;
    let mut reached = false;
    while left < right {
        let mid = left + (right - left + 1) / 2;
        if check(&adj, k, mid) {
            left = mid;
            reached = true;
        } else {
            right = mid - 1;
        }
    }
    if reached { left } else { -1 }
}

fn check(adj: &[Vec<(usize, i32)>], k: i64, mid: i32) -> bool {
    let n = adj.len();
    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    let mut dists = vec![1 + k; n];
    dists[0] = 0;
    while let Some((Reverse(total), node)) = heap.pop() {
        if node == n - 1 && total <= k {
            return true;
        }
        if total > dists[node] {
            continue;
        }
        for &(next, cost) in &adj[node] {
            if cost < mid {
                continue;
            }
            let ntotal = total + i64::from(cost);
            if ntotal < dists[next] {
                dists[next] = ntotal;
                heap.push((Reverse(ntotal), next));
            }
        }
    }
    false
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
            find_max_path_score(
                &[[0, 1, 5], [1, 3, 10], [0, 2, 3], [2, 3, 4]],
                &[true; 4],
                10
            ),
            3
        );
        assert_eq!(
            find_max_path_score(
                &[
                    [0, 1, 7],
                    [1, 4, 5],
                    [0, 2, 6],
                    [2, 3, 6],
                    [3, 4, 2],
                    [2, 4, 6]
                ],
                &[true, true, true, false, true],
                12
            ),
            6
        );
    }

    #[test]
    fn test() {
        assert_eq!(find_max_path_score(&[[0, 1, 8]], &[true, true], 11), 8);
    }
}
