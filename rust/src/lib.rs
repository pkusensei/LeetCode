mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn transpose(matrix: &[&[i32]]) -> Vec<Vec<i32>> {
    let (rows, cols) = get_dimensions(matrix);
    let mut res = vec![vec![0; rows]; cols];
    for (y, r) in matrix.iter().enumerate() {
        for (x, &v) in r.iter().enumerate() {
            res[x][y] = v
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            transpose(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]),
            [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
        );
        debug_assert_eq!(
            transpose(&[&[1, 2, 3], &[4, 5, 6]]),
            [[1, 4], [2, 5], [3, 6]]
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
