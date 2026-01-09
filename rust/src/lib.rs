mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_distances_in_tree(n: usize, edges: &[[usize; 2]]) -> Vec<i32> {
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, &e| {
        let [a, b] = e;
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut res = vec![0; n];
    let mut subtree = vec![0; n];
    res[0] = dfs(&adj, 0, n, &mut subtree)[0];
    reroot(&adj, &subtree, &mut res, 0, n);
    res
}

fn reroot(adj: &[Vec<usize>], subtree: &[i32], res: &mut [i32], node: usize, prev: usize) {
    let n = adj.len();
    if prev < n {
        let other = n as i32 - subtree[node];
        let curr = res[prev] - subtree[node] + other;
        res[node] = curr;
    }
    for &next in &adj[node] {
        if next != prev {
            reroot(adj, subtree, res, next, node);
        }
    }
}

// [sum_dist, subtree_size]
fn dfs(adj: &[Vec<usize>], node: usize, prev: usize, subtree: &mut [i32]) -> [i32; 2] {
    let mut dist = 0;
    let mut sub = 1;
    for &next in &adj[node] {
        if next != prev {
            let v = dfs(adj, next, node, subtree);
            dist += v[0] + v[1];
            sub += v[1];
        }
    }
    subtree[node] = sub;
    [dist, sub]
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
            sum_of_distances_in_tree(6, &[[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]]),
            [8, 12, 6, 10, 10, 10]
        );
        assert_eq!(sum_of_distances_in_tree(2, &[[1, 0]]), [1, 1]);
    }

    #[test]
    fn test() {}
}
