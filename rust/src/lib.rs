mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut dp = grid.clone();
    let mut res = 0;
    for r in (0..rows - 1).rev() {
        for c in 1..cols - 1 {
            if dp[1 + r][c] > 0 && dp[r][c] > 0 {
                dp[r][c] = 1 + dp[1 + r][c - 1].min(dp[1 + r][c + 1]);
                res += dp[r][c] - 1;
            }
        }
    }
    dp = grid;
    for r in 1..rows {
        for c in 1..cols - 1 {
            if dp[r - 1][c] > 0 && dp[r][c] > 0 {
                dp[r][c] = 1 + dp[r - 1][c - 1].min(dp[r - 1][c + 1]);
                res += dp[r][c] - 1;
            }
        }
    }
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
        // assert_eq!(count_pyramids(vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1]]), 2);
        // assert_eq!(count_pyramids(vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1]]), 2);
        assert_eq!(
            count_pyramids(vec![
                vec![1, 1, 1, 1, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 0, 1]
            ]),
            13
        );
    }

    #[test]
    fn test() {}
}
