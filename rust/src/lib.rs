mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn max_score(grid: &[&[i32]]) -> i32 {
    let rows = grid.len();
    let mut map = HashMap::<_, HashSet<_>>::new();
    for (num, r) in grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().map(move |&v| (v, r)))
    {
        map.entry(num).or_default().insert(r);
    }
    let arr = map.into_iter().collect_vec();
    dfs(&arr, 0, 0, &mut vec![vec![-1; 1 << rows]; arr.len()])
}

fn dfs(arr: &[(i32, HashSet<usize>)], idx: usize, mask: usize, memo: &mut [Vec<i32>]) -> i32 {
    if idx >= arr.len() {
        return 0;
    }
    if memo[idx][mask] > -1 {
        return memo[idx][mask];
    }
    let mut res = dfs(arr, 1 + idx, mask, memo);
    let (val, set) = &arr[idx];
    for &row in set {
        if (mask >> row) & 1 == 0 {
            res = res.max(val + dfs(arr, 1 + idx, mask | (1 << row), memo))
        }
    }
    memo[idx][mask] = res;
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
        assert_eq!(max_score(&[&[1, 2, 3], &[4, 3, 2], &[1, 1, 1]]), 8);
        assert_eq!(max_score(&[&[8, 7, 6], &[8, 3, 2]]), 15);
    }

    #[test]
    fn test() {}
}
