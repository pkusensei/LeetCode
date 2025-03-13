mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn selling_wood(m: i32, n: i32, prices: &[[i32; 3]]) -> i64 {
    let map = prices.iter().fold(HashMap::new(), |mut acc, p| {
        let [h, w, p] = p[..] else { unreachable!() };
        acc.insert([h, w], i64::from(p));
        acc
    });
    dfs(
        m,
        n,
        &map,
        &mut vec![vec![-1; 1 + n as usize]; 1 + m as usize],
    )
}

fn dfs(m: i32, n: i32, map: &HashMap<[i32; 2], i64>, memo: &mut [Vec<i64>]) -> i64 {
    if m == 0 || n == 0 {
        return 0;
    }
    if memo[m as usize][n as usize] > -1 {
        return memo[m as usize][n as usize];
    }
    let mut res = *map.get(&[m, n]).unwrap_or(&0);
    for h in 1..=m / 2 {
        res = res.max(dfs(h, n, map, memo) + dfs(m - h, n, map, memo));
    }
    for w in 1..=n / 2 {
        res = res.max(dfs(m, w, map, memo) + dfs(m, n - w, map, memo))
    }
    memo[m as usize][n as usize] = res;
    res
}

pub fn tabulation(m: i32, n: i32, prices: &[[i32; 3]]) -> i64 {
    let [m, n] = [m, n].map(|v| v as usize);
    let mut dp = vec![vec![0; 1 + n]; 1 + m];
    for p in prices.iter() {
        dp[p[0] as usize][p[1] as usize] = i64::from(p[2]);
    }
    for h in 1..=m {
        for w in 1..=n {
            for k in 1..=h / 2 {
                dp[h][w] = dp[h][w].max(dp[k][w] + dp[h - k][w]);
            }
            for k in 1..=w / 2 {
                dp[h][w] = dp[h][w].max(dp[h][k] + dp[h][w - k]);
            }
        }
    }
    dp[m][n]
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
        assert_eq!(selling_wood(3, 5, &[[1, 4, 2], [2, 2, 7], [2, 1, 3]]), 19);
        assert_eq!(selling_wood(4, 6, &[[3, 2, 10], [1, 4, 2], [4, 1, 3]]), 32);

        assert_eq!(tabulation(3, 5, &[[1, 4, 2], [2, 2, 7], [2, 1, 3]]), 19);
        assert_eq!(tabulation(4, 6, &[[3, 2, 10], [1, 4, 2], [4, 1, 3]]), 32);
    }

    #[test]
    fn test() {
        assert_eq!(
            selling_wood(
                20,
                13,
                &[
                    [5, 10, 15],
                    [7, 12, 24],
                    [15, 12, 1],
                    [17, 3, 10],
                    [20, 9, 22],
                    [5, 13, 15],
                    [16, 7, 28],
                    [12, 10, 29],
                    [2, 9, 1],
                    [14, 6, 15],
                    [20, 8, 20]
                ]
            ),
            70
        );

        assert_eq!(
            tabulation(
                20,
                13,
                &[
                    [5, 10, 15],
                    [7, 12, 24],
                    [15, 12, 1],
                    [17, 3, 10],
                    [20, 9, 22],
                    [5, 13, 15],
                    [16, 7, 28],
                    [12, 10, 29],
                    [2, 9, 1],
                    [14, 6, 15],
                    [20, 8, 20]
                ]
            ),
            70
        );
    }
}
