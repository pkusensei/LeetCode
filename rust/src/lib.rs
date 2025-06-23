mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_weight(n: i32, edges: &[[i32; 3]], k: i32, t: i32) -> i32 {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    let mut indegs = vec![0; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push((b, e[2]));
        indegs[b] += 1;
    }
    let mut res = -1;
    let mut path = vec![0];
    let mut memo = HashMap::new();
    for (node, &d) in indegs.iter().enumerate() {
        if d == 0 {
            res = res.max(dfs(&adj, k as usize, t, node, &mut path, &mut memo))
        }
    }
    res
}

fn dfs(
    adj: &[Vec<(usize, i32)>],
    k: usize,
    t: i32,
    node: usize,
    path: &mut Vec<i32>,
    memo: &mut HashMap<(usize, usize, i32), i32>,
) -> i32 {
    let dist = *path.last().unwrap();
    let n = path.len(); // depth
    if let Some(&v) = memo.get(&(node, n, dist)) {
        return v;
    }
    let mut res = -1;
    if n > k && (path[n - 1] - path[n - k - 1]) < t {
        res = path[n - 1] - path[n - k - 1];
    }
    for &(next, w) in &adj[node] {
        path.push(w + path.last().unwrap_or(&0));
        res = res.max(dfs(adj, k, t, next, path, memo));
        path.pop();
    }
    memo.insert((node, n, dist), res);
    res
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
        assert_eq!(max_weight(3, &[[0, 1, 1], [1, 2, 2]], 2, 4), 3);
        assert_eq!(max_weight(3, &[[0, 1, 2], [0, 2, 3]], 1, 3), 2);
        assert_eq!(max_weight(3, &[[0, 1, 6], [1, 2, 8]], 1, 6), -1);
    }

    #[test]
    fn test() {}
}
