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

pub fn interaction_costs(n: i32, edges: &[[i32; 2]], group: &[i32]) -> i64 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let group_freq = group.iter().fold(HashMap::new(), |mut acc, &g| {
        *acc.entry(g).or_insert(0) += 1;
        acc
    });
    dfs(&adj, &group, &group_freq, 0, n).0
}

// group-freq
fn dfs(
    adj: &[Vec<usize>],
    group: &[i32],
    gf: &HashMap<i32, i64>,
    node: usize,
    prev: usize,
) -> (i64, HashMap<i32, i64>) {
    let mut freq = HashMap::from([(group[node], 1)]);
    let mut res = 0;
    for &next in &adj[node] {
        if next != prev {
            let subtree = dfs(adj, group, gf, next, node);
            res += subtree.0;
            for (k, v) in subtree.1 {
                res += v * (gf[&k] - v);
                *freq.entry(k).or_insert(0) += v;
            }
        }
    }
    (res, freq)
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
        assert_eq!(interaction_costs(3, &[[0, 1], [1, 2]], &[1, 1, 1]), 4);
        assert_eq!(interaction_costs(3, &[[0, 1], [1, 2]], &[3, 2, 3]), 2);
        assert_eq!(
            interaction_costs(4, &[[0, 1], [0, 2], [0, 3]], &[1, 1, 4, 4]),
            3
        );
        assert_eq!(interaction_costs(2, &[[0, 1]], &[9, 8]), 0);
    }

    #[test]
    fn test() {}
}
