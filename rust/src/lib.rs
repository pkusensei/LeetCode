mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&coins);
    let mut dp = vec![vec![[i32::MIN >> 1; 3]; cols]; rows];
    dp[0][0][0] = coins[0][0];
    dp[0][0][1..].fill(coins[0][0].max(0));
    for r in 0..rows {
        for c in 0..cols {
            let v = coins[r][c];
            for i in 0..3 {
                if r > 0 {
                    dp[r][c][i] = dp[r][c][i].max(v + dp[r - 1][c][i]);
                    if i > 0 {
                        dp[r][c][i] = dp[r][c][i].max(dp[r - 1][c][i - 1]);
                    }
                }
                if c > 0 {
                    dp[r][c][i] = dp[r][c][i].max(v + dp[r][c - 1][i]);
                    if i > 0 {
                        dp[r][c][i] = dp[r][c][i].max(dp[r][c - 1][i - 1]);
                    }
                }
            }
        }
    }
    *dp[rows - 1][cols - 1].iter().max().unwrap()
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
