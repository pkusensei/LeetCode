mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut dp_max = vec![vec![i64::MIN; cols]; rows];
    let mut dp_min = vec![vec![i64::MAX; cols]; rows];
    dp_max[0][0] = grid[0][0].into();
    dp_min[0][0] = grid[0][0].into();
    for r in 0..rows {
        for c in 0..cols {
            let curr = i64::from(grid[r][c]);
            if r > 0 {
                dp_max[r][c] = dp_max[r][c]
                    .max(curr * dp_max[r - 1][c])
                    .max(curr * dp_min[r - 1][c]);
                dp_min[r][c] = dp_min[r][c]
                    .min(curr * dp_max[r - 1][c])
                    .min(curr * dp_min[r - 1][c]);
            }
            if c > 0 {
                dp_max[r][c] = dp_max[r][c]
                    .max(curr * dp_max[r][c - 1])
                    .max(curr * dp_min[r][c - 1]);
                dp_min[r][c] = dp_min[r][c]
                    .min(curr * dp_max[r][c - 1])
                    .min(curr * dp_min[r][c - 1]);
            }
        }
    }
    (dp_max[rows - 1][cols - 1] % 1_000_000_007).max(-1) as i32
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
