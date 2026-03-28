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
    use std::collections::HashSet;
    let [rows, cols] = get_dimensions(&grid);
    let mut dp = vec![vec![HashSet::new(); cols]; rows];
    dp[0][0] = HashSet::from([grid[0][0]]);
    for r in 0..rows {
        for c in 0..cols {
            let mut temp = vec![];
            if r > 0 {
                for prev in &dp[r - 1][c] {
                    temp.push(prev ^ grid[r][c]);
                }
            }
            if c > 0 {
                for &prev in &dp[r][c - 1] {
                    temp.push(prev ^ grid[r][c]);
                }
            }
            dp[r][c].extend(temp);
        }
    }
    *dp[rows - 1][cols - 1].iter().min().unwrap()
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
