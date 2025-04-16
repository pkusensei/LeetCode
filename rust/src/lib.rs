mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score_after_operations(edges: &[[i32; 2]], values: &[i32]) -> i64 {
    let n = values.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let sum: i64 = values.iter().map(|&v| i64::from(v)).sum();
    sum - dfs(&adj, values, 0, n, &mut vec![-1; n])
}

fn dfs(adj: &[Vec<usize>], values: &[i32], node: usize, prev: usize, memo: &mut [i64]) -> i64 {
    if memo[node] > -1 {
        return memo[node];
    }
    let mut leaves = 0;
    for &next in adj[node].iter() {
        if next != prev {
            leaves += dfs(adj, values, next, node, memo);
        }
    }
    let curr = i64::from(values[node]);
    memo[node] = if leaves == 0 { curr } else { curr.min(leaves) };
    memo[node]
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
            maximum_score_after_operations(
                &[[0, 1], [0, 2], [0, 3], [2, 4], [4, 5]],
                &[5, 2, 5, 2, 1, 1]
            ),
            11
        );
        assert_eq!(
            maximum_score_after_operations(
                &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                &[20, 10, 9, 7, 4, 3, 5],
            ),
            40
        );
    }

    #[test]
    fn test() {}
}
