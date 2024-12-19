mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_squares(matrix: &[&[i32]]) -> i32 {
    let (rows, cols) = get_dimensions(matrix);
    let mut dp = vec![vec![0; 1 + cols]; 1 + rows];
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 1 {
                dp[1 + row][1 + col] = 1 + dp[row][col].min(dp[row + 1][col]).min(dp[row][col + 1])
            }
        }
    }
    dp.into_iter().flatten().sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // assert_eq!(
        //     count_squares(&[&[0, 1, 1, 1], &[1, 1, 1, 1], &[0, 1, 1, 1]]),
        //     15
        // );
        assert_eq!(count_squares(&[&[1, 0, 1], &[1, 1, 0], &[1, 1, 0]]), 7);
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
