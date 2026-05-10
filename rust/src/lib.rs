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

pub fn min_cost(n: i32, prices: Vec<i32>, roads: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let adj = roads.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, i64::from(e[2]), i64::from(e[2]) * i64::from(e[3])));
        acc[b].push((a, i64::from(e[2]), i64::from(e[2]) * i64::from(e[3])));
        acc
    });
    let mut res = Vec::with_capacity(n);
    for start in 0..n {
        let empty = f(&adj, start, true);
        let full = f(&adj, start, false);
        let mut val = i64::from(prices[start]);
        for goal in (0..n).filter(|&v| v != start) {
            let curr = empty[goal] + full[goal] + i64::from(prices[goal]);
            val = val.min(curr);
        }
        res.push(val as i32);
    }
    res
}

fn f(adj: &[Vec<(usize, i64, i64)>], start: usize, empty: bool) -> Vec<i64> {
    let n = adj.len();
    let mut heap = BinaryHeap::from([(Reverse(0), start)]);
    let mut costs = vec![i64::MAX >> 2; n];
    costs[start] = 0;
    while let Some((Reverse(cost), node)) = heap.pop() {
        if cost > costs[node] {
            continue;
        }
        for &(next, c, taxi) in &adj[node] {
            let nc = cost + if empty { c } else { taxi };
            if nc < costs[next] {
                costs[next] = nc;
                heap.push((Reverse(nc), next));
            }
        }
    }
    costs
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
