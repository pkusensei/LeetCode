mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let max = dfs(&adj, 0, n);
    mod_pow(2, max - 1) as i32
}

fn dfs(adj: &[Vec<usize>], node: usize, prev: usize) -> i32 {
    if adj[node] == [prev] {
        return 0;
    }
    let mut res = 0;
    for &next in &adj[node] {
        if next != prev {
            res = res.max(1 + dfs(adj, next, node))
        }
    }
    res
}

const fn mod_pow(b: i64, exp: i32) -> i64 {
    const M: i64 = 1_000_000_007;
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 1 {
        mod_pow(b * b % M, exp >> 1) * b % M
    } else {
        mod_pow(b * b % M, exp >> 1)
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
    fn basics() {}

    #[test]
    fn test() {}
}
