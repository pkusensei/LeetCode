mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_flips(n: i32, edges: &[[i32; 2]], start: String, target: String) -> Vec<i32> {
    use itertools::{Itertools, izip};
    let n = n as usize;
    let adj = edges
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, e)| {
            let [a, b] = [0, 1].map(|i| e[i] as usize);
            acc[a].push((b, i));
            acc[b].push((a, i));
            acc
        });
    let mut flip = izip!(start.bytes(), target.bytes())
        .map(|(a, b)| a != b)
        .collect_vec();
    let mut res = vec![];
    if dfs(&adj, 0, n, n, &mut flip, &mut res) {
        res.sort_unstable();
        res
    } else {
        vec![-1]
    }
}

fn dfs(
    adj: &[Vec<(usize, usize)>],
    node: usize,
    prev: usize,
    edge_idx: usize,
    flip: &mut [bool],
    res: &mut Vec<i32>,
) -> bool {
    for &(next, ei) in &adj[node] {
        if next != prev && !dfs(adj, next, node, ei, flip, res) {
            return false;
        }
    }
    if flip[node] {
        flip[node] = false;
        if let Some(v) = flip.get_mut(prev) {
            *v = !(*v);
            res.push(edge_idx as i32);
        } else {
            return false;
        }
    }
    true
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
