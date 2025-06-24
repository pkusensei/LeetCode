mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn assign_edge_weights(edges: &[[i32; 2]]) -> i32 {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let depth = dfs(&adj, 0, n);
    mod_pow(2, depth - 2) as i32
}

const M: i64 = 1_000_000_007;

fn dfs(adj: &[Vec<usize>], node: usize, prev: usize) -> i64 {
    let mut res = 0;
    for &next in &adj[node] {
        if next != prev {
            res = res.max(dfs(adj, next, node))
        }
    }
    1 + res
}

const fn mod_pow(b: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % M, exp >> 1)
    } else {
        mod_pow(b * b % M, exp >> 1) * b % M
    }
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
        assert_eq!(assign_edge_weights(&[[1, 2], [1, 3], [3, 4], [3, 5]]), 2);
    }

    #[test]
    fn test() {}
}
