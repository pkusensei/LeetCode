mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn lucky_numbers(matrix: &[&[i32]]) -> Vec<i32> {
    let mins: HashSet<_> = matrix
        .iter()
        .map(|row| row.iter().copied().min().unwrap())
        .collect();
    let cols = matrix[0].len();
    let maxs: HashSet<_> = (0..cols)
        .map(|col| matrix.iter().map(|r| r[col]).max().unwrap())
        .collect();
    mins.intersection(&maxs).copied().collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            lucky_numbers(&[&[3, 7, 8], &[9, 11, 13], &[15, 16, 17]]),
            [15]
        );
        assert_eq!(
            lucky_numbers(&[&[1, 10, 4, 2], &[9, 3, 8, 7], &[15, 16, 17, 12]]),
            [12]
        );
        assert_eq!(lucky_numbers(&[&[7, 8], &[1, 2]]), [7]);
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
