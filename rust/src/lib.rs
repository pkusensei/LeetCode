mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn k_weakest_rows(mat: &[&[i32]], k: i32) -> Vec<i32> {
    let mut rows: Vec<_> = mat
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            let count = row.partition_point(|&v| v == 1);
            [count, idx]
        })
        .collect();
    rows.sort_unstable();
    rows.into_iter()
        .map(|v| v[1] as i32)
        .take(k as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            k_weakest_rows(
                &[
                    &[1, 1, 0, 0, 0],
                    &[1, 1, 1, 1, 0],
                    &[1, 0, 0, 0, 0],
                    &[1, 1, 0, 0, 0],
                    &[1, 1, 1, 1, 1]
                ],
                3
            ),
            [2, 0, 3]
        );
        assert_eq!(
            k_weakest_rows(
                &[&[1, 0, 0, 0], &[1, 1, 1, 1], &[1, 0, 0, 0], &[1, 0, 0, 0]],
                2
            ),
            [0, 2]
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
