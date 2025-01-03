mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn diagonal_sort(mat: &[&[i32]]) -> Vec<Vec<i32>> {
    let (rows, cols) = get_dimensions(mat);
    let mut map = std::collections::HashMap::<i32, (Vec<_>, Vec<_>)>::new();
    let mut res = vec![vec![0; cols]; rows];
    for (y, row) in mat.iter().enumerate() {
        for (x, &num) in row.iter().enumerate() {
            let v = map.entry(y as i32 - x as i32).or_default();
            v.0.push((y, x));
            v.1.push(num);
        }
    }
    for v in map.values_mut() {
        v.1.sort_unstable();
    }
    for ((y, x), num) in map
        .into_values()
        .flat_map(|(v1, v2)| v1.into_iter().zip(v2))
    {
        res[y][x] = num
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            diagonal_sort(&[&[3, 3, 1, 1], &[2, 2, 1, 2], &[1, 1, 1, 2]]),
            [[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]]
        );
        assert_eq!(
            diagonal_sort(&[
                &[11, 25, 66, 1, 69, 7],
                &[23, 55, 17, 45, 15, 52],
                &[75, 31, 36, 44, 58, 8],
                &[22, 27, 33, 25, 68, 4],
                &[84, 28, 14, 11, 5, 50]
            ]),
            [
                [5, 17, 4, 1, 52, 7],
                [11, 11, 25, 45, 8, 69],
                [14, 23, 25, 44, 58, 15],
                [22, 27, 31, 36, 50, 66],
                [84, 28, 75, 33, 55, 68]
            ]
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
