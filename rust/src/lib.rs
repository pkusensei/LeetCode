mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn remaining_methods(n: i32, k: i32, invocations: &[[i32; 2]]) -> Vec<i32> {
    let [n, k] = [n, k].map(|v| v as usize);
    let [mut forward, mut backward] = [0, 1].map(|_| vec![vec![]; n]);
    for e in invocations.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        forward[a].push(b);
        backward[b].push(a);
    }
    let mut mark = bfs(&forward, k);
    let mut seen = vec![false; n];
    for i in 0..n {
        if mark[i] && !dfs(&backward, i, &mut mark, &mut seen) {
            mark.fill(false);
            break;
        }
    }
    mark.iter()
        .enumerate()
        .filter_map(|(i, &v)| if !v { Some(i as i32) } else { None })
        .collect()
}

fn bfs(adj: &[Vec<usize>], start: usize) -> Vec<bool> {
    use std::collections::VecDeque;
    let n = adj.len();
    let mut queue = VecDeque::from([start]);
    let mut mark = vec![false; n];
    mark[start] = true;
    while let Some(node) = queue.pop_front() {
        for &next in &adj[node] {
            if !mark[next] {
                mark[next] = true;
                queue.push_back(next);
            }
        }
    }
    mark
}

fn dfs(adj: &[Vec<usize>], node: usize, mark: &mut [bool], seen: &mut [bool]) -> bool {
    if seen[node] || !mark[node] {
        return mark[node];
    }
    seen[node] = true;
    for &next in &adj[node] {
        if !seen[next] && !dfs(adj, next, mark, seen) {
            mark[node] = false;
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
        assert_eq!(remaining_methods(3, 2, &[[1, 2], [0, 1], [2, 0]]), []);
        assert_eq!(
            remaining_methods(4, 1, &[[1, 2], [0, 1], [3, 2]]),
            [0, 1, 2, 3]
        );
        assert_eq!(
            remaining_methods(5, 0, &[[1, 2], [0, 2], [0, 1], [3, 4]]),
            [3, 4]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            remaining_methods(4, 1, &[[2, 3], [2, 0], [3, 2], [0, 1], [1, 2]]),
            []
        );
        assert_eq!(remaining_methods(3, 2, &[[1, 0], [2, 0]]), [0, 1, 2]);
    }
}
