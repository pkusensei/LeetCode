mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
    let n = n as usize;
    let s = labels.as_bytes();
    let adj = edges.iter().fold(vec![HashMap::new(); n], |mut acc, e| {
        let v = acc[e[0] as usize].entry(e[1] as usize).or_insert(e[2]);
        *v = (*v).min(e[2]);
        acc
    });
    // (total weight, streak, node)
    let mut heap = BinaryHeap::from([(Reverse((0, 1)), 0)]);
    let mut seen = vec![vec![i32::MAX >> 2; 1 + k as usize]; n];
    let mut res = i32::MAX >> 2;
    while let Some((Reverse((dist, streak)), node)) = heap.pop() {
        if dist > seen[node][streak as usize] {
            continue;
        }
        if node == n - 1 {
            res = res.min(dist);
            continue;
        }
        for (&next, &w) in &adj[node] {
            let (nd, nstreak) = if s[next] != s[node] {
                (dist + w, 1)
            } else if 1 + streak <= k {
                (dist + w, 1 + streak)
            } else {
                continue;
            };
            if nd < seen[next][nstreak as usize] {
                seen[next][nstreak as usize] = nd;
                heap.push((Reverse((nd, nstreak)), next));
            }
        }
    }
    if res >= i32::MAX >> 2 { -1 } else { res }
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
    fn basics() {}

    #[test]
    fn test() {}
}
