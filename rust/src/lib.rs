mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn can_partition_grid(mut grid: Vec<Vec<i32>>) -> bool {
    let sum = grid.iter().flatten().fold(0, |acc, &v| acc + i64::from(v));
    if f(&grid, sum) {
        return true;
    }
    let mut tr = transpose(&grid);
    if f(&tr, sum) {
        return true;
    }
    grid.reverse();
    tr.reverse();
    f(&grid, sum) || f(&tr, sum)
}

fn f(grid: &[Vec<i32>], sum: i64) -> bool {
    let mut seen = HashSet::new();
    let mut up = 0;
    for (r, row) in grid.iter().enumerate() {
        for &v in row {
            up += i64::from(v);
            seen.insert(i64::from(v));
        }
        let diff = up - (sum - up);
        // 1 ..
        // 2 ..
        // 1 ..
        if diff == 0 || diff == i64::from(row[0]) {
            return true;
        }
        // 2 1 ..
        // 1 ..
        if i64::from(grid[0][0]) == diff {
            return true;
        }
        // 1 ..
        // 1 2 ..
        // 1 ..
        if grid[0].len() > 1 && r > 0 && seen.contains(&diff) {
            return true;
        }
    }
    false
}

fn transpose(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(&grid);
    let mut res = vec![vec![0; rows]; cols];
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            res[c][r] = v
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
