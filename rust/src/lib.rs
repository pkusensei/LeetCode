mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let (rows, cols) = get_dimensions(&grid);
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] > 0 {
                res = res.max(dfs(&mut grid, row, col))
            }
        }
    }
    res
}

fn dfs(grid: &mut [Vec<i32>], row: usize, col: usize) -> i32 {
    if grid[row][col] == 0 {
        return 0;
    }
    let curr = grid[row][col];
    grid[row][col] = 0;
    let mut res = 0;
    for (nr, nc) in neighbors((row, col)) {
        if grid.get(nr).is_some_and(|r| r.get(nc).is_some()) {
            res = res.max(curr + dfs(grid, nr, nc));
        }
    }
    grid[row][col] = curr;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
            24
        );
        assert_eq!(
            get_maximum_gold(vec![
                vec![1, 0, 7],
                vec![2, 0, 6],
                vec![3, 4, 5],
                vec![0, 3, 0],
                vec![9, 0, 20]
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
