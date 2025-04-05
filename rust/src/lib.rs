mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn paint_walls(cost: &[i32], time: &[i32]) -> i32 {
    let n = cost.len();
    dfs(cost, time, 0, n as i32, &mut HashMap::new())
}

fn dfs(
    cost: &[i32],
    time: &[i32],
    idx: usize,
    remain: i32,
    memo: &mut HashMap<(usize, i32), i32>,
) -> i32 {
    let n = cost.len();
    if remain <= 0 {
        return 0;
    }
    if idx >= n {
        return i32::MAX / 2;
    }
    if let Some(&v) = memo.get(&(idx, remain)) {
        return v;
    }
    let take = cost[idx] + dfs(cost, time, 1 + idx, remain - 1 - time[idx], memo);
    let skip = dfs(cost, time, 1 + idx, remain, memo);
    let res = take.min(skip);
    memo.insert((idx, remain), res);
    res
}

pub fn tabulation(cost: &[i32], time: &[i32]) -> i32 {
    let n = cost.len();
    let mut dp = vec![i32::MAX / 2; 1 + n];
    dp[0] = 0;
    for idx in (0..n).rev() {
        let mut curr = vec![0; 1 + n];
        for remain in 1..=n {
            let take = cost[idx] + 0.max(remain as i32 - 1 - time[idx]);
            let skip = dp[remain];
            curr[remain] = skip.min(take);
        }
        dp = curr
    }
    dp[n]
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
        assert_eq!(paint_walls(&[1, 2, 3, 2], &[1, 2, 3, 2]), 3);
        assert_eq!(paint_walls(&[2, 3, 4, 2], &[1, 1, 1, 1]), 4);

        assert_eq!(tabulation(&[1, 2, 3, 2], &[1, 2, 3, 2]), 3);
        assert_eq!(tabulation(&[2, 3, 4, 2], &[1, 1, 1, 1]), 4);
    }

    #[test]
    fn test() {}
}
