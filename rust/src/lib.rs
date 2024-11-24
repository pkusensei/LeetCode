mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_matrix_sum(matrix: &[&[i32]]) -> i64 {
    let count = matrix
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&v| v < 0)
        .count();
    let it = matrix
        .iter()
        .flat_map(|v| v.iter().map(|&v| i64::from(v).abs()));
    if count & 1 == 1 {
        let min = it.clone().min().unwrap_or(0);
        it.sum::<i64>() - 2 * min
    } else {
        it.sum()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_matrix_sum(&[&[1, -1], &[-1, 1]]), 4);
        debug_assert_eq!(max_matrix_sum(&[&[1, 2, 3], &[-1, -2, -3], &[1, 2, 3]]), 16);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            max_matrix_sum(&[&[9, -3, -4], &[-4, -1, -3], &[-6, -3, -3]]),
            36
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
