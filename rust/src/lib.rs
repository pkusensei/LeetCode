mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_sum_of_weights(edges: &[[i32; 3]], k: i32) -> i64 {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push((b, i64::from(e[2])));
        acc[b].push((a, i64::from(e[2])));
        acc
    });
    dfs(&adj, k as usize, 0, n)[1]
}

// sum with [at most k-1 children, at most k]
fn dfs(adj: &[Vec<(usize, i64)>], k: usize, node: usize, prev: usize) -> [i64; 2] {
    let mut res = 0;
    let mut diff = vec![];
    for &(next, w) in &adj[node] {
        if next == prev {
            continue;
        }
        let [a, b] = dfs(adj, k, next, node);
        res += b; // Assume subtree with k children -- no connection
        // For each child, what if current `w` is picked?
        // i.e try find a "upgrade" with this edge
        diff.push((a + w - b).max(0));
    }
    if diff.len() >= k {
        diff.select_nth_unstable_by(k - 1, |a, b| b.cmp(a)); // big first
    }
    res += diff.iter().take(k - 1).sum::<i64>();
    let res2 = res + diff.get(k - 1).unwrap_or(&0);
    [res, res2]
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
            maximize_sum_of_weights(&[[0, 1, 4], [0, 2, 2], [2, 3, 12], [2, 4, 6]], 2),
            22
        );
        assert_eq!(
            maximize_sum_of_weights(
                &[
                    [0, 1, 5],
                    [1, 2, 10],
                    [0, 3, 15],
                    [3, 4, 20],
                    [3, 5, 5],
                    [0, 6, 10]
                ],
                3
            ),
            65
        );
    }

    #[test]
    fn test() {}
}
