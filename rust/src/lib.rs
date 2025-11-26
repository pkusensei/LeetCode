mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const M: i32 = 1_000_000_007;
    let [rows, cols] = get_dimensions(&grid);
    let k = k as usize;
    let mut dp = vec![vec![vec![0; k]; cols]; rows];
    dp[0][0][grid[0][0] as usize % k] = 1;
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            for rem in 0..k {
                if r > 0 {
                    dp[r][c][(rem + v as usize) % k] =
                        (dp[r][c][(rem + v as usize) % k] + dp[r - 1][c][rem]) % M;
                }
                if c > 0 {
                    dp[r][c][(rem + v as usize) % k] =
                        (dp[r][c][(rem + v as usize) % k] + dp[r][c - 1][rem]) % M;
                }
            }
        }
    }
    dp[rows - 1][cols - 1][0] as i32
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
