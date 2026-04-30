mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid.len();
    let mut prefix = vec![vec![0; n]; 1 + n];
    for c in 0..n {
        for r in 1..=n {
            prefix[r][c] = prefix[r - 1][c] + i64::from(grid[r - 1][c]);
        }
    }
    let mut prev = vec![[0; 2]; 1 + n];
    for col in (0..n).rev() {
        let mut curr = vec![[0; 2]; 1 + n];
        for last_h in 0..=n {
            for h in 0..=n {
                for taken in 0..2 {
                    if h <= last_h {
                        let val = prefix[last_h][col] - prefix[h][col];
                        curr[last_h][taken] =
                            curr[last_h][taken].max(val + prev[h][1]).max(prev[h][0]);
                    } else if taken == 1 {
                        curr[last_h][taken] = curr[last_h][taken].max(prev[h][0]);
                    } else {
                        let val = if col > 0 {
                            prefix[h][col - 1] - prefix[last_h][col - 1]
                        } else {
                            0
                        };
                        curr[last_h][taken] = curr[last_h][taken].max(val + prev[h][0]);
                    }
                }
            }
        }
        prev = curr;
    }
    prev[0][0]
    // let mut dp = vec![vec![[0; 2]; 1 + n]; 1 + n];
    // for col in (0..n).rev() {
    //     for last_h in 0..=n {
    //         for h in 0..=n {
    //             for taken in 0..2 {
    //                 if h <= last_h {
    //                     let curr = prefix[last_h][col] - prefix[h][col];
    //                     dp[col][last_h][taken] = dp[col][last_h][taken]
    //                         .max(curr + dp[1 + col][h][1])
    //                         .max(dp[1 + col][h][0]);
    //                 } else {
    //                     if taken == 1 {
    //                         dp[col][last_h][taken] = dp[col][last_h][taken].max(dp[1 + col][h][0]);
    //                     } else {
    //                         let curr = if col > 0 {
    //                             prefix[h][col - 1] - prefix[last_h][col - 1]
    //                         } else {
    //                             0
    //                         };
    //                         dp[col][last_h][taken] =
    //                             dp[col][last_h][taken].max(curr + dp[1 + col][h][0]);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // dp[0][0][0]
    // let mut memo = vec![vec![[-1; 2]; 1 + n]; n];
    // dfs(&prefix, 0, 0, false, &mut memo)
}

fn dfs(
    prefix: &[Vec<i64>],
    idx: usize,
    last_h: usize,
    taken: bool,
    memo: &mut [Vec<[i64; 2]>],
) -> i64 {
    let n = prefix.len() - 1;
    if idx >= n {
        return 0;
    }
    if memo[idx][last_h][usize::from(taken)] > -1 {
        return memo[idx][last_h][usize::from(taken)];
    }
    let mut res = 0;
    for h in 0..=n {
        if h <= last_h {
            let curr = prefix[last_h][idx] - prefix[h][idx];
            // Take [h..=last_h] points on current col
            res = res.max(curr + dfs(prefix, 1 + idx, h, true, memo));
            // Skip these points
            res = res.max(dfs(prefix, 1 + idx, h, false, memo));
        } else {
            // h > last_h
            // No point on current col is available
            if !taken {
                // These points are on prev col
                // They are only available if not taken previously
                let mut curr = 0;
                if idx > 0 {
                    curr = prefix[h][idx - 1] - prefix[last_h][idx - 1];
                }
                res = res.max(curr + dfs(prefix, 1 + idx, h, false, memo))
            } else {
                res = res.max(dfs(prefix, 1 + idx, h, false, memo))
            }
        }
    }
    memo[idx][last_h][usize::from(taken)] = res;
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
