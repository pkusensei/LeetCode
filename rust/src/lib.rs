mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn stone_game_viii(stones: &[i32]) -> i32 {
    let n = stones.len();
    let prefix = stones.iter().fold(Vec::with_capacity(n), |mut acc, &num| {
        acc.push(num + acc.last().unwrap_or(&0));
        acc
    });
    let mut dp = prefix[n - 1];
    for idx in (1..n - 1).rev() {
        dp = dp.max(prefix[idx] - dp);
    }
    dp
    // dfs(&prefix, 0, &mut vec![None; n])
}

fn dfs(prefix: &[i32], idx: usize, memo: &mut [Option<i32>]) -> i32 {
    let n = prefix.len();
    if idx >= n - 1 {
        return 0;
    }
    if idx == n - 2 {
        return prefix[n - 1];
    }
    if let Some(v) = memo[idx] {
        return v;
    }
    let res = dfs(prefix, 1 + idx, memo).max(prefix[1 + idx] - dfs(prefix, 1 + idx, memo));
    memo[idx] = Some(res);
    res
}

// TLE's
// fn dfs(prefix: &[i32], idx: usize, memo: &mut [Option<i32>]) -> i32 {
//     let n = prefix.len();
//     if idx >= n - 1 {
//         return 0;
//     }
//     if let Some(v) = memo[idx] {
//         return v;
//     }
//     let res = (1 + idx..n)
//         .map(|i| prefix[i] - dfs(prefix, i, memo))
//         .max()
//         .unwrap();
//     memo[idx] = Some(res);
//     res
// }

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
        assert_eq!(stone_game_viii(&[-1, 2, -3, 4, -5]), 5);
        assert_eq!(stone_game_viii(&[7, -6, 5, 10, 5, -2, -6]), 13);
        assert_eq!(stone_game_viii(&[-10, -12]), -22);
    }

    #[test]
    fn test() {}
}
