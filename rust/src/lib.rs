mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::iter::once;

#[allow(unused_imports)]
use helper::*;

pub fn find_answer(parent: &[i32], s: &str) -> Vec<bool> {
    let n = parent.len();
    let adj = parent
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &v)| {
            if i > 0 {
                acc[v as usize].push(i);
            }
            acc
        });
    let mut traversed = vec![b'#'; 1 + 2 * n];
    let mut subs = vec![[0, 0]; n];
    dfs(&adj, s.as_bytes(), &mut traversed, 0, &mut 0, &mut subs);
    let d = manacher_odd(traversed);
    let mut res = Vec::with_capacity(n);
    for [left, right] in subs {
        let len = right + 1 - left;
        res.push(d[left + right + 1] - 1 >= len);
    }
    res
}

fn dfs(
    adj: &[Vec<usize>],
    s: &[u8],
    traversed: &mut [u8],
    node: usize,
    count: &mut usize,
    subs: &mut [[usize; 2]],
) {
    let left = *count;
    for &next in &adj[node] {
        dfs(adj, s, traversed, next, count, subs);
    }
    traversed[1 + 2 * (*count)] = s[node];
    subs[node] = [left, *count];
    *count += 1;
}

fn manacher_odd(s: Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let s: Vec<u8> = once(b'$').chain(s).chain(once(b'^')).collect();
    let mut p = vec![0; 2 + n];
    let [mut left, mut right] = [1_usize, 1];
    for idx in 1..=n {
        p[idx] = right.saturating_sub(idx).min(p[left + right - idx]);
        while s[idx - p[idx]] == s[idx + p[idx]] {
            p[idx] += 1;
        }
        if idx + p[idx] > right {
            left = idx - p[idx];
            right = idx + p[idx];
        }
    }
    p[1..n].to_vec()
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
            find_answer(&[-1, 0, 0, 1, 1, 2], "aababa"),
            [true, true, false, true, true, true]
        );
    }

    #[test]
    fn test() {}
}
