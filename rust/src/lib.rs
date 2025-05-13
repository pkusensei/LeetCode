mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn time_taken(edges: &[[i32; 2]]) -> Vec<i32> {
    let n = 1 + edges.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    // max and second max to visit substree
    let mut subtree = vec![[0, 0]; n];
    mark(&adj, 0, n, &mut subtree);
    let mut dp = vec![[0, 0]; n];
    dp[0] = subtree[0];
    for &next in &adj[0] {
        dfs(&adj, next, 0, &subtree, &mut dp);
    }
    dp.iter().map(|v| v[0]).collect()
}

fn dfs(adj: &[Vec<usize>], node: usize, prev: usize, subtree: &[[i32; 2]], dp: &mut [[i32; 2]]) {
    let [prev_max1, prev_max2] = dp[prev];
    let [curr_max1, curr_max2] = subtree[node];
    let prev_time = if curr_max1 + delta(node) == prev_max1 {
        // If curr contributed to subtree at prev,
        // curr must be excluded to avoid double counting
        prev_max2
    } else {
        prev_max1
    };
    let prev_rerooted = prev_time + delta(prev);
    if curr_max1 >= prev_rerooted {
        dp[node][0] = curr_max1;
        dp[node][1] = curr_max2.max(prev_rerooted);
    } else {
        dp[node] = [prev_rerooted, curr_max1];
    }
    for &next in &adj[node] {
        if next != prev {
            dfs(adj, next, node, subtree, dp);
        }
    }
}

#[inline]
const fn delta(node: usize) -> i32 {
    if node & 1 == 1 { 1 } else { 2 }
}

fn mark(adj: &[Vec<usize>], node: usize, prev: usize, subtree: &mut [[i32; 2]]) -> i32 {
    let [mut max1, mut max2] = [0, 0];
    for &next in &adj[node] {
        if next != prev {
            let curr = mark(adj, next, node, subtree) + delta(next);
            if curr >= max1 {
                max2 = max1;
                max1 = curr;
            } else if curr > max2 {
                max2 = curr;
            }
        }
    }
    subtree[node] = [max1, max2];
    max1
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
        assert_eq!(time_taken(&[[0, 1], [0, 2]]), [2, 4, 3]);
        assert_eq!(time_taken(&[[0, 1]]), [1, 2]);
        assert_eq!(
            time_taken(&[[2, 4], [0, 1], [2, 3], [0, 2]]),
            [4, 6, 3, 5, 5]
        );
    }

    #[test]
    fn test() {}
}
