mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn placed_coins(edges: &[[i32; 2]], cost: &[i32]) -> Vec<i64> {
    let n = cost.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut res = vec![0; n];
    dfs(&adj, cost, 0, n, &mut res);
    res
}

// (pos_heap min_heap, neg_heap)
fn dfs(
    adj: &[Vec<usize>],
    cost: &[i32],
    node: usize,
    prev: usize,
    res: &mut [i64],
) -> (BinaryHeap<Reverse<i64>>, BinaryHeap<i64>) {
    let val = i64::from(cost[node]);
    let mut pos = BinaryHeap::new();
    let mut neg = BinaryHeap::new();
    if val > 0 {
        pos.push(Reverse(val));
    } else {
        neg.push(val);
    }
    for &next in adj[node].iter() {
        if next != prev {
            let (npos, nneg) = dfs(adj, cost, next, node, res);
            pos.extend(npos);
            neg.extend(nneg);
        }
    }
    while pos.len() > 3 {
        pos.pop();
    }
    while neg.len() > 3 {
        neg.pop();
    }
    let nums = pos
        .iter()
        .map(|v| v.0)
        .chain(neg.iter().copied())
        .sorted_unstable()
        .collect_vec();
    if nums.len() < 3 {
        res[node] = 1;
    } else {
        let a = nums[0] * nums[1] * nums.last().unwrap_or(&1);
        let b: i64 = nums.iter().tail(3).product();
        res[node] = a.max(b).max(0);
    }
    (pos, neg)
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
            placed_coins(
                &[[0, 1], [0, 2], [0, 3], [0, 4], [0, 5]],
                &[1, 2, 3, 4, 5, 6]
            ),
            [120, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            placed_coins(
                &[
                    [0, 1],
                    [0, 2],
                    [1, 3],
                    [1, 4],
                    [1, 5],
                    [2, 6],
                    [2, 7],
                    [2, 8]
                ],
                &[1, 4, 2, 3, 5, 7, 8, -4, 2]
            ),
            [280, 140, 32, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(placed_coins(&[[0, 1], [0, 2]], &[1, 2, -2]), [0, 1, 1]);
    }

    #[test]
    fn test() {}
}
