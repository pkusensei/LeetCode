mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::Itertools;
use std::collections::HashSet;

pub fn supersequences(words: &[&str]) -> Vec<Vec<i32>> {
    let mut nodes = HashSet::new();
    let mut edges = vec![];
    for s in words.iter().map(|s| s.as_bytes()) {
        let [a, b] = [0, 1].map(|i| usize::from(s[i] - b'a'));
        nodes.extend([a, b]);
        edges.push([a, b]);
    }
    let mut res = vec![];
    for d in find_doubles(&edges, &nodes) {
        let mut curr = vec![0; 26];
        for &i in &nodes {
            curr[i] = 1;
        }
        for i in d {
            curr[i] = 2;
        }
        res.push(curr);
    }
    res
}

fn find_doubles(edges: &[[usize; 2]], nodes: &HashSet<usize>) -> Vec<Vec<usize>> {
    let n = nodes.len();
    let mut res = vec![];
    for len in 0..=n {
        for doubles in nodes.iter().copied().combinations(len) {
            let mut adj = [const { vec![] }; 26];
            for &[a, b] in edges {
                if !doubles.contains(&a) && !doubles.contains(&b) {
                    adj[a].push(b);
                }
            }
            if !has_cycle(&adj) {
                res.push(doubles);
            }
        }
        if !res.is_empty() {
            break;
        }
    }
    res
}

fn has_cycle(adj: &[Vec<usize>; 26]) -> bool {
    let mut colors = [Color::New; 26];
    (0..26).any(|node| dfs(adj, node, &mut colors))
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    New,
    Curr,
    Done,
}

fn dfs(adj: &[Vec<usize>; 26], node: usize, colors: &mut [Color; 26]) -> bool {
    if colors[node] != Color::New {
        return colors[node] == Color::Curr;
    }
    colors[node] = Color::Curr;
    for &next in &adj[node] {
        if dfs(adj, next, colors) {
            return true;
        }
    }
    colors[node] = Color::Done;
    false
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
            supersequences(&["ab", "ba"]),
            [
                [
                    2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
                [
                    1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ],
            ]
        );
        assert_eq!(
            supersequences(&["aa", "ac"]),
            [[
                2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]]
        );
        assert_eq!(
            supersequences(&["aa", "bb", "cc"]),
            [[
                2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]]
        );
    }

    #[test]
    fn test() {}
}
