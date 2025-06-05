mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(n: i32, cost: &[[i32; 3]]) -> i64 {
    dfs(cost, 0, 3, 3, &mut vec![[[-1; 4]; 4]; n as usize / 2])
}

fn dfs(
    cost: &[[i32; 3]],
    idx: usize,
    left_c: usize,
    right_c: usize,
    memo: &mut [[[i64; 4]; 4]],
) -> i64 {
    use itertools::Itertools;
    let n = cost.len() / 2;
    if idx >= n {
        return 0;
    }
    if memo[idx][left_c][right_c] > -1 {
        return memo[idx][left_c][right_c];
    }
    let left = n - idx - 1;
    let right = n + idx;
    let mut res = i64::MAX;
    for [a, b] in (0..3).array_combinations() {
        if a != left_c && b != right_c {
            res =
                res.min(i64::from(cost[left][a] + cost[right][b]) + dfs(cost, 1 + idx, a, b, memo));
        }
        if a != right_c && b != left_c {
            res =
                res.min(i64::from(cost[left][b] + cost[right][a]) + dfs(cost, 1 + idx, b, a, memo));
        }
    }
    memo[idx][left_c][right_c] = res;
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
        assert_eq!(
            min_cost(4, &[[3, 5, 7], [6, 2, 9], [4, 8, 1], [7, 3, 5]]),
            9
        );
        assert_eq!(
            min_cost(
                6,
                &[
                    [2, 4, 6],
                    [5, 3, 8],
                    [7, 1, 9],
                    [4, 6, 2],
                    [3, 5, 7],
                    [8, 2, 4]
                ]
            ),
            18
        );
    }

    #[test]
    fn test() {}
}
