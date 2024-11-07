mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_falling_path_sum(matrix: &[&[i32]]) -> i32 {
    let n = matrix.len();
    let mut dp = matrix[0].to_vec();
    for row in matrix.iter().skip(1) {
        let mut dp2 = vec![];
        for (x, v) in row.iter().enumerate() {
            let temp = (x.saturating_sub(1)..=(x + 1).min(n - 1))
                .map(|i| dp[i])
                .min()
                .unwrap_or(0)
                + v;
            dp2.push(temp);
        }
        dp = dp2
    }
    dp.into_iter().min().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            min_falling_path_sum(&[&[2, 1, 3], &[6, 5, 4], &[7, 8, 9]]),
            13
        );
        debug_assert_eq!(min_falling_path_sum(&[&[-19, 57], &[-40, -5]]), -59);
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
