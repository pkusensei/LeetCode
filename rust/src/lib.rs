mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(&grid);
    let pre_rows: Vec<_> = grid.iter().map(|r| r.iter().sum::<i32>()).collect();
    let pre_cols: Vec<_> = (0..cols)
        .map(|c| grid.iter().map(|row| row[c]).sum::<i32>())
        .collect();
    let mut res = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            res[r][c] = pre_rows[r] + pre_cols[c]
                - (cols as i32 - pre_rows[r])
                - (rows as i32 - pre_cols[c]);
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
    fn basics() {}

    #[test]
    fn test() {}
}
