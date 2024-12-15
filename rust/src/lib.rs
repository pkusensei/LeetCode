mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn closed_island(grid: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(grid);
    let mut seen = vec![vec![false; cols]; rows];
    let mut res = 0;
    for (row, r) in grid.iter().enumerate() {
        for (col, &v) in r.iter().enumerate() {
            if v == 0 && !seen[row][col] {
                res += bfs(rows, cols, row, col, grid, &mut seen);
            }
        }
    }
    res
}

fn bfs(
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
    grid: &[&[i32]],
    seen: &mut [Vec<bool>],
) -> i32 {
    let mut queue = VecDeque::from([[row, col]]);
    let mut inland = true;
    seen[row][col] = true;
    while let Some([row, col]) = queue.pop_front() {
        if !(1..rows - 1).contains(&row) || !(1..cols - 1).contains(&col) {
            inland = false
        }
        for (nr, nc) in neighbors((row, col)) {
            if grid
                .get(nr)
                .is_some_and(|r| r.get(nc).is_some_and(|&v| v == 0))
                && !seen[nr][nc]
            {
                seen[nr][nc] = true;
                queue.push_back([nr, nc]);
            }
        }
    }
    i32::from(inland)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            closed_island(&[
                &[1, 1, 1, 1, 1, 1, 1, 0],
                &[1, 0, 0, 0, 0, 1, 1, 0],
                &[1, 0, 1, 0, 1, 1, 1, 0],
                &[1, 0, 0, 0, 0, 1, 0, 1],
                &[1, 1, 1, 1, 1, 1, 1, 0]
            ]),
            2
        );
        assert_eq!(
            closed_island(&[&[0, 0, 1, 0, 0], &[0, 1, 0, 1, 0], &[0, 1, 1, 1, 0]]),
            1
        );
        assert_eq!(
            closed_island(&[
                &[1, 1, 1, 1, 1, 1, 1],
                &[1, 0, 0, 0, 0, 0, 1],
                &[1, 0, 1, 1, 1, 0, 1],
                &[1, 0, 1, 0, 1, 0, 1],
                &[1, 0, 1, 1, 1, 0, 1],
                &[1, 0, 0, 0, 0, 0, 1],
                &[1, 1, 1, 1, 1, 1, 1]
            ]),
            2
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
            "left = {a:?}, right = {b:?}",
        );
    }
}
