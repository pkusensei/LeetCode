mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn cherry_pickup(grid: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    dfs(
        grid,
        0,
        0,
        cols - 1,
        &mut vec![vec![vec![-1; cols]; cols]; rows],
    )
}

fn dfs(grid: &[&[i32]], row: usize, col1: usize, col2: usize, memo: &mut [Vec<Vec<i32>>]) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    if row >= rows {
        return 0;
    }
    if memo[row][col1][col2] > -1 {
        return memo[row][col1][col2];
    }
    let mut res = grid[row][col1] + grid[row][col2];
    let mut curr = 0;
    for c1 in col1.saturating_sub(1)..=(1 + col1).min(cols - 1) {
        for c2 in col2.saturating_sub(1)..=(1 + col2).min(cols - 1) {
            if c1 < c2 {
                curr = curr.max(dfs(grid, 1 + row, c1, c2, memo));
            }
        }
    }
    res += curr;
    memo[row][col1][col2] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            cherry_pickup(&[&[3, 1, 1], &[2, 5, 1], &[1, 5, 5], &[2, 1, 1]]),
            24
        );
        assert_eq!(
            cherry_pickup(&[
                &[1, 0, 0, 0, 0, 0, 1],
                &[2, 0, 0, 0, 0, 3, 0],
                &[2, 0, 9, 0, 0, 0, 0],
                &[0, 3, 0, 5, 4, 0, 0],
                &[1, 0, 2, 3, 0, 0, 6]
            ]),
            28
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
