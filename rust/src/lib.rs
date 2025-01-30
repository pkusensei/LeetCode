mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_vii(stones: &[i32]) -> i32 {
    let n = stones.len();
    let prefix = stones.iter().fold(Vec::with_capacity(n), |mut acc, &num| {
        acc.push(num + acc.last().unwrap_or(&0));
        acc
    });
    let mut dp = vec![vec![0; n]; n];
    for left in (0..n).rev() {
        for right in 1 + left..n {
            let remove_left = prefix[right] - prefix[left];
            let remove_right = prefix[right - 1] - if left > 0 { prefix[left - 1] } else { 0 };
            dp[left][right] =
                (remove_left - dp[1 + left][right]).max(remove_right - dp[left][right - 1]);
        }
    }
    dp[0][n - 1]
    // dfs(&prefix, 0, n - 1, &mut vec![vec![-1; n]; n])
}

fn dfs(prefix: &[i32], left: usize, right: usize, memo: &mut [Vec<i32>]) -> i32 {
    if left == right {
        return 0;
    }
    if memo[left][right] > -1 {
        return memo[left][right];
    }
    let remove_left = prefix[right] - prefix[left];
    let remove_right = prefix[right - 1] - if left > 0 { prefix[left - 1] } else { 0 };
    // max (curr_score - opponent_score)
    let res = (remove_left - dfs(prefix, 1 + left, right, memo))
        .max(remove_right - dfs(prefix, left, right - 1, memo));
    memo[left][right] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(stone_game_vii(&[5, 3, 1, 4, 2]), 6);
        assert_eq!(stone_game_vii(&[7, 90, 5, 1, 100, 10, 10, 2]), 122);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
