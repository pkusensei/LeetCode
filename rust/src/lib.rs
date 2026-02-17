mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_lca(adj: &[Vec<usize>], a: usize, b: usize) -> usize {
    let (euler, first, depth) = euler_tour(adj);
    let st = build(&euler, &depth);
    let [a, b] = [a, b].map(|v| first[v]);
    let idx = rmq(&st, &euler, &depth, a.min(b), a.max(b));
    euler[idx]
}

fn rmq(st: &[Vec<usize>], euler: &[usize], depth: &[i32], left: usize, right: usize) -> usize {
    let len = 1 + right - left;
    let k = len.ilog2() as usize;
    let left = st[k][left];
    let right = st[k][right + 1 - (1 << k)];
    if depth[euler[left]] < depth[euler[right]] {
        left
    } else {
        right
    }
}

fn build(euler: &[usize], depth: &[i32]) -> Vec<Vec<usize>> {
    let n = euler.len();
    let log = 1 + n.ilog2() as usize;
    let mut st = vec![vec![0; n]; log];
    st[0] = (0..n).collect();
    for k in 1..log {
        let len = 1 << k;
        for i in 0..=n - len {
            let left = st[k - 1][i];
            let right = st[k - 1][i + (len >> 1)];
            st[k][i] = if depth[euler[left]] < depth[euler[right]] {
                left
            } else {
                right
            }
        }
    }
    st
}

pub fn euler_tour(adj: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>, Vec<i32>) {
    let n = adj.len();
    let mut euler = Vec::with_capacity(2 * n - 1);
    let mut first = vec![0; n];
    let mut depth = vec![0; n];
    dfs(&adj, 0, n, &mut euler, &mut first, &mut depth);
    (euler, first, depth)
}

fn dfs(
    adj: &[Vec<usize>],
    node: usize,
    prev: usize,
    euler: &mut Vec<usize>,
    first: &mut [usize],
    depth: &mut Vec<i32>,
) {
    first[node] = euler.len();
    euler.push(node);
    for &next in &adj[node] {
        if next != prev {
            depth[next] = 1 + depth[node];
            dfs(adj, next, node, euler, first, depth);
            euler.push(node);
        }
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
        let e = [
            [0, 1],
            [0, 2],
            [1, 3],
            [1, 4],
            [3, 5],
            [3, 6],
            [2, 7],
            [2, 8],
        ];
        let adj = e.iter().fold(vec![vec![]; 9], |mut acc, e| {
            acc[e[0]].push(e[1]);
            acc[e[1]].push(e[0]);
            acc
        });
        assert_eq!(find_lca(&adj, 4, 5), 1);
    }

    #[test]
    fn test() {}
}
