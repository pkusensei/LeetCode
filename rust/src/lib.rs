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

pub fn min_time_max_power(
    n: i32,
    edges: Vec<Vec<i32>>,
    power: i32,
    cost: Vec<i32>,
    source: i32,
    target: i32,
) -> Vec<i64> {
    let [n, power, source, target] = [n, power, source, target].map(|v| v as usize);
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        acc[e[0] as usize].push((e[1] as usize, e[2]));
        acc
    });
    let mut heap = BinaryHeap::from([(Reverse(0), power, source)]);
    let mut seen = vec![vec![i64::MAX >> 1; 1 + power]; n];
    seen[source].fill(0);
    let mut res = [None, None];
    while let Some((Reverse(dist), power, node)) = heap.pop() {
        if node == target {
            let v = res[0].get_or_insert(dist);
            if dist <= *v {
                *v = dist;
                res[1] = res[1].max(Some(power as i64));
            }
            continue;
        }
        if dist > seen[node][power] || power < cost[node] as usize {
            continue;
        }
        for &(next, t) in &adj[node] {
            let ndist = i64::from(t) + dist;
            let np = power - cost[node] as usize;
            if ndist < seen[next][np] {
                seen[next][np] = ndist;
                heap.push((Reverse(ndist), np, next));
            }
        }
    }
    res.map(|v| v.unwrap_or(-1)).into()
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
