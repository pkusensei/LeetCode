mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_dp(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![i32::MAX; 1 + n];
    dp[0] = 0;
    // for idx in 1..=n {
    //     let mut d = 1;
    //     while d * (1 + d) / 2 <= idx as i32 {
    //         let delta = idx as i32 - d * (1 + d) / 2;
    //         if delta > 0 {
    //             dp[idx] = dp[idx].min(1 + d as i32 + dp[delta as usize])
    //         } else if delta == 0 {
    //             dp[idx] = dp[idx].min(d);
    //         }
    //         d += 1;
    //     }
    // }
    let mut idx = 1;
    let mut streak = 1;
    while idx <= n {
        dp[idx] = streak;
        for right in (1 + idx)..=n.min(2 * idx) {
            dp[right] = dp[right].min(dp[right - idx] + 1 + streak);
        }
        streak += 1;
        idx += streak as usize;
    }
    dp[n]
}

pub fn min_days(n: i32) -> i32 {
    let mut memo = vec![-1; 1 + n as usize];
    dfs(n, &mut memo)
}

fn dfs(n: i32, memo: &mut [i32]) -> i32 {
    if n == 0 {
        return 0;
    }
    if memo[n as usize] > -1 {
        return memo[n as usize];
    }
    let mut res = i32::MAX;
    let mut i = 1;
    while i * (1 + i) / 2 <= n {
        let delta = n - i * (1 + i) / 2;
        if delta > 0 {
            res = res.min(i + 1 + dfs(delta, memo))
        } else if delta == 0 {
            res = res.min(i);
        }
        i += 1
    }
    memo[n as usize] = res;
    res
}

// 447*448/2 = 100_128
const SUMS: [i32; 448] = f();
const fn f() -> [i32; 448] {
    let mut res = [0; 448];
    let mut i = 1;
    while i <= 447 {
        res[i] = i as i32 * (1 + i as i32) / 2;
        i += 1;
    }
    res
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
    fn basics() {
        assert_eq!(with_dp(3), 2);
        assert_eq!(with_dp(9), 6);

        assert_eq!(min_days(3), 2);
        assert_eq!(min_days(9), 6);
    }

    #[test]
    fn test() {
        assert_eq!(with_dp(15), 5);

        assert_eq!(min_days(15), 5);
    }
}
