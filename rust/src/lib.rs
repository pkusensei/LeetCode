mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let k = k as usize;
    let mut dp = vec![vec![vec![-1; 1 + k]; cols]; rows];
    dp[0][0][0] = 0;
    for r in 0..rows {
        for c in 0..cols {
            let cost = usize::from(grid[r][c] > 0);
            for kk in 0..=k {
                if kk + cost <= k {
                    if r > 0 && dp[r - 1][c][kk] >= 0 {
                        dp[r][c][kk + cost] =
                            dp[r][c][kk + cost].max(grid[r][c] + dp[r - 1][c][kk]);
                    }
                    if c > 0 && dp[r][c - 1][kk] >= 0 {
                        dp[r][c][kk + cost] =
                            dp[r][c][kk + cost].max(grid[r][c] + dp[r][c - 1][kk]);
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
