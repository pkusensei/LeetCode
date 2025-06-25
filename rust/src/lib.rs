mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit(
    n: i32,
    present: &[i32],
    future: &[i32],
    hierarchy: &[[i32; 2]],
    budget: i32,
) -> i32 {
    let n = n as usize;
    let adj = hierarchy.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a].push(b);
        acc
    });
    *dfs(&adj, present, future, budget as usize, 0, n)[0]
        .iter()
        .max()
        .unwrap()
}

fn dfs(
    adj: &[Vec<usize>],
    present: &[i32],
    future: &[i32],
    budget: usize,
    node: usize,
    prev: usize,
) -> [Vec<i32>; 2] {
    let mut dp0 = vec![0; 1 + budget];
    let mut dp1 = vec![0; 1 + budget];
    for &next in &adj[node] {
        if next != prev {
            let [res0, res1] = dfs(adj, present, future, budget, next, node);
            dp0 = merge(&dp0, &res0);
            dp1 = merge(&dp1, &res1);
        }
    }
    let mut res0 = dp0.clone();
    let mut res1 = dp0.clone();
    let mut cost = present[node] as usize;
    for b in cost..=budget {
        res0[b] = res0[b].max(dp1[b - cost] + future[node] - cost as i32);
    }
    cost >>= 1;
    for b in cost..=budget {
        res1[b] = res1[b].max(dp1[b - cost] + future[node] - cost as i32);
    }
    [res0, res1]
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut res = vec![i32::MIN; n];
    for (i1, v1) in a.iter().enumerate() {
        for (i2, v2) in b[..n - i1].iter().enumerate() {
            res[i1 + i2] = res[i1 + i2].max(v1 + v2);
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
        assert_eq!(max_profit(2, &[1, 2], &[4, 3], &[[1, 2]], 3), 5);
        assert_eq!(max_profit(2, &[3, 4], &[5, 8], &[[1, 2]], 4), 4);
        assert_eq!(
            max_profit(3, &[5, 2, 3], &[8, 5, 6], &[[1, 2], [2, 3]], 7),
            12
        );
        assert_eq!(
            max_profit(3, &[4, 6, 8], &[7, 9, 11], &[[1, 2], [1, 3]], 10),
            10
        );
    }

    #[test]
    fn test() {}
}
