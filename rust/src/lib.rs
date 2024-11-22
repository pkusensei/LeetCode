mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_equal_rows_after_flips(matrix: &[&[i32]]) -> i32 {
    let mut map = std::collections::HashMap::new();
    for v in matrix.iter() {
        let (mut zeros, mut ones) = (vec![], vec![]);
        for (i, &num) in v.iter().enumerate() {
            if num == 0 {
                zeros.push(i);
            } else {
                ones.push(i);
            }
        }
        *map.entry(zeros).or_insert(0) += 1;
        *map.entry(ones).or_insert(0) += 1;
    }
    map.into_values().max().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_equal_rows_after_flips(&[&[0, 1], &[1, 1]]), 1);
        debug_assert_eq!(max_equal_rows_after_flips(&[&[0, 1], &[1, 0]]), 2);
        debug_assert_eq!(
            max_equal_rows_after_flips(&[&[0, 0, 0], &[0, 0, 1], &[1, 1, 0]]),
            2
        );
        // debug_assert_eq!(color_border(&[&[1, 1], &[1, 2]], 0, 0, 3), [[3, 3], [3, 2]]);
        // debug_assert_eq!(
        //     color_border(&[&[1, 2, 2], &[2, 3, 2]], 0, 1, 3),
        //     [[1, 3, 3], [2, 3, 3]]
        // );
        // debug_assert_eq!(
        //     color_border(&[&[1, 1, 1], &[1, 1, 1], &[1, 1, 1]], 1, 1, 2),
        //     [[2, 2, 2], [2, 1, 2], [2, 2, 2]]
        // );
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
