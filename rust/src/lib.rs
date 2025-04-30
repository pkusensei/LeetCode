mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
    let cols = grid[0].len();
    let row_sums: Vec<_> = grid
        .iter()
        .map(|r| r.iter().map(|&v| i64::from(v)).sum::<i64>())
        .collect();
    let col_sums: Vec<_> = (0..cols)
        .map(|c| grid.iter().map(|r| i64::from(r[c])).sum::<i64>())
        .collect();
    let mut res = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == 1 {
                res += (row_sums[r] - 1) * (col_sums[c] - 1);
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
    fn basics() {}

    #[test]
    fn test() {}
}
