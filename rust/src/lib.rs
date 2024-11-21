mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_unguarded(m: i32, n: i32, guards: &[[i32; 2]], walls: &[[i32; 2]]) -> i32 {
    let [rows, cols] = [m, n].map(|v| v as usize);
    let mut grid = vec![vec![0; cols]; rows];
    for v in walls.iter().chain(guards.iter()) {
        let [r, c] = [v[0], v[1]].map(|v| v as usize);
        grid[r][c] = 1;
    }
    for g in guards.iter() {
        let [r, c] = [g[0], g[1]].map(|v| v as usize);
        for i in 1 + r..rows {
            if grid[i][c] == 1 {
                break;
            }
            grid[i][c] = 2;
        }
        for i in (0..r).rev() {
            if grid[i][c] == 1 {
                break;
            }
            grid[i][c] = 2;
        }
        for i in 1 + c..cols {
            if grid[r][i] == 1 {
                break;
            }
            grid[r][i] = 2;
        }
        for i in (0..c).rev() {
            if grid[r][i] == 1 {
                break;
            }
            grid[r][i] = 2;
        }
    }
    grid.into_iter().flatten().filter(|&v| v == 0).count() as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            count_unguarded(4, 6, &[[0, 0], [1, 1], [2, 3]], &[[0, 1], [2, 2], [1, 4]]),
            7
        );
        debug_assert_eq!(
            count_unguarded(3, 3, &[[1, 1]], &[[0, 1], [1, 0], [2, 1], [1, 2]]),
            4
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            count_unguarded(10, 8, &[[6, 4], [4, 5], [0, 3], [8, 2], [6, 3]], &[[7, 2]]),
            28
        );
    }

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
