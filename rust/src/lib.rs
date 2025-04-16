mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_points(edges: &[[i32; 2]], coins: &[i32], k: i32) -> i32 {
    let n = coins.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    dfs(&adj, coins, k, 0, n, 0, &mut vec![[-1; 15]; n])
}

fn dfs(
    adj: &[Vec<usize>],
    coins: &[i32],
    k: i32,
    node: usize,
    prev: usize,
    halfed: usize,
    memo: &mut [[i32; 15]],
) -> i32 {
    if halfed > 14 {
        return 0;
    }
    if memo[node][halfed] > -1 {
        return memo[node][halfed];
    }
    let coin = coins[node] >> halfed;
    let mut no_half = coin - k;
    let mut half = coin >> 1;
    for &next in adj[node].iter() {
        if next != prev {
            no_half += dfs(adj, coins, k, next, node, halfed, memo);
            half += dfs(adj, coins, k, next, node, 1 + halfed, memo);
        }
    }
    memo[node][halfed] = no_half.max(half);
    memo[node][halfed]
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
            maximum_points(&[[0, 1], [1, 2], [2, 3]], &[10, 10, 3, 3], 5),
            11
        );
        assert_eq!(maximum_points(&[[0, 1], [0, 2]], &[8, 4, 4], 0), 16);
    }

    #[test]
    fn test() {}
}
