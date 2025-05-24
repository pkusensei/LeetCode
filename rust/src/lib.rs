mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_subtree_sizes(mut parent: Vec<i32>, s: &str) -> Vec<i32> {
    let n = parent.len();
    let mut adj = build(&parent);
    dfs1(&adj, s.as_bytes(), 0, &mut [None; 26], &mut parent);
    adj = build(&parent);
    let mut res = vec![0; n];
    dfs2(&adj, 0, &mut res);
    res
}

fn build(parent: &[i32]) -> Vec<Vec<usize>> {
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
    adj
}

fn dfs1(
    adj: &[Vec<usize>],
    s: &[u8],
    node: usize,
    last: &mut [Option<i32>; 26],
    parent: &mut [i32],
) {
    let idx = usize::from(s[node] - b'a');
    let old = last[idx];
    if let Some(v) = last[idx] {
        parent[node] = v
    }
    last[idx] = Some(node as i32);
    for &next in &adj[node] {
        dfs1(adj, s, next, last, parent);
    }
    last[idx] = old;
}

fn dfs2(adj: &[Vec<usize>], node: usize, res: &mut [i32]) -> i32 {
    let mut sz = 1;
    for &next in &adj[node] {
        sz += dfs2(adj, next, res);
    }
    res[node] = sz;
    sz
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
            find_subtree_sizes(vec![-1, 0, 4, 0, 1], "abbba"),
            [5, 2, 1, 1, 1]
        );
    }

    #[test]
    fn test() {}
}
