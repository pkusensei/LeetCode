mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
    let n = stones.len();
    let prefix = stones.iter().fold(Vec::with_capacity(n), |mut acc, v| {
        acc.push(v + acc.last().unwrap_or(&0));
        acc
    });
    let mut dp = prefix[n - 1];
    for idx in (0..n - 2).rev() {
        let curr = dp.max(prefix[1 + idx] - dp);
        dp = curr;
    }
    dp
    // let mut dp = vec![i32::MIN >> 1; n];
    // dp[n - 2] = prefix[n - 1];
    // for idx in (0..n - 2).rev() {
    //     dp[idx] = dp[1 + idx].max(prefix[1 + idx] - dp[1 + idx]);
    // }
    // dp[0]
    // dfs(&prefix, 0)
}

fn dfs(prefix: &[i32], idx: usize) -> i32 {
    let n = prefix.len();
    if idx >= n - 1 {
        return 0;
    }
    if idx >= n - 2 {
        return prefix[n - 1];
    }
    dfs(prefix, 1 + idx).max(prefix[1 + idx] - dfs(prefix, 1 + idx))
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
