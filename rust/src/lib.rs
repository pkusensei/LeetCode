mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn min_activation_cost(edges: &[[i32; 3]], values: &[i32]) -> i32 {
    let n = values.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, i64::from(e[2])));
        acc[b].push((a, i64::from(e[2])));
        acc
    });
    let v = dfs(&adj, &values, 0, n).1;
    i32::try_from(v).unwrap_or(-1)
}

fn dfs(
    adj: &[Vec<(usize, i64)>],
    values: &[i32],
    node: usize,
    prev: usize,
) -> (HashMap<i32, i64>, i64) {
    let mut subtree = HashMap::new();
    let mut min_cost = i64::MAX >> 1;
    for &(next, cost) in &adj[node] {
        if next != prev {
            let _v = dfs(adj, values, next, node);
            min_cost = min_cost.min(_v.1);
            for (mask, c) in _v.0.iter() {
                for (prev_mask, prev_cost) in subtree.iter() {
                    let nmask = prev_mask ^ mask;
                    if nmask.count_ones() & 1 == 0 {
                        min_cost = min_cost.min(prev_cost + c + cost);
                    }
                }
            }
            for (mask, c) in _v.0 {
                let ncost = cost + c;
                let nmask = mask ^ values[node];
                if nmask.count_ones() & 1 == 0 {
                    min_cost = min_cost.min(ncost);
                }
                let v = subtree.entry(nmask).or_insert(ncost);
                *v = (*v).min(ncost);
            }
        }
    }
    subtree.insert(values[node], 0);
    (subtree, min_cost)
}

//     0
//    1
//   2 3

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
            min_activation_cost(
                &[[0, 1, 4], [0, 2, 6], [0, 3, 3], [3, 4, 5]],
                &[3, 1, 7, 1, 3]
            ),
            7
        );
        assert_eq!(min_activation_cost(&[[0, 1, 5], [1, 2, 10]], &[1, 7, 3]), 5);
    }

    #[test]
    fn test() {
        assert_eq!(
            min_activation_cost(&[[1, 0, 6], [2, 1, 6], [3, 1, 4]], &[5, 7, 10, 10]),
            -1
        );
    }
}
