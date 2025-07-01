mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::BTreeSet;

#[allow(unused_imports)]
use helper::*;

pub fn kth_smallest(par: &[i32], vals: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = par.len();
    let adj = par
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &v)| {
            if v >= 0 {
                acc[v as usize].push(i);
            }
            acc
        });
    let mut qs = vec![vec![]; n];
    for (i, q) in queries.iter().enumerate() {
        let [node, k] = [0, 1].map(|i| q[i] as usize);
        qs[node].push((k - 1, i));
    }
    let mut res = vec![0; queries.len()];
    dfs(&adj, &vals, 0, 0, &qs, &mut res);
    res
}

fn dfs(
    adj: &[Vec<usize>],
    vals: &[i32],
    node: usize,
    mut path: i32,
    qmap: &[Vec<(usize, usize)>],
    res: &mut [i32],
) -> BTreeSet<i32> {
    path ^= vals[node];
    let mut set = BTreeSet::from([path]);
    for &next in &adj[node] {
        let mut nset = dfs(adj, vals, next, path, qmap, res);
        if nset.len() > set.len() {
            nset.extend(set);
            set = nset
        } else {
            set.extend(nset);
        }
    }
    for &(k, i) in &qmap[node] {
        res[i] = *set.iter().nth(k).unwrap_or(&-1);
    }
    set
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
            kth_smallest(&[-1, 0, 0], &[1, 1, 1], &[[0, 1], [0, 2], [0, 3]]),
            [0, 1, -1]
        );
        assert_eq!(
            kth_smallest(&[-1, 0, 1], &[5, 2, 7], &[[0, 1], [1, 2], [1, 3], [2, 1]]),
            [0, 7, -1, 0]
        );
    }

    #[test]
    fn test() {}
}
