mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unique_paths_iii(grid: &mut [&mut [i32]]) -> i32 {
    let [x, y] = {
        let mut start = [0, 0];
        for (y, row) in grid.iter().enumerate() {
            for (x, &v) in row.iter().enumerate() {
                if v == 1 {
                    start = [x, y];
                }
            }
        }
        start
    };
    grid[y][x] = -1;
    let mut res = 0;
    backtrack(grid, x, y, &mut res);
    res
}

fn backtrack(grid: &mut [&mut [i32]], x: usize, y: usize, res: &mut i32) {
    if grid[y][x] == 2 {
        if grid.iter().all(|r| r.iter().all(|&v| v == -1 || v == 2)) {
            *res += 1;
        }
        return;
    }
    grid[y][x] = -1;
    for (nx, ny) in neighbors((x, y)) {
        if grid
            .get(ny)
            .is_some_and(|r| r.get(nx).is_some_and(|&v| v > -1))
        {
            backtrack(grid, nx, ny, res);
        }
    }
    grid[y][x] = 0;
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            unique_paths_iii(&mut [&mut [1, 0, 0, 0], &mut [0, 0, 0, 0], &mut [0, 0, 2, -1]]),
            2
        );
        debug_assert_eq!(
            unique_paths_iii(&mut [&mut [1, 0, 0, 0], &mut [0, 0, 0, 0], &mut [0, 0, 0, 2]]),
            4
        );
        debug_assert_eq!(unique_paths_iii(&mut [&mut [0, 1], &mut [2, 0]]), 0);
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
