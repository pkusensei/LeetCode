mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn root_count(edges: &[[i32; 2]], guesses: &[[i32; 2]], k: i32) -> i32 {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let set: HashSet<_> = guesses
        .iter()
        .map(|g| [0, 1].map(|i| g[i] as usize))
        .collect();
    let curr = count(&adj, &set, 0, n);
    let mut res = 0;
    dfs(&adj, &set, k, 0, n, curr, &mut res);
    res
}

fn dfs(
    adj: &[Vec<usize>],
    set: &HashSet<[usize; 2]>,
    k: i32,
    node: usize,
    prev: usize,
    mut curr: i32,
    res: &mut i32,
) {
    if set.contains(&[prev, node]) {
        curr -= 1;
    }
    if set.contains(&[node, prev]) {
        curr += 1;
    }
    if curr >= k {
        *res += 1;
    }
    for &next in adj[node].iter() {
        if prev != next {
            dfs(adj, set, k, next, node, curr, res);
        }
    }
}

fn count(adj: &[Vec<usize>], set: &HashSet<[usize; 2]>, node: usize, prev: usize) -> i32 {
    let mut res = 0;
    for &next in adj[node].iter() {
        if prev != next {
            res += i32::from(set.contains(&[node, next]));
            res += count(adj, set, next, node);
        }
    }
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
        assert_eq!(
            root_count(
                &[[0, 1], [1, 2], [1, 3], [4, 2]],
                &[[1, 3], [0, 1], [1, 0], [2, 4]],
                3
            ),
            3
        );
        assert_eq!(
            root_count(
                &[[0, 1], [1, 2], [2, 3], [3, 4]],
                &[[1, 0], [3, 4], [2, 1], [3, 2]],
                1
            ),
            5
        );
    }

    #[test]
    fn test() {}
}
