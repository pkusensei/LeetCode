mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_subgraph_score(n: i32, edges: &[[i32; 2]], good: &[i32]) -> Vec<i32> {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut res = vec![0; n];
    res[0] = dfs(&adj, &good, 0, n, &mut res);
    reroot(&adj, 0, n, &mut res);
    res
}

fn reroot(adj: &[Vec<usize>], node: usize, prev: usize, res: &mut [i32]) {
    for &next in &adj[node] {
        if next != prev {
            let curr = res[next].max(0);
            let prev_score = res[node] - curr;
            res[next] += prev_score.max(0);
            reroot(adj, next, node, res);
        }
    }
}

fn dfs(adj: &[Vec<usize>], good: &[i32], node: usize, prev: usize, res: &mut [i32]) -> i32 {
    let mut score = if good[node] > 0 { 1 } else { -1 };
    for &next in &adj[node] {
        if next != prev {
            let subtree = dfs(adj, good, next, node, res);
            if subtree > 0 {
                score += subtree;
            }
        }
    }
    res[node] = score;
    score
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
            max_subgraph_score(3, &[[0, 1], [1, 2]], &[1, 0, 1]),
            [1, 1, 1]
        );
        assert_eq!(
            max_subgraph_score(5, &[[1, 0], [1, 2], [1, 3], [3, 4]], &[0, 1, 0, 1, 1]),
            [2, 3, 2, 3, 3]
        );
        assert_eq!(max_subgraph_score(2, &[[0, 1]], &[0, 0]), [-1, -1]);
    }

    #[test]
    fn test() {}
}
