mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut dp = vec![vec![[false; 1024]; cols]; rows];
    dp[0][0][grid[0][0] as usize] = true;
    for r in 0..rows {
        for c in 0..cols {
            let num = grid[r][c] as usize;
            for i in 0..1024 {
                if r > 0 && dp[r - 1][c][i] {
                    dp[r][c][i ^ num] = true;
                }
                if c > 0 && dp[r][c - 1][i] {
                    dp[r][c][i ^ num] = true;
                }
            }
        }
    }
    dp[rows - 1][cols - 1]
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v { Some(i as i32) } else { None })
        .min()
        .unwrap()
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
    fn basics() {}

    #[test]
    fn test() {}
}
