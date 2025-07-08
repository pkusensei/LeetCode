mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    events.sort_unstable_by_key(|e| e[0]);
    let n = events.len();
    let k = k as usize;
    dfs(&events, 0, k, &mut vec![vec![-1; 1 + k]; n])
}

fn dfs(arr: &[Vec<i32>], idx: usize, k: usize, memo: &mut [Vec<i32>]) -> i32 {
    if idx >= arr.len() || k == 0 {
        return 0;
    }
    if memo[idx][k] > -1 {
        return memo[idx][k];
    }
    let skip = dfs(arr, 1 + idx, k, memo);
    let i = arr.partition_point(|v| v[0] <= arr[idx][1]);
    let take = arr[idx][2] + dfs(arr, i, k - 1, memo);
    memo[idx][k] = skip.max(take);
    memo[idx][k]
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
    fn basics() {}

    #[test]
    fn test() {}
}
