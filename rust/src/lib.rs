mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_increase(n: i32, edges: &[[i32; 2]], cost: &[i32]) -> i32 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut max_until = vec![0; n];
    let max = dfs_max(&adj, cost, 0, n, &mut max_until);
    dfs_inc(&adj, cost, &max_until, 0, n, max, &mut vec![0; n])
}

fn dfs_max(
    adj: &[Vec<usize>],
    cost: &[i32],
    node: usize,
    prev: usize,
    max_until: &mut [i64],
) -> i64 {
    let mut res = 0;
    for &next in &adj[node] {
        if next != prev {
            res = res.max(dfs_max(adj, cost, next, node, max_until))
        }
    }
    res += i64::from(cost[node]);
    max_until[node] = res;
    res
}

fn dfs_inc(
    adj: &[Vec<usize>],
    cost: &[i32],
    max_until: &[i64],
    node: usize,
    prev: usize,
    target: i64,
    inc: &mut [i64],
) -> i32 {
    inc[node] = target - max_until[node];
    let ntarget = target - i64::from(cost[node]);
    let mut res = 0;
    for &next in &adj[node] {
        if next != prev {
            res += dfs_inc(adj, cost, max_until, next, node, ntarget, inc);
            res += i32::from(inc[node] != inc[next]);
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
        assert_eq!(min_increase(3, &[[0, 1], [0, 2]], &[2, 1, 3]), 1);
        assert_eq!(min_increase(3, &[[0, 1], [1, 2]], &[5, 1, 4]), 0);
        assert_eq!(
            min_increase(5, &[[0, 4], [0, 1], [1, 2], [1, 3]], &[3, 4, 1, 1, 7]),
            1
        )
    }

    #[test]
    fn test() {}
}
