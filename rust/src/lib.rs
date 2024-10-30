mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
    // row starts with 0 => flip
    for v in grid.iter_mut() {
        if v.first() == Some(&0) {
            for n in v.iter_mut() {
                *n = 1 - *n;
            }
        }
    }
    let (rows, cols) = get_dimensions(&grid);
    // col has more 0s than 1s => flip
    for c in 0..cols {
        let ones: i32 = (0..rows).map(|r| grid[r][c]).sum();
        let zeros = rows as i32 - ones;
        if ones < zeros {
            for v in grid.iter_mut() {
                v[c] = 1 - v[c]
            }
        }
    }
    grid.into_iter()
        .map(|v| v.into_iter().fold(0, |acc, n| (acc << 1) | n))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
            39
        );
        debug_assert_eq!(matrix_score(vec![vec![0]]), 1);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
