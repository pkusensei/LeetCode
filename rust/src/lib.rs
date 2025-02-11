mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_skips(dist: &[i32], speed: i32, hours_before: i32) -> i32 {
    let n = dist.len();
    let mut dp = vec![vec![f64::INFINITY; 1 + n]; 1 + n];
    dp[0][0] = 0.0;
    for idx in 1..=n {
        for k in 0..=idx {
            let time = f64::from(dist[idx - 1]) / f64::from(speed);
            if k < idx {
                dp[idx][k] = (dp[idx - 1][k] + time - 1e-9).ceil(); // take rest
            }
            if k > 0 {
                dp[idx][k] = dp[idx][k].min(dp[idx - 1][k - 1] + time); // skip rest
            }
        }
    }
    for k in 0..=n {
        if dp[n][k] <= f64::from(hours_before) {
            return k as _;
        }
    }
    // let speed = i64::from(speed);
    // let mut memo = vec![vec![-1; 1 + n]; n];
    // for k in 0..=n {
    //     if dfs(dist, speed, n as i32 - 1, k, &mut memo) <= i64::from(hours_before) * speed {
    //         return k as _;
    //     }
    // }
    -1
}

fn dfs(dist: &[i32], speed: i64, idx: i32, k: usize, memo: &mut [Vec<i64>]) -> i64 {
    if idx < 0 {
        return 0;
    }
    if memo[idx as usize][k] > -1 {
        return memo[idx as usize][k];
    }
    let mut res = (dfs(dist, speed, idx - 1, k, memo) + i64::from(dist[idx as usize]) + speed - 1)
        / speed
        * speed;
    if k > 0 {
        res = res.min(dfs(dist, speed, idx - 1, k - 1, memo) + i64::from(dist[idx as usize]));
    }
    memo[idx as usize][k] = res;
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
        assert_eq!(min_skips(&[1, 3, 2], 4, 2), 1);
        assert_eq!(min_skips(&[7, 3, 5, 5], 2, 10), 2);
        assert_eq!(min_skips(&[7, 3, 5, 5], 1, 10), -1);
    }

    #[test]
    fn test() {}
}
