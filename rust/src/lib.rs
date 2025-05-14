mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let n = 1 + edges.len();
        let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
            let [a, b] = [0, 1].map(|i| e[i] as usize);
            acc[a].push(b);
            acc[b].push(a);
            acc
        });
        let mut res = 0;
        dfs(&adj, 0, n, &mut res);
        res
}

fn dfs(adj: &[Vec<usize>], node: usize, prev: usize, res: &mut i32) -> i32 {
    let mut subs = vec![];
    for &next in &adj[node] {
        if next != prev {
            subs.push(dfs(adj, next, node, res));
        }
    }
    if subs.iter().all_equal() {
        *res += 1
    }
    1 + subs.iter().sum::<i32>()
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
    fn basics() {}

    #[test]
    fn test() {}
}
