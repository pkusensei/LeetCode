mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
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
        acc[e[0] as usize - 1].push(e[1] as usize - 1);
        acc
    });
    let [a, _] = dfs(&adj, &present, &future, budget as usize, 0, n);
    a.into_iter().max().unwrap_or_default()
}

fn dfs(
    adj: &[Vec<usize>],
    present: &[i32],
    future: &[i32],
    budget: usize,
    node: usize,
    prev: usize,
) -> [Vec<i32>; 2] {
    let mut full = vec![0; 1 + budget];
    let mut half = vec![0; 1 + budget];
    for &next in &adj[node] {
        if next != prev {
            let [a, b] = dfs(adj, present, future, budget, next, node);
            full = merge(&full, &a);
            half = merge(&half, &b);
        }
    }
    let mut res_full = full.clone();
    let mut res_half = full.clone();
    let mut cost = present[node] as usize;
    for b in cost..=budget {
        res_full[b] = res_full[b].max(half[b - cost] + future[node] - cost as i32);
    }
    cost /= 2;
    for b in cost..=budget {
        res_half[b] = res_half[b].max(half[b - cost] + future[node] - cost as i32);
    }
    [res_full, res_half]
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut res = vec![0; n];
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
            max_profit(3, &[4, 6, 8], &[7, 9, 11], &[[1, 2], [1, 3]], 10),
            10
        );
        assert_eq!(
            max_profit(3, &[5, 2, 3], &[8, 5, 6], &[[1, 2], [2, 3]], 7),
            12
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            max_profit(3, &[42, 27, 32], &[46, 8, 17], &[[1, 2], [2, 3]], 93),
            4
        );
    }
}
